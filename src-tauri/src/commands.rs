use botan_core::client::VRCApiClient;
use botan_core::models::{LoginCrendentials, VRCCurrentUser};
use tauri::{AppHandle, Runtime};

#[tauri::command]
pub async fn auth_user(
    _app_handle: AppHandle<impl Runtime>,
    credentials: LoginCrendentials,
) -> Result<VRCCurrentUser, String> {
    log::info!("Tauri command, api - 'auth/user', login");

    match VRCApiClient::login(&credentials.username, &credentials.password).await {
        Ok((_api_client, user_data_from_login)) => {
            log::info!(
                "Login successful for user: {}",
                serde_json::to_string(&user_data_from_login).unwrap_or_default()
            );
            Ok(user_data_from_login)
        }
        Err(e) => {
            log::error!("Login failed: {}", e);
            Err(format!("Login failed: {}", e))
        }
    }
}
