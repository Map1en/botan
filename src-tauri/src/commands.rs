use botan_core::auth::auth_login_and_get_current_user;
use botan_core::models::response::ApiResponse;
use botan_core::models::TwoFactorVerifyResult;
use botan_core::models::{EitherTwoFactorAuthCodeType, LoginCredentials};
use botan_core::vrchatapi_models::EitherUserOrTwoFactor;
// use tauri::Manager;

// fn get_cookies_path(app_handle: &tauri::AppHandle) -> Option<String> {
//     match app_handle.path().app_config_dir() {
//         Ok(cookies_path_buf) => match cookies_path_buf.to_str() {
//             Some(path_str) => Some(path_str.to_string()),
//             None => {
//                 log::error!("Failed to convert cookies path to string");
//                 None
//             }
//         },
//         Err(e) => {
//             log::error!("Failed to get app config directory: {}", e);
//             None
//         }
//     }
// }

#[tauri::command]
pub async fn login(
    // app_handle: tauri::AppHandle,
    credentials: Option<LoginCredentials>,
) -> ApiResponse<EitherUserOrTwoFactor> {
    log::info!(
        "Tauri command, api - 'auth/user', login, credentials: {:?}",
        credentials
    );
    // let cookies_path = get_cookies_path(&app_handle);
    auth_login_and_get_current_user(&credentials).await
}

#[tauri::command]
pub async fn verify2_fa(
    two_fa_type: String,
    code: EitherTwoFactorAuthCodeType,
) -> ApiResponse<TwoFactorVerifyResult> {
    log::info!(
        "Tauri command, api - 'auth/verify2fa', auth_verify2_fa, two_fa_type: {:?}, code: {:?}",
        two_fa_type,
        code
    );
    botan_core::auth::auth_verify2_fa(&two_fa_type, code).await
}
