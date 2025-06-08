use crate::models::response::ApiResponse;
use std::sync::{LazyLock, RwLock};
pub use vrchatapi::apis::configuration::BasicAuth;
use vrchatapi::apis::configuration::Configuration;
use vrchatapi::apis::Error;

pub static GLOBAL_API_CLIENT: LazyLock<RwLock<VrcApiClient>> =
    LazyLock::new(|| RwLock::new(VrcApiClient::new()));

pub static GLOBAL_USER_AGENT: LazyLock<String> =
    LazyLock::new(|| format!("{}/{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")));

pub struct VrcApiClient {
    pub config: Configuration,
}

impl Default for VrcApiClient {
    fn default() -> Self {
        let mut config = Configuration::default();

        config.user_agent = Some(GLOBAL_USER_AGENT.clone());

        Self { config }
    }
}

impl VrcApiClient {
    pub fn new() -> Self {
        Self::default()
    }
}

pub fn create_error_response<T, E>(error: &Error<E>, base_message: &str) -> ApiResponse<T> {
    let (status_code, error_details) = match error {
        Error::ResponseError(response) => {
            let details = if let Ok(json_value) =
                serde_json::from_str::<serde_json::Value>(&response.content)
            {
                Some(json_value)
            } else {
                Some(serde_json::json!({
                    "raw_content": response.content,
                    "status": response.status.as_u16()
                }))
            };
            (response.status.as_u16(), details)
        }
        Error::Reqwest(reqwest_err) => {
            let status = reqwest_err.status().map(|s| s.as_u16()).unwrap_or(500);
            let details = Some(serde_json::json!({
                "type": "reqwest_error",
                "message": reqwest_err.to_string()
            }));
            (status, details)
        }
        _ => (
            500,
            Some(serde_json::json!({
                "type": "other_error",
                "message": format!("{}", error)
            })),
        ),
    };

    ApiResponse::error(
        status_code,
        format!("{}: {}", base_message, error),
        error_details,
    )
}
