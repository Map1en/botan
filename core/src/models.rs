use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone)]
pub struct LoginCrendentials {
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
    pub id: String,
    pub username: String,
    pub display_name: String,
    pub bio: Option<String>,
    pub current_avatar_thumbnail_image_url: Option<String>,
    pub status: Option<String>,
    pub last_login: Option<String>, // ISO 8601 DataTime string
    pub email_verified: Option<bool>,
    // login: "emailOtp", "?"(2FA), null
    pub requires_two_factor_auth: Option<String>,
    // ... more fields, not all will be used, just write these for now.
}

#[derive(Deserialize, Debug, Clone)]
pub struct AuthContext {
    pub user: VRCCurrentUser,
    pub auth_cookie_value: String,
}
