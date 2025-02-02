use chrono;
use crate::{
    connection::establish_connection,
    models::{ TaxDetail, NewTaxDetails },
    db_ops::tax_details,
    utils
};

#[tauri::command]
pub fn has_tax_details_for(fy: String) -> Result<bool, String> {
    let mut conn = establish_connection().unwrap();

    tax_details::has_tax_details_for(fy, &mut conn).map_err(|err| err.to_string())
}

#[tauri::command]
pub fn find_all_tax_details(financial_year: String) -> Result<Vec<TaxDetail>, String> {
    let mut conn = establish_connection().unwrap();

    tax_details::find_all(financial_year, &mut conn).map_err(|_| "Error while getting tax details".to_string())
}

#[tauri::command]
pub fn find_latest_tax_details(financial_year: String) -> Result<TaxDetail, String> {
    let mut conn = establish_connection().map_err(|err| err.to_string())?;

    tax_details::find_latest(financial_year, &mut conn).map_err(|err| err.to_string())
}

#[tauri::command]
pub fn tax_details_get_all_fy() -> Result<Vec<String>, String> {
    let mut conn = establish_connection().map_err(|err| err.to_string())?;

    let mut fy_list = tax_details::get_all_financial_year_list(&mut conn).map_err(|err| err.to_string())?;
    let curr_fy = utils::get_current_fy();
    if !fy_list.contains(&curr_fy) {
        fy_list.push(curr_fy);
    }

    Ok(fy_list)
}

#[tauri::command]
pub fn save_tax_details(new_tax_details: NewTaxDetails) -> Result<usize, String> {
    let mut conn = establish_connection().unwrap();

    tax_details::save(new_tax_details, &mut conn).map_err(|err| err.to_string())
}

#[tauri::command]
pub fn update_tax_details(mut td: TaxDetail) -> Result<usize, String> {
    let mut conn = establish_connection().map_err(|err| err.to_string())?;

    td.modified_date = Some(chrono::Utc::now().naive_utc());
    tax_details::update(td, &mut conn).map_err(|err| err.to_string())
}