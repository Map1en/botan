use crate::AppState;
use api_models::EitherUserOrTwoFactor;
use botan_core::api_models;
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

    log::info!(
        "Cookie store: {:?}",
        state.store.lock().unwrap().get("cookies")
    );

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
            Err(format!("Login failed: {:?}", e))
        }
    }
}
