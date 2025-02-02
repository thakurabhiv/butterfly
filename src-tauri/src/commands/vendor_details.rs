use crate::{
    connection::establish_connection,
    models::{ VendorDetail, NewVendorDetails },
    db_ops::vendor_details
};

#[tauri::command]
pub fn find_vendors(name: String) -> Result<Vec<VendorDetail>, String> {
    let mut conn = establish_connection().map_err(|err| err.to_string())?;
    
    vendor_details::search(name, &mut conn).map_err(|err| err.to_string())
}

#[tauri::command]
pub fn find_all_vendors() -> Result<Vec<VendorDetail>, String> {
    let mut conn = establish_connection().map_err(|err| err.to_string())?;
    
    vendor_details::find_all(&mut conn).map_err(|err| err.to_string())
}

#[tauri::command]
pub fn save_vendor_details(data: NewVendorDetails) -> Result<usize, String> {
    let mut conn = establish_connection().map_err(|err| err.to_string())?;
    
    vendor_details::save(data, &mut conn).map_err(|err| err.to_string())
}

#[tauri::command]
pub fn update_vendor_details(mut data: VendorDetail) -> Result<usize, String> {
    let mut conn = establish_connection().map_err(|err| err.to_string())?;

    data.modified_date = Some(chrono::Utc::now().naive_utc());
    vendor_details::update(data, &mut conn).map_err(|err| err.to_string())
}