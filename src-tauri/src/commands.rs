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

#[tauri::command]
pub async fn mock_login(
    _app_handle: AppHandle<Wry>,
    credentials: LoginCrendentials,
) -> Result<VrcCurrentUser, String> {
    log::info!("Using mock login for user: {}", credentials.username);
    
    // 模拟网络延迟
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    
    match (credentials.username.as_str(), credentials.password.as_str()) {
        ("vivi", "vivi") => {
            Ok(VrcCurrentUser {
                id: "usr_vivi_123".to_string(),
                username: credentials.username,
                display_name: "Vivi".to_string(),
                bio: Some("This is Vivi's test account".to_string()),
                current_avatar_thumbnail_image_url: Some("https://example.com/vivi-avatar.png".to_string()),
                status: Some("active".to_string()),
                last_login: Some(chrono::Utc::now().to_rfc3339()),
                email_verified: Some(true),
                requires_two_factor_auth: None,
            })
        },
        _ => Err("Invalid mock credentials".to_string())
    }
}
