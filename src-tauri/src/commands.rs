use crate::AppState;
use api_models::EitherUserOrTwoFactor;
use botan_core::api_models;
use botan_core::auth::login;
use botan_core::models::LoginCredentials;

#[tauri::command]
pub async fn auth_user(
    state: tauri::State<'_, AppState>,
    credentials: LoginCredentials,
) -> Result<EitherUserOrTwoFactor, String> {
    log::info!("Tauri command, api - 'auth/user', login");

    let basic_auth_data = (
        credentials.username.clone(),
        Some(credentials.password.clone()),
    );

    let login_config;
    {
        let mut client_guard = state.vrc_client.lock().map_err(|e| {
            log::error!("Failed to lock VrcApiClient: {}", e.to_string());
            e.to_string()
        })?;

        client_guard.config.basic_auth = Some(basic_auth_data);
        login_config = client_guard.config.clone();
    }

    let login_result = login(&login_config).await;

    match login_result {
        Ok(user_data) => {
            log::info!(
                "Login successful for user: {}",
                serde_json::to_string(&user_data).unwrap_or_default()
            );
            Ok(user_data)
        }
        Err(e) => {
            log::error!("Login failed: {}", e);
            Err(format!("Login failed: {}", e))
        }
    }
}
