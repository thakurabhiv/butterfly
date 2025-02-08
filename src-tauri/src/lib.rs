use tauri::Manager;

mod commands;
mod connection;
mod constants;
mod db_ops;
mod models;
mod schema;
mod utils;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::new()
            .target(tauri_plugin_log::Target::new(
                tauri_plugin_log::TargetKind::LogDir { file_name: Some("logs".to_string()) }
            ))
            .max_file_size(50_000)
            .rotation_strategy(tauri_plugin_log::RotationStrategy::KeepAll)
            .build())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_single_instance::init(|app, _, _| {
            let _ = app
                .get_webview_window("main")
                .expect("no main window")
                .set_focus();
        }))
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            commands::tax_details::find_all_tax_details,
            commands::tax_details::find_latest_tax_details,
            commands::tax_details::save_tax_details,
            commands::tax_details::update_tax_details,
            commands::tax_details::tax_details_get_all_fy,
            commands::tax_details::has_tax_details_for,
            commands::branch_owner_details::find_all_branch_owner_details,
            commands::branch_owner_details::save_branch_owner_details,
            commands::branch_owner_details::update_branch_owner_details,
            commands::product_details::find_all_product_details,
            commands::product_details::find_products,
            commands::product_details::get_multiple_products,
            commands::product_details::save_product_details,
            commands::product_details::update_product_details,
            commands::vendor_details::find_vendors,
            commands::vendor_details::find_all_vendors,
            commands::vendor_details::save_vendor_details,
            commands::vendor_details::update_vendor_details,
            commands::sales_invoice::sales_invoice_get_all_fy,
            commands::sales_invoice::next_invoice_id,
            commands::sales_invoice::save_invoice_data,
            commands::sales_invoice::update_invoice_data,
            commands::sales_invoice::find_sales_invoice_number,
            commands::sales_invoice::get_invoice_with_details,
            commands::common::get_qr_code,
            commands::common::find_states,
            commands::common::get_state,
            commands::common::log,
            commands::common::get_current_fy,
            commands::common::get_all_financial_year,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
