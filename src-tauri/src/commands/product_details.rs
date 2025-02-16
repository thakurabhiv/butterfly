use crate::{
    connection::establish_connection,
    db_ops::product_details,
    models::{NewProductDetails, ProductDetail},
};

#[tauri::command]
pub fn find_all_product_details(handle: tauri::AppHandle) -> Result<Vec<ProductDetail>, String> {
    let mut conn = establish_connection(handle).map_err(|err| err.to_string())?;

    product_details::find_all(&mut conn).map_err(|err| err.to_string())
}

#[tauri::command]
pub fn find_products(handle: tauri::AppHandle, query: String) -> Result<Vec<ProductDetail>, String> {
    let mut conn = establish_connection(handle).map_err(|err| err.to_string())?;

    product_details::search(query, &mut conn).map_err(|err| err.to_string())
}

#[tauri::command]
pub fn get_multiple_products(handle: tauri::AppHandle, product_ids: Vec<i32>) -> Result<Vec<ProductDetail>, String> {
    let mut conn = establish_connection(handle).map_err(|err| err.to_string())?;

    product_details::get_multiple_products(product_ids, &mut conn).map_err(|err| err.to_string())
}

#[tauri::command]
pub fn save_product_details(handle: tauri::AppHandle, data: NewProductDetails) -> Result<usize, String> {
    let mut conn = establish_connection(handle).map_err(|err| err.to_string())?;

    product_details::save(data, &mut conn).map_err(|err| err.to_string())
}

#[tauri::command]
pub fn update_product_details(handle: tauri::AppHandle, mut data: ProductDetail) -> Result<usize, String> {
    let mut conn = establish_connection(handle).map_err(|err| err.to_string())?;

    data.modified_date = Some(chrono::Utc::now().naive_utc());
    product_details::update(data, &mut conn).map_err(|err| err.to_string())
}
