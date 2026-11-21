use crate::constants::{BACKUP_FOLDER_NAME, DRIVE_FILE_SCOPE, DRIVE_METADATA_SCOPE, TOKEN_PATH};
use crate::state::{AppState, Database};
use async_trait::async_trait;
use chrono::Local;
use filepath::FilePath;
use flate2::write::GzEncoder;
use flate2::Compression;
use reqwest::Client;
use serde_json::{json, Value};
use std::error::Error;
use std::fs::File;
use std::future::Future;
use std::path::PathBuf;
use std::pin::Pin;
use std::process::{Command, Stdio};
use tauri::{AppHandle, Manager, Runtime, Wry};
use tauri_plugin_opener::OpenerExt;
use tauri_plugin_store::StoreExt;
use tokio_cron_scheduler::{Job, JobScheduler};
use yup_oauth2::authenticator_delegate::InstalledFlowDelegate;
use yup_oauth2::storage::{TokenInfo, TokenStorage};
use yup_oauth2::{ApplicationSecret, InstalledFlowAuthenticator, InstalledFlowReturnMethod};

struct TauriBrowserDelegate {
    app_handle: tauri::AppHandle,
}

impl InstalledFlowDelegate for TauriBrowserDelegate {
    fn present_user_url<'a>(
        &'a self,
        url: &'a str,
        _need_code: bool,
    ) -> Pin<Box<dyn Future<Output = Result<String, String>> + Send + 'a>> {
        let handle = self.app_handle.clone();
        let url_owned = url.to_string();

        Box::pin(async move {
            // Opens the URL in the user's default browser
            handle
                .opener()
                .open_url(&url_owned, None::<&str>)
                .map_err(|e| format!("Failed to open browser: {}", e))?;

            // Return empty string because HTTPRedirect receives the code via local server
            Ok(String::new())
        })
    }
}

pub struct TauriTokenStorage<R: Runtime = Wry> {
    app: tauri::AppHandle<R>,
    path: String,
}

impl<R: Runtime> TauriTokenStorage<R> {
    pub fn new(app: AppHandle<R>, path: String) -> Self {
        Self { app, path }
    }
}

#[async_trait]
impl TokenStorage for TauriTokenStorage<Wry> {
    async fn set(&self, scopes: &[&str], token: TokenInfo) -> anyhow::Result<()> {
        let store = self
            .app
            .store(&self.path)
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;

        let key = scopes.join(" ");
        let value = serde_json::to_value(&token)?;

        store.set(key, value);
        store.save().map_err(|e| anyhow::anyhow!(e.to_string()))?;
        Ok(())
    }

    async fn get(&self, scopes: &[&str]) -> Option<TokenInfo> {
        let store = self.app.store("oauth_tokens.json").ok()?;
        let key = scopes.join(" ");
        let value = store.get(&key)?;

        serde_json::from_value::<TokenInfo>(value).ok()
    }
}

// method  for open google auhenticator
// and caceh refresh token
#[tauri::command]
pub async fn authenticate_google_account_for_backup(
    handle: tauri::AppHandle,
) -> Result<(), String> {
    let (client_id, client_secret) = {
        let state = handle.state::<AppState>();
        let app_state = state.lock().unwrap();

        let Some(backup) = app_state.backup.clone() else {
            return Err("".to_string());
        };

        (backup.client_id.clone(), backup.client_secret.clone())
    };

    // if not present return err
    if client_id.trim().is_empty() || client_secret.trim().is_empty() {
        return Err("Client id and client secret both need to be configured".to_string());
    }

    // construct the application secret
    let secret = ApplicationSecret {
        client_id,
        client_secret,
        token_uri: "https://oauth2.googleapis.com/token".to_string(),
        auth_uri: "https://accounts.google.com/o/oauth2/auth".to_string(),
        redirect_uris: vec!["http://localhost".to_string()],
        ..Default::default()
    };

    // build authenticator (opens browser once, cache refresh token)
    let auth = InstalledFlowAuthenticator::builder(secret, InstalledFlowReturnMethod::HTTPRedirect)
        .with_storage(Box::new(TauriTokenStorage::new(
            handle.clone(),
            TOKEN_PATH.to_string(),
        )))
        .flow_delegate(Box::new(TauriBrowserDelegate {
            app_handle: handle.clone(),
        }))
        .build()
        .await
        .map_err(|err| format!("Google authentication failed: {}", err))?;

    // 5. Trigger the browser login & save tokens to tokencache.json
    let scopes = &[DRIVE_FILE_SCOPE, DRIVE_METADATA_SCOPE];
    auth.token(scopes)
        .await
        .map_err(|err| format!("Google authentication failed: {}", err))?;

    Ok(())
}

