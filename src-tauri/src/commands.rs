use api_models::EitherUserOrTwoFactor;
use botan_core::api_models::{self};
use botan_core::auth::auth_and_get_current_user;
use botan_core::models::{
    EitherTwoFactorAuthCodeType, EitherTwoFactorResultType, LoginCredentials,
};
use tauri::Manager;

#[tauri::command]
pub async fn login(
    app_handle: tauri::AppHandle,
    credentials: Option<LoginCredentials>,
) -> Result<EitherUserOrTwoFactor, String> {
    log::info!("Tauri command, api - 'auth/user', login");

    let cookies_path_buf = app_handle
        .path()
        .app_config_dir()
        .expect("Failed to get app config directory")
        .join("web.dat");
    let cookies_path = cookies_path_buf
        .to_str()
        .expect("Failed to convert path to string");

    let login_result = auth_and_get_current_user(&credentials, cookies_path).await;

    match login_result {
        Ok(either_result) => match either_result {
            api_models::EitherUserOrTwoFactor::CurrentUser(user_data) => {
                log::info!(
                    "Login successful for user: {}",
                    serde_json::to_string(&user_data).unwrap_or_default()
                );

                Ok(api_models::EitherUserOrTwoFactor::CurrentUser(user_data))
            }
            api_models::EitherUserOrTwoFactor::RequiresTwoFactorAuth(two_factor_data) => {
                log::info!(
                    "Login requires 2FA for user: {}",
                    serde_json::to_string(&two_factor_data).unwrap_or_default()
                );
                Ok(api_models::EitherUserOrTwoFactor::RequiresTwoFactorAuth(
                    two_factor_data,
                ))
            }
        },
        Err(e) => {
            log::error!("Login failed: {:?}", e);
            Err(serde_json::to_string(&format!("Login failed: {:?}", e)).unwrap())
        }
    }
}

#[tauri::command]
pub async fn verify2_fa(
    two_fa_type: &str,
    code: EitherTwoFactorAuthCodeType,
) -> Result<EitherTwoFactorResultType, String> {
    log::info!("Tauri command, api - 'auth/verify2fa', verify2_fa");

    match botan_core::auth::verify2_fa(two_fa_type, code).await {
        Ok(result) => {
            log::info!("2FA verification successful");
            Ok(result)
        }
        Err(e) => {
            log::error!("2FA verification failed: {:?}", e);
            Err(format!("2FA verification failed: {}", e))
        }
    }
}
