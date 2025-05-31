use crate::AppState;
use api_models::EitherUserOrTwoFactor;
use botan_core::api_models::{
    self, TwoFactorAuthCode, TwoFactorEmailCode, Verify2FaEmailCodeResult, Verify2FaResult,
};
use botan_core::auth::auth_get_current_user;
use botan_core::models::LoginCredentials;
use reqwest::cookie::CookieStore;
use reqwest::Url;
use std::str::FromStr;
use std::sync::Arc;
use tauri::http::HeaderValue;

#[tauri::command]
pub async fn login(
    state: tauri::State<'_, AppState>,
    credentials: LoginCredentials,
) -> Result<EitherUserOrTwoFactor, String> {
    log::info!("Tauri command, api - 'auth/user', login");

    let basic_auth_data = (
        credentials.username.clone(),
        Some(credentials.password.clone()),
    );

    let login_config = &mut state.vrc_client.write().await.config;

    login_config.basic_auth = Some(basic_auth_data);

    match state.store.lock().unwrap().get("cookies") {
        Some(cookies) => {
            let jar = reqwest::cookie::Jar::default();
            jar.set_cookies(
                &mut [HeaderValue::from_str(
                    &serde_json::to_string(&cookies).expect("Failed to serialize cookies"),
                )
                .expect("Cookie not okay")]
                .iter(),
                &Url::from_str("https://api.vrchat.cloud").expect("Url not okay"),
            );
            let jar = Arc::new(jar);
            login_config.client = reqwest::Client::builder()
                .cookie_provider(jar)
                .build()
                .unwrap();
            log::info!("Cookies loaded from store, proceeding with cookies");
        }
        None => {
            let cookie_store = std::sync::Arc::new(reqwest::cookie::Jar::default());
            login_config.client = reqwest::Client::builder()
                .cookie_provider(cookie_store.clone())
                .build()
                .unwrap();
            log::warn!("No cookies found in store, proceeding without cookies");
        }
    }

    let login_result = auth_get_current_user(&login_config).await;

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

pub enum EitherTwoFactorAuthCodeType {
    IsA(TwoFactorAuthCode),
    IsB(TwoFactorEmailCode),
}

pub enum EitherTwoFactorResultType {
    IsA(Verify2FaResult),
    IsB(Verify2FaEmailCodeResult),
}

#[tauri::command]
pub async fn verify2_fa(
    state: tauri::State<'_, AppState>,
    two_fa_type: &str,
    code: &EitherTwoFactorAuthCodeType,
) -> Result<EitherTwoFactorResultType, String> {
    log::info!("Tauri command, api - 'auth/verify2fa', verify2_fa");

    let client = state.vrc_client.read().await;

    match two_fa_type {
        "2fa" => {
            if let EitherTwoFactorAuthCodeType::IsA(auth_code) = code {
                let result =
                    botan_core::auth::auth_verify2_fa(&client.config, auth_code.clone()).await;
                match result {
                    Ok(res) => Ok(EitherTwoFactorResultType::IsA(res)),
                    Err(e) => {
                        log::error!("Failed to verify 2FA auth code: {:?}", e);
                        Err(serde_json::to_string(&format!(
                            "Failed to verify 2FA auth code: {:?}",
                            e
                        ))
                        .unwrap())
                    }
                }
            } else {
                Err("Invalid code type for auth verification".to_string())
            }
        }
        "email" => {
            if let EitherTwoFactorAuthCodeType::IsB(email_code) = code {
                let result =
                    botan_core::auth::auth_verify2_fa_email(&client.config, email_code.clone())
                        .await;
                match result {
                    Ok(res) => Ok(EitherTwoFactorResultType::IsB(res)),
                    Err(e) => {
                        log::error!("Failed to verify 2FA email code: {:?}", e);
                        Err(serde_json::to_string(&format!(
                            "Failed to verify 2FA email code: {:?}",
                            e
                        ))
                        .unwrap())
                    }
                }
            } else {
                Err("Invalid code type for email verification".to_string())
            }
        }
        _ => Err("Unknown two-factor type".to_string()),
    }
}
