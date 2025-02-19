use std::io::Write;
use std::sync::Mutex;
use std::fs;

use directories;
use diesel::Connection;
use serde::{ Deserialize, Serialize };

use diesel::mysql::MysqlConnection;
use diesel::result::ConnectionError;

use crate::constants::CONFIG_FILE_NAME;

const CONFIG_TEMPL: &'static str = include_str!("../../config/config_templ.toml");

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Database {
    pub db_type: String,
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub db_name: Option<String>,
}

impl Database {
    fn connection_url(&self) -> String {
        format!(
            "mysql://{}:{}@{}:{}/{}",
            self.username, self.password,
            self.host, self.port, self.db_name.as_ref().unwrap()
        )
    }

    pub fn establish_connection(&self) -> Result<MysqlConnection, ConnectionError> {
        MysqlConnection::establish(&self.connection_url())
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Sidecar {
    pub name: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UI {
    pub mode: String,
    pub toast_rich_colors: bool,
    pub date_format: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    pub database: Database,
    pub sidecar: Sidecar,
    pub ui: UI
}

pub type AppState = Mutex<Config>;

pub fn create_config_file_templ() -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
    let config_dir = get_config_dir()?;
    log::debug!("Config directory path: {:?}", config_dir);

    log::debug!("Creating config directory path");
    std::fs::create_dir_all(&config_dir)?;

    let config_path = config_dir.join(CONFIG_FILE_NAME);
    let exists = std::fs::exists(&config_path)?;
    
    if !exists {
        log::debug!("Writing config template to config directory");
        let mut file = std::fs::File::create(&config_path)?;
        file.write_all(CONFIG_TEMPL.as_bytes())?;
    } else {
        log::debug!("Config file already exists");
    }

    Ok(config_path)
}

pub fn get_config_dir() -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
    if let Some(proj_dirs) = directories::ProjectDirs::from(
        "", "com.butterfly", ""
    ) {
        let config_dir = proj_dirs.config_dir();
        return Ok(std::path::PathBuf::from(config_dir));
    } else {
        return Err("Unable to get project directories".into());
    }
}

pub fn read_app_config_file(path: std::path::PathBuf) -> Result<AppState, Box<dyn std::error::Error>> {
    let config_content = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(e) => return Err(Box::new(e)),
    };

    let data: AppState = toml::from_str(&config_content)?;

    Ok(data)
}
