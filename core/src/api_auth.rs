use crate::client::{get_default_user_agent, VRCHAT_API_BASE_URL};
use crate::error::VRCError;
use crate::models::{AuthContext, VRCCurrentUser, VRCErrorResponse};
use base64::{engine::general_purpose::STANDARD as BASE64_STANDARD, Engine as _};
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE, SET_COOKIE, USER_AGENT};
use reqwest::Client as ReqwestClient;

pub(crate) fn encode_vrchat_credentials(username: &str, password: &str) -> String {
    let encoded_username = utf8_percent_encode(username, NON_ALPHANUMERIC).to_string();
    let encoded_password = utf8_percent_encode(password, NON_ALPHANUMERIC).to_string();
    let combined = format!("{}:{}", encoded_username, encoded_password);
    BASE64_STANDARD.encode(combined)
}

pub(crate) fn extract_auth_cookie_value(
    response_headers: &reqwest::header::HeaderMap,
) -> Option<String> {
    for header_value in response_headers.get_all(SET_COOKIE) {
        if let Ok(cookie_str) = header_value.to_str() {
            if let Some(auth_part) = cookie_str
                .split(';')
                .map(|part| part.trim())
                .find(|part| part.starts_with("auth="))
            {
                return auth_part
                    .strip_prefix("auth=")
                    .map(String::from)
                    .filter(|s| !s.is_empty());
            }
        }
    }
    None
}
#[derive(Debug)]
pub struct AuthError {
    pub message: String,
    pub auth_cookie_value: Option<String>,
}

impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(cookie) = &self.auth_cookie_value {
            write!(f, "{} (Auth Cookie found: {})", self.message, cookie)
        } else {
            write!(
                f,
                "{} (No valid Auth Cookie found in headers)",
                self.message
            )
        }
    }
}

impl std::error::Error for AuthError {}

pub(crate) async fn authenticate_with_vrchat_username_password(
    http_client: &ReqwestClient,
    username: &str,
    password: &str,
) -> Result<AuthContext, VRCError> {
    let encoded_credentials = encode_vrchat_credentials(username, password);
    let auth_header_value = format!("Basic {}", encoded_credentials);
    let request_url = format!("{}/auth/user", VRCHAT_API_BASE_URL);

    log::info!("Attempting login for user: {}", username);

    let response = http_client
        .get(&request_url)
        .header(AUTHORIZATION, auth_header_value)
        .header(CONTENT_TYPE, "application/json;charset=utf-8")
        .header(USER_AGENT, get_default_user_agent())
        .send()
        .await?;

    let response_status = response.status();
    let response_headers = response.headers().clone();

    if response_status.is_success() {
        let current_user = response.json::<VRCCurrentUser>().await?;
        if let Some(auth_cookie_val) = extract_auth_cookie_value(&response_headers) {
            log::info!("(Core Lib Raw Auth) Extracted auth cookie.");
            Ok(AuthContext {
                user: current_user,
                auth_cookie_value: auth_cookie_val,
            })
        } else {
            Err(VRCError::CookieExtractionFailed(
                "No valid auth cookie found in response headers.".to_string(),
            ))
        }
    } else {
        let raw_body_text = response.text().await.ok();
        let message = format!("API request failed with status {}", response_status);
        if let Some(ref body_str) = raw_body_text {
            if let Ok(vrc_error) = serde_json::from_str::<VRCErrorResponse>(body_str) {
                return Err(VRCError::ApiError {
                    status_code: vrc_error.error.status_code,
                    message: vrc_error.error.message,
                    raw_body: Some(body_str.clone()),
                });
            }
        }
        log::warn!("(Core Raw Auth) {}", message);
        Err(VRCError::ApiError {
            status_code: response_status.as_u16(),
            message,
            raw_body: raw_body_text,
        })
    }
}
