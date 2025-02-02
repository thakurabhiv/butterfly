use crate::{connection::establish_connection, models::{NewInvoiceSummary, InvoiceSummary}, database_ops};
use chrono;

#[tauri::command]
pub fn insert_invoice_summary() -> Result<Vec<i32>, String> {
    let mut conn = establish_connection().unwrap();
    let mut res = Vec::new();

    for i in 1..10 {
        let new_invoice_summary = NewInvoiceSummary {
            invoice_number: &format!("INV00{i}"),
            invoice_date: Some(chrono::Utc::now().naive_local()),
            financial_year: "2023-24",
            place_of_supply: "Chaul",
            vendor_id: 100,
            comments: Some("Comments"),
            is_deleted: false,
        };

        let invoice_summary = database_ops::new_invoice_summary(new_invoice_summary, &mut conn);
        res.push(invoice_summary.id);
    }

    Ok(res)
}

#[tauri::command]
pub fn get_invoice_summary() -> Result<Vec<InvoiceSummary>, String> {
    let mut conn = establish_connection().unwrap();

    Ok(database_ops::get_invoice_summary(&mut conn))
}