pub(crate) use crate::models::{AuthContext, VrcCurrentUser, VrcErrorResponse};

use base64::{engine::general_purpose::STANDARD as BASE64_STANDARD, Engine as _};
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use tauri_plugin_http::reqwest::header::{AUTHORIZATION, CONTENT_TYPE, SET_COOKIE, USER_AGENT};
use tauri_plugin_http::reqwest::Client as HttpClient;

const VRCHAT_API_BASE_URL: &str = "https://api.vrchat.cloud/api/1";

fn encode_vrchat_credentials(username: &str, password: &str) -> String {
    let encoded_username = utf8_percent_encode(username, NON_ALPHANUMERIC).to_string();
    let encoded_password = utf8_percent_encode(password, NON_ALPHANUMERIC).to_string();
    let combined = format!("{}:{}", encoded_username, encoded_password);
    BASE64_STANDARD.encode(combined)
}

//
fn extract_auth_cookie_value(
    response_headers: &reqwest::header::HeaderMap,
) -> Option<String> {
    for header_value in response_headers.get_all(SET_COOKIE) {
        if let Ok(cookie_str) = header_value.to_str() {
            if let Some(auth_part) = cookie_str
                .split(";")
                .find(|part| part.trim().starts_with("auth="))
            {
                return auth_part.trim().strip_prefix("auth=").map(String::from);
            }
        }
    }
    None
}

pub async fn authenticate_with_vrchat_credentials(
    http_client: &HttpClient,
    username: &str,
    password: &str,
) -> Result<AuthContext, String> {
    let encoded_credentials = encode_vrchat_credentials(username, password);
    let auth_header_value = format!("Basic {}", encoded_credentials);

    let request_url = format!("{}/auth/user", VRCHAT_API_BASE_URL);

    log::info!("Attempting login for user: {}", username);

    let crate_name = env!("CARGO_PKG_NAME");
    let crate_version = env!("CARGO_PKG_VERSION");

    let request = http_client
        .get(request_url.clone())
        .header(AUTHORIZATION, auth_header_value)
        .header(CONTENT_TYPE, "application/json;charset=utf-8")
        .header(USER_AGENT, format!("{} v{}", crate_name, crate_version));

    match request.send().await {
        Ok(response) => {
            log::debug!("API response status: {}", response.status());
            log::debug!("API response headers: {:?}", response.headers());

            let response_status = response.status().as_u16();
            let respose_headers = response.headers().clone();

            if response_status == 200 {
                let body_bytes = response.bytes().await.map_err(|e| {
                    log::error!("Failed to read response body: {}", e);
                    "Failed to read response body".to_string()
                })?;
                match serde_json::from_slice::<VrcCurrentUser>(&body_bytes) {
                    Ok(current_user) => {
                        log::info!("Login successful for user: {}", username);

                        if let Some(auth_cookie_value) = extract_auth_cookie_value(&respose_headers)
                        {
                            log::info!("Auth cookie value extracted");
                            Ok(AuthContext {
                                user: current_user,
                                auth_cookie_value,
                            })
                        } else {
                            log::error!("Auth cookie not found in response headers");
                            Err("Auth cookie not found in response headers".to_string())
                        }
                    }
                    Err(e) => {
                        log::error!("200 Ok, Failed to parse auth cookie: {}", e);
                        Err(format!("200 Ok, Failed to parse auth cookie: {}", e))
                    }
                }
            } else {
                let error_body_bytes = response.bytes().await.unwrap_or_default();
                match serde_json::from_slice::<VrcErrorResponse>(&error_body_bytes) {
                    Ok(error_response) => {
                        log::warn!(
                            "VRChat API error ({}): {}",
                            error_response.error.status_code,
                            error_response.error.message
                        );
                        Err(format!(
                            "VRChat API error ({}): {}",
                            error_response.error.status_code, error_response.error.message
                        ))
                    }
                    Err(_) => {
                        log::warn!(
                            "VRChat API returned status {} with non-JSON error body.",
                            response_status
                        );
                        let _ = String::from_utf8_lossy(&error_body_bytes);
                        Err(format!(
                            "VRChat API returned status {} with non-JSON error body.",
                            response_status
                        ))
                    }
                }
            }
        }
        Err(http_error) => {
            log::error!("VRChat API request failed: {:?}", http_error);
            Err(format!("HTTP request error: {:?}", http_error))
        }
    }
}
