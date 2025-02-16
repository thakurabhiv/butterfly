use crate::{
    connection::establish_connection,
    db_ops::sales_invoice,
    models::{
        InvoiceDetail, InvoiceDetailsWithProduct, InvoiceSummary, NewInvoiceDetail,
        NewInvoiceSummary, TaxDetail, VendorDetail,
    },
    utils,
};

#[tauri::command]
pub fn find_sales_invoice_number(handle: tauri::AppHandle, query: String) -> Result<Vec<InvoiceSummary>, String> {
    let mut conn = establish_connection(handle).map_err(|err| err.to_string())?;

    sales_invoice::search(query, &mut conn).map_err(|err| err.to_string())
}

#[tauri::command]
pub fn sales_invoice_get_all_fy(handle: tauri::AppHandle) -> Result<Vec<String>, String> {
    let mut conn = establish_connection(handle).map_err(|err| err.to_string())?;

    let mut fy_list =
        sales_invoice::get_all_financial_year_list(&mut conn).map_err(|err| err.to_string())?;

    let curr_fy = utils::get_current_fy();
    if !fy_list.contains(&curr_fy) {
        fy_list.push(curr_fy);
    }

    Ok(fy_list)
}

#[tauri::command]
pub fn next_invoice_id(handle: tauri::AppHandle, fy: String) -> Result<String, String> {
    let mut conn = establish_connection(handle).map_err(|err| err.to_string())?;

    sales_invoice::next_invoice_id(fy, &mut conn).map_err(|err| err.to_string())
}

#[tauri::command]
pub fn save_invoice_data(
    handle: tauri::AppHandle,
    new_invoice_summary: NewInvoiceSummary,
    new_invoice_details: Vec<NewInvoiceDetail>,
    fy: String,
) -> Result<(), String> {
    let mut conn = establish_connection(handle).map_err(|err| err.to_string())?;

    sales_invoice::save_invoice(new_invoice_summary, new_invoice_details, fy, &mut conn)
        .map_err(|err| err.to_string())
}

#[tauri::command]
pub fn update_invoice_data(
    handle: tauri::AppHandle,
    invoice_summary: InvoiceSummary,
    invoice_details: Vec<InvoiceDetail>,
) -> Result<(), String> {
    let mut conn = establish_connection(handle).map_err(|err| err.to_string())?;

    sales_invoice::update_invoice(invoice_summary, invoice_details, &mut conn)
        .map_err(|err| err.to_string())
}

#[tauri::command]
pub fn get_invoice_with_details(
    handle: tauri::AppHandle,
    invoice_summary: InvoiceSummary,
) -> Result<
    (
        InvoiceSummary,
        VendorDetail,
        TaxDetail,
        Vec<InvoiceDetailsWithProduct>,
    ),
    String,
> {
    let mut conn = establish_connection(handle).map_err(|err| err.to_string())?;

    sales_invoice::get_invoice_with_details(invoice_summary, &mut conn)
        .map_err(|err| err.to_string())
}
