use chrono::Local;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::File;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use tauri::Manager;
use tokio_cron_scheduler::{Job, JobScheduler};

use crate::state::{AppState, Database};

fn normalize_cron_expression(expression: &str) -> String {
    let expression = expression.trim();
    if expression.split_whitespace().count() == 5 {
        format!("0 {}", expression)
    } else {
        expression.to_string()
    }
}

fn run_mysqldump_backup(
    database: &Database,
    backup_dir: PathBuf,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // 1. Prepare file destination: backup_2026-10-18_19-37-00.sql.gz
    let timestamp = Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();
    let filename = format!("backup_{}.sql.gz", timestamp);
    let file_path = backup_dir.join(filename);

    let file = File::create(file_path)?;
    let mut encoder = GzEncoder::new(file, Compression::default());

    // 2. Build the mysqldump system command
    // Storing password in environment variable avoids exposing it in process command strings
    let mut child = Command::new("mysqldump")
        .env("MYSQL_PWD", &database.password)
        .args([
            "-h",
            &database.host,
            "-P",
            &database.port.to_string(),
            "-u",
            &database.username,
            "--single-transaction", // Safe for transactional engines like InnoDB
            &database.db_name.clone().unwrap(),
        ])
        .stdout(Stdio::piped()) // Capture SQL plain text stream
        .stderr(Stdio::piped()) // Capture any errors
        .spawn()?;

    // 3. Pipe plain text SQL output directly into the GzEncoder stream
    if let Some(mut stdout) = child.stdout.take() {
        std::io::copy(&mut stdout, &mut encoder)?;
    }

    // Finish compressing the file structure
    encoder.finish()?;

    // 4. Validate process exit status
    let output = child.wait_with_output()?;
    if !output.status.success() {
        let err_msg = String::from_utf8_lossy(&output.stderr);
        return Err(format!("mysqldump failed: {}", err_msg).into());
    }

    Ok(())
}

pub fn setup_auto_backup(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let app_handle = app.app_handle().clone();
    let state = app_handle.state::<AppState>();
    let app_state = state.lock().unwrap();

    let db_config = app_state.database.clone();
    let backup_config = app_state.backup.clone();
    drop(app_state);

    let backup_path = if backup_config.path.trim().is_empty() {
        app_handle.path().app_data_dir()?.join("backups")
    } else {
        PathBuf::from(&backup_config.path)
    };
    std::fs::create_dir_all(&backup_path)?;

    tauri::async_runtime::spawn(async move {
        let scheduler = JobScheduler::new().await.unwrap();
        let cron_expression = normalize_cron_expression(&backup_config.cron);

        log::debug!("cron_expression: {}", cron_expression);

        let backup_job = Job::new_async(cron_expression.as_str(), move |_uuid, _l| {
            let database = db_config.clone();
            let backup_path = backup_path.clone();
            log::debug!("backup_path: {:?}", backup_path);

            Box::pin(async move {
                println!("Starting system mysqldump background routine...");
                // Offload standard blocking IO Command task safely away from tokio executor core loop
                let result = tauri::async_runtime::spawn_blocking(move || {
                    run_mysqldump_backup(&database, backup_path)
                })
                .await;

                match result {
                    Ok(Ok(())) => println!("Database backup compressed successfully!"),
                    Ok(Err(e)) => errors_logger(format!("Runtime Backup Execution Error: {}", e)),
                    Err(e) => errors_logger(format!("Task Join Runtime Error: {}", e)),
                }
            })
        });

        let backup_job = match backup_job {
            Ok(job) => job,
            Err(error) => {
                log::error!(
                    "Invalid automatic backup cron expression '{}': {}",
                    cron_expression,
                    error
                );
                return;
            }
        };

        scheduler.add(backup_job).await.unwrap();
        scheduler.start().await.unwrap();

        std::future::pending::<()>().await;
    });

    Ok(())
}

fn errors_logger(msg: String) {
    eprintln!("{}", msg);
}

#[cfg(test)]
mod tests {
    use super::normalize_cron_expression;

    #[test]
    fn adds_seconds_to_five_field_cron_expression() {
        assert_eq!(normalize_cron_expression("* * * * *"), "0 * * * * *");
    }

    #[test]
    fn preserves_six_field_cron_expression() {
        assert_eq!(normalize_cron_expression("0 0 0 * * *"), "0 0 0 * * *");
    }
}
