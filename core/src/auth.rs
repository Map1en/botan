use crate::client::initialize_client_with_cookies;
use crate::models::{EitherTwoFactorAuthCodeType, EitherTwoFactorResultType};
use vrchatapi::apis::authentication_api::{
    get_current_user, GetCurrentUserError, VerifyAuthTokenError,
};
pub use vrchatapi::apis::configuration::BasicAuth;
use vrchatapi::apis::{Error, ResponseContent};

use crate::client::GLOBAL_API_CLIENT;
use crate::models::LoginCredentials;

pub async fn auth_and_get_current_user(
    credentials: &Option<LoginCredentials>,
    cookies_path: &str,
) -> Result<vrchatapi::models::EitherUserOrTwoFactor, Error<GetCurrentUserError>> {
    initialize_client_with_cookies(credentials, cookies_path)
        .await
        .map_err(|e| {
            Error::ResponseError(ResponseContent {
                status: reqwest::StatusCode::INTERNAL_SERVER_ERROR,
                content: e,
                entity: None,
            })
        })?;

    let client_config = {
        let client = GLOBAL_API_CLIENT.read().unwrap();
        client.config.clone()
    };

    match get_current_user(&client_config).await {
        Ok(user) => {
            verify_auth().await.map_err(|e| {
                Error::ResponseError(ResponseContent {
                    status: reqwest::StatusCode::INTERNAL_SERVER_ERROR,
                    content: e.to_string(),
                    entity: None,
                })
            })?;
            Ok(user)
        }
        Err(e) => {
            eprintln!("Failed to login: {}", e);
            log::debug!("Failed to login: {}", e);
            Err(e)
        }
    }
}

pub async fn verify2_fa(
    two_fa_type: &str,
    code: EitherTwoFactorAuthCodeType,
) -> Result<EitherTwoFactorResultType, String> {
    let client_config = {
        let client = GLOBAL_API_CLIENT.read().unwrap();
        client.config.clone()
    };

    match two_fa_type {
        "2fa" => {
            if let EitherTwoFactorAuthCodeType::IsA(auth_code) = code {
                let result = vrchatapi::apis::authentication_api::verify2_fa(
                    &client_config,
                    auth_code.clone(),
                )
                .await;
                match result {
                    Ok(res) => Ok(EitherTwoFactorResultType::IsA(res)),
                    Err(e) => {
                        log::error!("Failed to verify 2FA auth code: {:?}", e);
                        Err(serde_json::to_string(&format!(
                            "Failed to verify 2FA auth code: {:?}",
                            e
                        ))
                        .unwrap())
                    }
                }
            } else {
                Err("Invalid code type for auth verification".to_string())
            }
        }
        "email" => {
            if let EitherTwoFactorAuthCodeType::IsB(email_code) = code {
                let result = vrchatapi::apis::authentication_api::verify2_fa_email_code(
                    &client_config,
                    email_code.clone(),
                )
                .await;
                match result {
                    Ok(res) => Ok(EitherTwoFactorResultType::IsB(res)),
                    Err(e) => {
                        log::error!("Failed to verify 2FA email code: {:?}", e);
                        Err(serde_json::to_string(&format!(
                            "Failed to verify 2FA email code: {:?}",
                            e
                        ))
                        .unwrap())
                    }
                }
            } else {
                Err("Invalid code type for email verification".to_string())
            }
        }
        _ => Err("Unknown two-factor type".to_string()),
    }
}

pub async fn verify_auth(
) -> Result<vrchatapi::models::VerifyAuthTokenResult, Error<VerifyAuthTokenError>> {
    let client_config = {
        let client = GLOBAL_API_CLIENT.read().unwrap();
        client.config.clone()
    };

    match vrchatapi::apis::authentication_api::verify_auth_token(&client_config).await {
        Ok(result) => Ok(result),
        Err(e) => {
            log::error!("Failed to verify auth token: {:?}", e);
            Err(e)
        }
    }
}
