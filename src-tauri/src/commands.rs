use crate::core_logic::authenticate_with_vrchat_credentials;
use crate::models::{LoginCrendentials, VrcCurrentUser};
use tauri::{AppHandle, Wry};

#[tauri::command]
pub async fn login(
    _app_handle: AppHandle<Wry>,
    credentials: LoginCrendentials,
) -> Result<VrcCurrentUser, String> {
    log::info!("login_command");

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
            Ok(auth_context.user)
        }
        Err(e) => {
            log::error!("Login failed: {}", e);
            Err(e)
        }
    }
}
