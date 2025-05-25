// use std::path::PathBuf;
use tauri_plugin_store::{Store};
use tauri::{AppHandle, Manager, Wry};

use crate::core_logic::authenticate_with_vrchat_credentials;
use crate::models::{LoginCrendentials, VrcCurrentUser};


// const AUTH_STORE_PATH: &str = "authcache.bin";
const AUTH_COOKIE_KEY: &str = "vrchat_auth_cookie";

#[tauri::command]
pub async fn auth_user(
    app_handle: AppHandle<Wry>,
    credentials: LoginCrendentials,
) -> Result<VrcCurrentUser, String> {
    log::info!("auth/user, login");

    let http_client = reqwest::Client::new();

    match authenticate_with_vrchat_credentials(
        &http_client,
        &credentials.username,
        &credentials.password,
    )
    .await
    {
        Ok(auth_context) => {
            log::info!("Login successful for user: {}", auth_context.user.username);
            
            // let store_path = PathBuf::from(AUTH_STORE_PATH);
            let stores = app_handle.state::<Store<Wry>>();

            stores.set(AUTH_COOKIE_KEY, auth_context.auth_cookie_value.clone());
            
            Ok(auth_context.user)
        }
        Err(e) => {
            log::error!("Login failed: {}", e);
            Err(e.message)
        }
    }
}
