use std::sync::Mutex;
use std::fs;
use serde::{ Deserialize, Serialize };

use crate::constants::CONFIG_FILE_PATH;

#[derive(Deserialize, Serialize)]
pub struct Database {
    pub db_type: String,
    pub url: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub db_name: String,
}

#[derive(Deserialize, Serialize)]
pub struct Sidecar {
    pub name: String,
    pub arguments: Vec<String>,
}

#[derive(Deserialize, Serialize)]
pub struct UI {
    pub mode: String,
    pub toast_rich_colors: bool,
    pub date_format: String,
}

#[derive(Deserialize, Serialize)]
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