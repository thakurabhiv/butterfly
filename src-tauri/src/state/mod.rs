use std::sync::Mutex;
use std::fs;
use diesel::Connection;
use serde::{ Deserialize, Serialize };

use diesel::mysql::MysqlConnection;
use diesel::result::ConnectionError;

use crate::constants::CONFIG_FILE_PATH;

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

pub fn read_app_config_file() -> Result<AppState, Box<dyn std::error::Error>> {
    let config_content = match fs::read_to_string(CONFIG_FILE_PATH) {
        Ok(content) => content,
        Err(e) => return Err(Box::new(e)),
    };

    let data: AppState = toml::from_str(&config_content)?;

    Ok(data)
}