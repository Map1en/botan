pub mod commands;
pub mod core_logic;
pub mod models;

use commands::auth_user;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .invoke_handler(tauri::generate_handler![auth_user])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
