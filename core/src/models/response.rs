use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub status: u16,
    pub success: bool,
    pub data: Option<T>,
    pub message: String,
    pub error_details: Option<serde_json::Value>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T, message: Option<String>) -> Self {
        Self {
            status: 200,
            success: true,
            data: Some(data),
            message: message.unwrap_or_else(|| "Success".to_string()),
            error_details: None,
        }
    }

    pub fn error(status: u16, message: String, details: Option<serde_json::Value>) -> Self {
        Self {
            status,
            success: false,
            data: None,
            message,
            error_details: details,
        }
    }

    pub fn simple_error(status: u16, message: String) -> Self {
        Self::error(status, message, None)
    }
}
