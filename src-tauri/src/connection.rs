use diesel::mysql::MysqlConnection;
use diesel::result::ConnectionError;
use tauri::Manager;

use crate::state::AppState;

pub fn establish_connection(handle: tauri::AppHandle) -> Result<MysqlConnection, ConnectionError> {
    let state = handle.state::<AppState>();
    let app_state = state.lock().unwrap();
    
    app_state.database.establish_connection()
}