#[tauri::command]
pub fn auth_token_created(handle: tauri::AppHandle) -> anyhow::Result<bool, String> {
    let Ok(store) = handle.store(TOKEN_PATH) else {
        return Err("Token store not preent".to_string());
    };

    let key = &[DRIVE_FILE_SCOPE, DRIVE_METADATA_SCOPE].join(" ");
    let Some(val) = store.get(key) else {
        return Err("No value inside token store".to_string());
    };

    match serde_json::from_value::<TokenInfo>(val) {
        Ok(info) => Ok(info.refresh_token.is_some()),
        Err(_) => Err("Error while parsing token".to_string()),
    }
}

pub async fn setup_auto_backup(
    app_handle: tauri::AppHandle,
) -> Result<(), Box<dyn std::error::Error>> {
    let backup_config = {
        let state = app_handle.state::<AppState>();
        let Ok(app_state) = state.lock() else {
            return Err("Unable to get app state".into());
        };
        let Some(backup_config) = app_state.backup.clone() else {
            return Err("Databse backup not configured".into());
        };
        backup_config
    };

    let scheduler = JobScheduler::new()
        .await
        .map_err(|err| -> Box<dyn std::error::Error> { Box::new(err) })?;
    let cron_expression = backup_config.cron;
    let app_handle_for_job = app_handle.clone();

    log::debug!("Cron expression: {}", cron_expression);

    let backup_job = Job::new_async(cron_expression.as_str(), move |_uuid, _l| {
        let app_handle = app_handle_for_job.clone();
        Box::pin(async move {
            if let Err(err) = backup_database(app_handle).await {
                log::error!("Automatic database backup failed: {}", err);
            }
        })
    })
    .map_err(|err| -> Box<dyn std::error::Error> { Box::new(err) })?;

    scheduler
        .add(backup_job)
        .await
        .map_err(|err| -> Box<dyn std::error::Error> { Box::new(err) })?;
    scheduler
        .start()
        .await
        .map_err(|err| -> Box<dyn std::error::Error> { Box::new(err) })?;

    Ok(())
}

#[tauri::command]
pub async fn backup_database(handle: tauri::AppHandle) -> Result<(), String> {
    if !auth_token_created(handle.clone())? {
        return Err("Token not present".to_string());
    }

    let db_config = {
        let state = handle.state::<AppState>();
        let app_state = state.lock().map_err(|e| e.to_string())?;
        app_state.database.clone()
    };

    let backup_dir = handle
        .path()
        .app_data_dir()
        .map_err(|err| err.to_string())?
        .join("backups");
    std::fs::create_dir_all(&backup_dir).map_err(|err| err.to_string())?;

    println!("Backup dir: {:?}", backup_dir);

    let (backup_file_path, file_name) = match run_mysqldump_backup(&db_config, backup_dir) {
        Ok((path, name)) => (path, name),
        Err(err) => return Err(err.to_string()),
    };

    println!("Backup dumbed: {:?}", backup_file_path);

    // create new reqwest client
    let client = Client::new();
    // get acecss token from store
    let Ok(store) = handle.store(TOKEN_PATH) else {
        return Err("Token store not present".to_string());
    };

    let key = &[DRIVE_FILE_SCOPE, DRIVE_METADATA_SCOPE].join(" ");
    let Some(val) = store.get(key) else {
        return Err("No value inside token store".to_string());
    };
    let Ok(token_info) = serde_json::from_value::<TokenInfo>(val) else {
        return Err("Error while parsing token".to_string());
    };

    let Some(access_token) = token_info.access_token else {
        return Err("Failed to get access token".to_string());
    };

    // create or get folder from google drive
    let folder_id = get_or_create_folder(&client, &access_token, BACKUP_FOLDER_NAME)
        .await
        .map_err(|err| err.to_string())?;

    println!("Folder id: {}", &folder_id);

    let Some(local_file_path) = backup_file_path.to_str() else {
        return Err("Unable to convert path to string".to_string());
    };

    upload_backup(
        &client,
        &access_token,
        &folder_id,
        &file_name,
        local_file_path,
    )
    .await
    .map_err(|err| err.to_string())?;

    Ok(())
}

