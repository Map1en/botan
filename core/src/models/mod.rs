pub mod response;

use serde::{Deserialize, Serialize};
use vrchatapi::models::{
    TwoFactorAuthCode, TwoFactorEmailCode, Verify2FaEmailCodeResult, Verify2FaResult,
};
#[derive(Deserialize, Debug, Clone)]
pub struct LoginCredentials {
    pub username: String,
    pub password: Option<String>,
    pub auto_login_user_id: Option<String>,
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
