use reqwest::Error as ReqwestError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum VRCError {
    #[error("Network error: {0}")]
    NetworkError(#[from] ReqwestError),

    #[error("Json parse error: {0}")]
    JsonParseError(serde_json::Error),

    #[error("API error: {status_code} - {message} {raw_body:?}")]
    ApiError {
        status_code: u16,
        message: String,
        raw_body: Option<String>,
    },

    #[error("Authentication error: {0}")]
    AuthenticationFailed(String),

    #[error("Parse cookie error: {0}")]
    CookieExtractionFailed(String),

    #[error("Not authenticated")]
    NotAuthenticated,

    #[error("InvalidInput: {0}")]
    InvalidInput(String),

    #[error("Internal error: {0}")]
    InternalError(String),
}
