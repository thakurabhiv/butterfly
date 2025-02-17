use std::fs::File;
use std::io::Write;

use fast_qr::convert::Builder;
use fast_qr::convert::{image::ImageBuilder, Shape};
use fast_qr::qr::QRBuilder;
use tauri::Manager;

use crate::connection::establish_connection;
use crate::db_ops::{ financial_year, state_list };
use crate::models::{ FinancialYear, StateList };
use crate::state::{ AppState, Config };
use crate::constants::CONFIG_FILE_PATH;

#[tauri::command]
pub fn get_qr_code(text: String, width: u32) -> Result<Vec<u8>, String> {
    let qr_code = QRBuilder::new(text)
        .build()
        .map_err(|err| err.to_string())?;

    ImageBuilder::default()
        .shape(Shape::Square)
        .background_color([255, 255, 255])
        .fit_width(width)
        .to_bytes(&qr_code)
        .map_err(|err| err.to_string())
}

#[tauri::command]
pub fn find_states(handle: tauri::AppHandle, query: String) -> Result<Vec<StateList>, String> {
    let mut conn = establish_connection(handle).map_err(|err| err.to_string())?;

    state_list::search(query, &mut conn).map_err(|err| err.to_string())
}

#[tauri::command]
pub fn get_state(handle: tauri::AppHandle, state_id: i32) -> Result<StateList, String> {
    let mut conn = establish_connection(handle).map_err(|err| err.to_string())?;

    state_list::get_state(state_id, &mut conn).map_err(|err| err.to_string())
}

#[tauri::command]
pub fn log(msg: String, log_type: String) {
    match log_type.as_ref() {
        "info" => log::info!("{}", msg),
        "error" => log::error!("{}", msg),
        _ => {}
    };
}

#[tauri::command]
pub fn get_current_fy() -> Result<String, String> {
    Ok(crate::utils::get_current_fy())
}

#[tauri::command]
pub fn get_all_financial_year(handle: tauri::AppHandle) -> Result<Vec<FinancialYear>, String> {
    let mut conn = establish_connection(handle).map_err(|err| err.to_string())?;

    financial_year::get_all_financal_years(&mut conn).map_err(|err| err.to_string())
}

#[tauri::command]
pub fn get_app_config(handle: tauri::AppHandle) -> Result<Config, String> {
    let state = handle.state::<AppState>();
    let app_state = state.lock().map_err(|e| e.to_string())?;

    Ok(app_state.clone())
}

#[tauri::command]
pub fn save_app_ui_mode(handle: tauri::AppHandle, mode: String) -> Result<(), String> {
    let state = handle.state::<AppState>();
    let mut app_state = state.lock().unwrap();
    app_state.ui.mode = mode;

    let config_content = toml::to_string(&app_state.clone())
        .map_err(|e| e.to_string())?;

    let mut file = File::create(CONFIG_FILE_PATH).map_err(|e| e.to_string())?;
    file.write_all(config_content.as_bytes()).map_err(|e| e.to_string())?;
    
    Ok(())
}