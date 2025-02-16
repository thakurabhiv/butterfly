use crate::{
    db_ops::branch_owner_details,
    models::{BranchOwnerDetail, NewBranchOwnerDetail},
    connection::establish_connection
};

#[tauri::command]
pub fn find_all_branch_owner_details(handle: tauri::AppHandle) -> Result<Vec<BranchOwnerDetail>, String> {
    let mut conn = establish_connection(handle).map_err(|err| err.to_string())?;

    branch_owner_details::find_all(&mut conn).map_err(|err| err.to_string())
}

#[tauri::command]
pub fn save_branch_owner_details(handle: tauri::AppHandle, new_bod: NewBranchOwnerDetail) -> Result<usize, String> {
    let mut conn = establish_connection(handle).map_err(|err| err.to_string())?;

    branch_owner_details::save(new_bod, &mut conn).map_err(|err| err.to_string())
}

#[tauri::command]
pub fn update_branch_owner_details(handle: tauri::AppHandle, mut bod: BranchOwnerDetail) -> Result<usize, String> {
    let mut conn = establish_connection(handle).map_err(|err| err.to_string())?;

    bod.modified_date = Some(chrono::Utc::now().naive_utc());
    branch_owner_details::update(bod, &mut conn).map_err(|err| err.to_string())
}
