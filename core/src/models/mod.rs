pub mod response;

use serde::{Deserialize, Serialize};
use vrchatapi::models::{
    TwoFactorAuthCode, TwoFactorEmailCode, Verify2FaEmailCodeResult, Verify2FaResult,
};
#[derive(Deserialize, Debug, Clone)]
pub struct LoginCredentials {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct VRCErrorDetail {
    pub message: String,
    pub status_code: u16,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct VRCErrorResponse {
    pub error: VRCErrorDetail,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VRCCurrentUser {
    pub id: Option<String>,
    pub username: Option<String>,
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub current_avatar_thumbnail_image_url: Option<String>,
    pub status: Option<String>,
    pub last_login: Option<String>, // ISO 8601 DataTime string
    pub email_verified: Option<bool>,
    // login: ["emailOtp"], "?"(2FA), null
    pub requires_two_factor_auth: Vec<String>,
    // ... more fields, not all will be used, just write these for now.
}

#[derive(Deserialize, Debug, Clone)]
pub struct AuthContext {
    pub user: VRCCurrentUser,
    pub auth_cookie_value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EitherTwoFactorAuthCodeType {
    IsA(TwoFactorAuthCode),
    IsB(TwoFactorEmailCode),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EitherTwoFactorResultType {
    IsA(Verify2FaResult),
    IsB(Verify2FaEmailCodeResult),
}

#[derive(Debug, Clone, PartialEq)]
pub struct CurrentSession {
    pub username: Option<String>,
    pub cookies_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TwoFactorVerifyResult {
    pub verified: bool,
    pub enabled: Option<bool>,
}

impl From<Verify2FaResult> for TwoFactorVerifyResult {
    fn from(result: Verify2FaResult) -> Self {
        Self {
            verified: result.verified,
            enabled: result.enabled,
        }
    }
}

impl From<Verify2FaEmailCodeResult> for TwoFactorVerifyResult {
    fn from(result: Verify2FaEmailCodeResult) -> Self {
        Self {
            verified: result.verified,
            enabled: None,
        }
    }
}
