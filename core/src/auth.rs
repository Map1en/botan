use crate::client::{
    create_error_response, initialize_client_with_cookies, save_cookies_from_jar, GLOBAL_API_CLIENT,
};
use crate::models::response::ApiResponse;
use crate::models::{EitherTwoFactorAuthCodeType, LoginCredentials, TwoFactorVerifyResult};
use vrchatapi::apis::authentication_api::{get_current_user, VerifyAuthTokenError};
pub use vrchatapi::apis::configuration::BasicAuth;
use vrchatapi::apis::Error;

pub async fn auth_and_get_current_user(
    credentials: &Option<LoginCredentials>,
    cookies_path: Option<String>,
) -> ApiResponse<vrchatapi::models::EitherUserOrTwoFactor> {
    let cookie_store = std::sync::Arc::new(reqwest::cookie::Jar::default());
    if let Err(e) =
        initialize_client_with_cookies(credentials, cookies_path.clone(), cookie_store.clone())
            .await
    {
        return ApiResponse::simple_error(500, format!("Client initialization failed: {}", e));
    }
    let client_config = {
        let client = GLOBAL_API_CLIENT.read().unwrap();
        client.config.clone()
    };

    match get_current_user(&client_config).await {
        Ok(user) => {
            if let Err(e) = save_cookies_from_jar(&cookie_store, cookies_path.clone()) {
                log::error!("Failed to save cookies: {}", e);
            } else {
                log::info!("Cookies saved successfully after login");
            }
            match &user {
                vrchatapi::models::EitherUserOrTwoFactor::CurrentUser(current_user) => {
                    log::info!("Login successful for user: {}", current_user.display_name);
                }
                vrchatapi::models::EitherUserOrTwoFactor::RequiresTwoFactorAuth(_) => {
                    log::info!("2FA required, not saving cookies yet");
                }
            }

            ApiResponse::success(user, None)
        }
        Err(e) => {
            log::info!("Failed to login: {}", e);
            create_error_response(&e, "Failed to login")
        }
    }
}

pub async fn verify2_fa(
    two_fa_type: &str,
    code: EitherTwoFactorAuthCodeType,
) -> ApiResponse<TwoFactorVerifyResult> {
    let client_config = {
        let client = GLOBAL_API_CLIENT.read().unwrap();
        client.config.clone()
    };

    match two_fa_type {
        "2fa" => {
            if let EitherTwoFactorAuthCodeType::IsA(auth_code) = code {
                match vrchatapi::apis::authentication_api::verify2_fa(
                    &client_config,
                    auth_code.clone(),
                )
                .await
                {
                    Ok(res) => {
                        log::info!("2FA verification successful: {:?}", res);
                        ApiResponse::success(
                            TwoFactorVerifyResult::from(res),
                            Some("2FA verification successful".to_string()),
                        )
                    }
                    Err(e) => {
                        log::error!("Failed to verify 2FA auth code: {:?}", e);
                        create_error_response(&e, "Failed to verify 2FA auth code")
                    }
                }
            } else {
                ApiResponse::simple_error(
                    400,
                    "Invalid code type for auth verification".to_string(),
                )
            }
        }
        "email" => {
            if let EitherTwoFactorAuthCodeType::IsB(email_code) = code {
                match vrchatapi::apis::authentication_api::verify2_fa_email_code(
                    &client_config,
                    email_code.clone(),
                )
                .await
                {
                    Ok(res) => ApiResponse::success(
                        TwoFactorVerifyResult::from(res),
                        Some("Email 2FA verification successful".to_string()),
                    ),
                    Err(e) => {
                        log::error!("Failed to verify 2FA email code: {:?}", e);
                        create_error_response(&e, "Failed to verify 2FA email code")
                    }
                }
            } else {
                ApiResponse::simple_error(
                    400,
                    "Invalid code type for email verification".to_string(),
                )
            }
        }
        _ => ApiResponse::simple_error(400, "Unknown two-factor type".to_string()),
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
