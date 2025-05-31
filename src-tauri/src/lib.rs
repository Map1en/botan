pub mod commands;
use botan_core::client::VrcApiClient;
use std::sync::{Arc, Mutex};
use tauri::{Manager, Wry};
use tauri_plugin_store::{Store, StoreExt};
use tokio::sync::RwLock;

pub struct AppState {
    vrc_client: RwLock<VrcApiClient>,
    store: Arc<Mutex<Arc<Store<Wry>>>>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let vrc_client_instance = VrcApiClient::new();

    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .setup(|app| {
            let store = app.store("store.json")?;
            let app_state = AppState {
                vrc_client: vrc_client_instance.into(),
                store: Arc::new(Mutex::new(store)),
            };
            app.manage(app_state);
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
            commands::login,
            commands::verify2_fa
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
