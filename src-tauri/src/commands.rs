use crate::AppState;
use api_models::EitherUserOrTwoFactor;
use botan_core::api_models;
use botan_core::auth::login;
use botan_core::models::LoginCredentials;
use reqwest::cookie::CookieStore;
use reqwest::Url;
use serde_json::json;
use std::str::FromStr;

#[tauri::command]
pub async fn auth_user(
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

    let mut login_config = state.vrc_client.config.clone();

    let cookie_store = std::sync::Arc::new(reqwest::cookie::Jar::default());
    login_config.client = reqwest::Client::builder()
        .cookie_provider(cookie_store.clone())
        .build()
        .unwrap();

    login_config.basic_auth = Some(basic_auth_data);

    let login_result = login(&login_config).await;

    if let Ok(url) = Url::from_str("https://api.vrchat.cloud") {
        if let Some(cookie) = cookie_store.cookies(&url) {
            if let Ok(cookie_str) = cookie.to_str() {
                log::info!("Cookies: {}", cookie_str);
                state
                    .store
                    .lock()
                    .unwrap()
                    .set("cookies", json!({"cookie":&cookie_str}))
            } else {
                log::warn!("Failed to convert cookies to string");
            }
        }
    } else {
        log::warn!("Failed to parse URL");
    }

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
