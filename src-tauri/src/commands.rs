use botan_core::auth::auth_and_get_current_user;
use botan_core::models::response::ApiResponse;
use botan_core::models::TwoFactorVerifyResult;
use botan_core::models::{EitherTwoFactorAuthCodeType, LoginCredentials};
use botan_core::vrchatapi_models::EitherUserOrTwoFactor;
use tauri::Manager;

fn get_cookies_path(app_handle: &tauri::AppHandle) -> String {
    let cookies_path_buf = app_handle
        .path()
        .app_config_dir()
        .expect("Failed to get app config directory")
        .join("web.dat");
    cookies_path_buf
        .to_str()
        .expect("Failed to convert path to string")
        .to_string()
}

#[tauri::command]
pub async fn login(
    app_handle: tauri::AppHandle,
    credentials: Option<LoginCredentials>,
) -> ApiResponse<EitherUserOrTwoFactor> {
    log::info!("Tauri command, api - 'auth/user', login");

    let cookies_path = get_cookies_path(&app_handle);

    log::info!("cookies path11111111: {}", cookies_path);

    auth_and_get_current_user(&credentials, &cookies_path).await
}

#[tauri::command]
pub async fn verify2_fa(
    two_fa_type: String,
    code: EitherTwoFactorAuthCodeType,
) -> ApiResponse<TwoFactorVerifyResult> {
    log::info!("Tauri command, api - 'auth/verify2fa', verify2_fa");

    botan_core::auth::verify2_fa(&two_fa_type, code).await
}
