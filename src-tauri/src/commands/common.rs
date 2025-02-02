use fast_qr::convert::Builder;
use fast_qr::qr::QRBuilder;
use fast_qr::convert::{ image::ImageBuilder, Shape };

use crate::connection::establish_connection;
use crate::db_ops::{ state_list, financial_year };
use crate::models::{ StateList, FinancialYear };

#[tauri::command]
pub fn get_qr_code(text: String, width: u32) -> Result<Vec<u8>, String> {
    let qr_code = QRBuilder::new(text)
        .build().map_err(|err| err.to_string())?;

    ImageBuilder::default()
        .shape(Shape::Square)
        .background_color([255, 255, 255])
        .fit_width(width)
        .to_bytes(&qr_code)
        .map_err(|err| err.to_string())
}

#[tauri::command]
pub fn find_states(query: String) -> Result<Vec<StateList>, String> {
    let mut conn = establish_connection().map_err(|err| err.to_string())?;

    state_list::search(query, &mut conn).map_err(|err| err.to_string())
}

#[tauri::command]
pub fn get_state(state_id: i32) -> Result<StateList, String> {
    let mut conn = establish_connection().map_err(|err| err.to_string())?;

    state_list::get_state(state_id, &mut conn).map_err(|err| err.to_string())
}

#[tauri::command]
pub fn log(text: String) {
    print!("{}", text);
}

#[tauri::command]
pub fn get_current_fy() -> Result<String, String> {
    Ok(crate::utils::get_current_fy())
}

#[tauri::command]
pub fn get_all_financial_year() -> Result<Vec<FinancialYear>, String> {
    let mut conn = establish_connection().map_err(|err| err.to_string())?;

    financial_year::get_all_financal_years(&mut conn).map_err(|err| err.to_string())
}