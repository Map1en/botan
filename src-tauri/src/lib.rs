pub mod commands;
pub mod core_logic;
pub mod models;

use commands::{login, mock_login};


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .invoke_handler(tauri::generate_handler![login, mock_login])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