async fn get_or_create_folder(
    client: &Client,
    token: &str,
    folder_name: &str,
) -> Result<String, Box<dyn Error>> {
    let query = format!(
        "name = '{}' and mimeType = 'application/vnd.google-apps.folder' and trashed = false",
        folder_name.replace('\'', "\\'")
    );

    let res: Value = client
        .get("https://www.googleapis.com/drive/v3/files")
        .bearer_auth(token)
        .header("Accept", "application/json")
        .query(&[("q", query.as_str()), ("fields", "files(id, name)")])
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    if let Some(files) = res["files"].as_array() {
        if let Some(first) = files.first() {
            if let Some(id) = first["id"].as_str() {
                return Ok(id.to_string());
            }
        }
    }

    let metadata = json!({
        "name": folder_name,
        "mimeType": "application/vnd.google-apps.folder"
    });

    let created: Value = client
        .post("https://www.googleapis.com/drive/v3/files")
        .bearer_auth(token)
        .json(&metadata)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    let new_id = created["id"]
        .as_str()
        .ok_or("Missing folder id in create response")?;

    Ok(new_id.to_string())
}

async fn upload_backup(
    client: &Client,
    token: &str,
    folder_id: &str,
    file_name: &str,
    local_path: &str,
) -> Result<(), Box<dyn Error>> {
    let file = tokio::fs::File::open(local_path).await?;
    let file_size = file.metadata().await?.len();

    let metadata = json!({
       "name": file_name,
      "parents": [folder_id]
    });

    let init_res = client
        .post("https://www.googleapis.com/upload/drive/v3/files?uploadType=resumable")
        .bearer_auth(token)
        .header("X-Upload-Content-Type", "application/gzip")
        .header("X-Upload-Content-Length", file_size.to_string())
        .json(&metadata)
        .send()
        .await?
        .error_for_status()?;

    let upload_uri = init_res
        .headers()
        .get("location")
        .ok_or("Missing Location header in resumable upload response")?
        .to_str()?;

    let upload_res: Value = client
        .put(upload_uri)
        .header("Content-Length", file_size.to_string())
        .header("Content-Type", "application/gzip")
        .body(reqwest::Body::from(file))
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    println!("Uploaded '{}' (ID: {})", file_name, upload_res["id"]);
    Ok(())
}

fn run_mysqldump_backup(
    database: &Database,
    backup_dir: PathBuf,
) -> Result<(PathBuf, String), Box<dyn std::error::Error + Send + Sync>> {
    // 1. Prepare file destination: backup_2026-10-18_19-37-00.sql.gz
    let timestamp = Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();
    let filename = format!("backup_{}.sql.gz", timestamp);
    let file_path = backup_dir.join(filename.clone());

    let file = File::create(file_path)?;
    let mut encoder = GzEncoder::new(&file, Compression::default());

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

    Ok((file.path().unwrap(), filename))
}
