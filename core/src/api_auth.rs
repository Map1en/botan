use base64::{engine::general_purpose::STANDARD as BASE64_STANDARD, Engine as _};
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE, USER_AGENT, SET_COOKIE};
use reqwest::Client;

const VRCHAT_API_BASE_URL: &str = "https://api.vrchat.cloud/api/1";

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
            write!(f, "{} (No valid Auth Cookie found in headers)", self.message)
        }
    }
}

impl std::error::Error for AuthError {}


pub(crate) use crate::models::{AuthContext, VRCCurrentUser, VRCErrorResponse};

fn encode_vrchat_credentials(username: &str, password: &str) -> String {
    let encoded_username = utf8_percent_encode(username, NON_ALPHANUMERIC).to_string();
    let encoded_password = utf8_percent_encode(password, NON_ALPHANUMERIC).to_string();
    let combined = format!("{}:{}", encoded_username, encoded_password);
    BASE64_STANDARD.encode(combined)
}

fn extract_auth_cookie_value(
    response_headers: &reqwest::header::HeaderMap,
) -> Option<String> {
    for header_value in response_headers.get_all(SET_COOKIE) {
        if let Ok(cookie_str) = header_value.to_str() {
            if let Some(auth_part) = cookie_str
                .split(';')
                .map(|part| part.trim())
                .find(|part| part.starts_with("auth="))
            {
                return auth_part.strip_prefix("auth=")
                    .map(String::from)
                    .filter(|s| !s.is_empty());
            }
        }
    }
    None
}

pub async fn authenticate_with_vrchat_credentials(
    username: &str,
    password: &str,
) -> Result<AuthContext, AuthError> {
    let encoded_credentials = encode_vrchat_credentials(username, password);
    let auth_header_value = format!("Basic {}", encoded_credentials);
    let request_url = format!("{}/auth/user", VRCHAT_API_BASE_URL);

    log::info!("Attempting login for user: {}", username);

    let crate_name = env!("CARGO_PKG_NAME");
    let crate_version = env!("CARGO_PKG_VERSION");

    let client = Client::builder().build().map_err(|e| AuthError {
        message: e.to_string(),
        auth_cookie_value: None,
    })?;

    let request_builder = client
        .get(&request_url)
        .header(AUTHORIZATION, auth_header_value)
        .header(CONTENT_TYPE, "application/json;charset=utf-8")
        .header(USER_AGENT, format!("{} v{}", crate_name, crate_version));

    match request_builder.send().await {
        Ok(response) => {
            log::debug!("API response status: {}", response.status());
            log::debug!("API response headers: {:?}", response.headers());

            let response_status = response.status().as_u16();

            let response_headers_clone = response.headers().clone();

            let extracted_cookie_opt: Option<String> = extract_auth_cookie_value(&response_headers_clone);

            if response_status == 200 {
                let body_bytes= match response.bytes().await {
                    Ok(b) => b,
                    Err(e) => {
                        let base_msg = format!("Failed to read response body: {}", e);
                        log::error!("{:#?}", AuthError { message: base_msg.clone(), auth_cookie_value: extracted_cookie_opt.clone() });
                        return Err(AuthError {
                            message: base_msg,
                            auth_cookie_value: extracted_cookie_opt,
                        });
                    }
                };

                match serde_json::from_slice::<VRCCurrentUser>(&body_bytes) {
                    Ok(current_user) => {
                        log::info!("Login successful (status 200, user data parsed) for user: {}", username);
                        if let Some(auth_cookie_value) = extracted_cookie_opt {
                            log::info!("Valid auth cookie successfully extracted for user: {}.", username);
                            Ok(AuthContext {
                                user: current_user,
                                auth_cookie_value,
                            })
                        } else {
                            let base_msg = "Auth cookie not found or invalid in response headers despite 200 OK.";
                            log::error!("{:#?}", AuthError { message: base_msg.to_string(), auth_cookie_value: None }); // extracted_cookie_opt is None here
                            Err(AuthError {
                                message: base_msg.to_string(),
                                auth_cookie_value: None,
                            })
                        }
                    }
                    Err(e) => {
                        let body_str_for_log = String::from_utf8_lossy(&body_bytes);
                        let base_msg = format!("200 OK, but failed to parse user data: {}. Body was: '{}'", e, body_str_for_log);
                        log::error!("{:#?}", AuthError { message: base_msg.clone(), auth_cookie_value: extracted_cookie_opt.clone() });
                        Err(AuthError {
                            message: base_msg,
                            auth_cookie_value: extracted_cookie_opt,
                        })
                    }
                }
            } else {
                let error_body_bytes = response.bytes().await.unwrap_or_default();

                match serde_json::from_slice::<VRCErrorResponse>(&error_body_bytes) {
                    Ok(error_response) => {
                        let base_msg = format!(
                            "VRChat API error ({}): {}",
                            error_response.error.status_code, error_response.error.message
                        );
                        log::warn!("{:#?}", AuthError { message: base_msg.clone(), auth_cookie_value: extracted_cookie_opt.clone() });
                        Err(AuthError {
                            message: base_msg,
                            auth_cookie_value: extracted_cookie_opt,
                        })
                    }
                    Err(parse_err) => {
                        let body_str_for_log = String::from_utf8_lossy(&error_body_bytes);
                        let base_msg = format!(
                            "VRChat API returned status {} with non-JSON error body. Parse error: {}. Body: '{}'",
                            response_status, parse_err, body_str_for_log
                        );
                        log::warn!("{:#?}", AuthError { message: base_msg.clone(), auth_cookie_value: extracted_cookie_opt.clone() });
                        Err(AuthError {
                            message: base_msg,
                            auth_cookie_value: extracted_cookie_opt,
                        })
                    }
                }
            }
        }
        Err(http_error) => {
            let base_msg = format!("HTTP request error: {:?}", http_error);
            log::error!("{:#?} (Auth Cookie not available as the request itself failed)", AuthError{message: base_msg.clone(), auth_cookie_value: None});
            Err(AuthError {
                message: base_msg,
                auth_cookie_value: None,
            })
        }
    }
}