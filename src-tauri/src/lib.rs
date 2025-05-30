pub mod commands;
use botan_core::client::VrcApiClient;
use std::sync::Mutex;

pub struct AppState {
    vrc_client: Mutex<VrcApiClient>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let vrc_client_instance = VrcApiClient::new();
    let app_state = AppState {
        vrc_client: Mutex::new(vrc_client_instance),
    };

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .invoke_handler(tauri::generate_handler![commands::auth_user])
        .manage(app_state)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
