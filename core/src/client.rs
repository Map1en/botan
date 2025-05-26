use reqwest::header::{CONTENT_TYPE, COOKIE};
use reqwest::{Client as ReqwestClient, Method};
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::api_auth;
use crate::error::VRCError;
use crate::models::{VRCCurrentUser, VRCErrorResponse};

const PKG_NAME: &str = env!("CARGO_PKG_NAME");
const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

pub(crate) const VRCHAT_API_BASE_URL: &str = "https://api.vrchat.cloud/api/1";

pub(crate) fn get_default_user_agent() -> String {
    format!("{} v{}", PKG_NAME, PKG_VERSION)
}

pub struct VRCApiClient {
    http_client: ReqwestClient,
    auth_cookie_value: Option<String>,
    base_url: String,
}

impl VRCApiClient {
    pub fn new(
        base_url_override: Option<String>,
        user_agent_override: Option<String>,
    ) -> Result<Self, VRCError> {
        let http_client = ReqwestClient::builder()
            .user_agent(user_agent_override.unwrap_or_else(|| get_default_user_agent().to_string()))
            .cookie_store(false)
            .build()?;

        Ok(Self {
            http_client,
            auth_cookie_value: None,
            base_url: base_url_override.unwrap_or_else(|| VRCHAT_API_BASE_URL.to_string()),
        })
    }

    pub async fn login(username: &str, password: &str) -> Result<(Self, VRCCurrentUser), VRCError> {
        let login_http_client = ReqwestClient::builder()
            .user_agent(get_default_user_agent())
            .cookie_store(false)
            .build()?;

        let auth_context = api_auth::authenticate_with_vrchat_username_password(
            &login_http_client,
            username,
            password,
        )
        .await?;
        log::info!("VRCApiClient::login, AuthContext get");

        let main_http_client = ReqwestClient::builder()
            .user_agent(get_default_user_agent())
            .cookie_store(false)
            .build()?;

        let client_instance = Self {
            http_client: main_http_client,
            auth_cookie_value: Some(auth_context.auth_cookie_value),
            base_url: VRCHAT_API_BASE_URL.to_string(),
        };

        Ok((client_instance, auth_context.user))
    }

    pub fn is_authenticated(&self) -> bool {
        self.auth_cookie_value.is_some()
    }

    // pub fn get_auth_cookie_value(&self) -> Option<&str> {
    //     self.auth_cookie_value.as_deref()
    // }

    pub fn set_auth_cookie_value(&mut self, cookie_value: String) {
        self.auth_cookie_value = Some(cookie_value)
    }

    async fn send_request_generic<T: DeserializeOwned>(
        &self,
        method: Method,
        path: &str,
        json_body: Option<&impl Serialize>,
        requires_auth: bool,
    ) -> Result<T, VRCError> {
        if requires_auth && !self.is_authenticated() {
            return Err(VRCError::NotAuthenticated);
        }

        let url = format!("{}{}", self.base_url, path);
        log::debug!("Sending {} request to URL: {}", method, url);

        let mut request_builder = self.http_client.request(method, &url);

        if requires_auth {
            if let Some(cookie_value) = &self.auth_cookie_value {
                request_builder = request_builder.header(COOKIE, format!("auth={}", cookie_value))
            }
        }

        if let Some(body_data) = json_body {
            request_builder = request_builder
                .header(CONTENT_TYPE, "application/json;charset=utf-8")
                .json(body_data);
        }

        let response = request_builder.send().await?;
        log::debug!("Response status from {}: {}", url, response.status());

        let status = response.status();

        if !status.is_success() {
            let raw_body_text = response.text().await.ok();
            let message = format!("API request failed with status: {}", status);
            log::warn!("{}", message);

            if let Some(ref body_str) = raw_body_text {
                if let Ok(vrc_error) = serde_json::from_str::<VRCErrorResponse>(body_str) {
                    return Err(VRCError::ApiError {
                        status_code: vrc_error.error.status_code,
                        message: vrc_error.error.message,
                        raw_body: Some(body_str.clone()),
                    });
                }
            }

            return Err(VRCError::ApiError {
                status_code: status.as_u16(),
                message,
                raw_body: raw_body_text,
            });
        }

        let parsed_response = response.json::<T>().await?;
        Ok(parsed_response)
    }

    pub async fn get<T: DeserializeOwned>(
        &self,
        path: &str,
        requires_auth: bool,
    ) -> Result<T, VRCError> {
        self.send_request_generic(Method::GET, path, None::<&()>, requires_auth)
            .await
    }

    pub async fn post<T: DeserializeOwned, B: Serialize>(
        &self,
        path: &str,
        body: &B,
        requires_auth: bool,
    ) -> Result<T, VRCError> {
        self.send_request_generic(Method::POST, path, Some(body), requires_auth)
            .await
    }

    // placeholder for PUT and DELETE methods

    pub async fn auth_user(&self) -> Result<VRCCurrentUser, VRCError> {
        self.get::<VRCCurrentUser>("/auth/user", true).await
    }
}
