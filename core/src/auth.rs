use vrchatapi::apis::authentication_api::{
    verify2_fa_email_code, Verify2FaEmailCodeError, Verify2FaError,
};
pub use vrchatapi::apis::configuration::BasicAuth;
use vrchatapi::apis::{configuration, Error};
use vrchatapi::models::{TwoFactorEmailCode, Verify2FaEmailCodeResult};
use vrchatapi::{
    apis::authentication_api::{get_current_user, verify2_fa, GetCurrentUserError},
    models::{TwoFactorAuthCode, Verify2FaResult},
};

pub async fn auth_get_current_user(
    client: &configuration::Configuration,
) -> Result<vrchatapi::models::EitherUserOrTwoFactor, Error<GetCurrentUserError>> {
    match get_current_user(client).await {
        Ok(user) => Ok(user),
        Err(e) => {
            eprintln!("Failed to login: {}", e);
            log::debug!("Failed to login: {}", e);
            Err(e)
        }
    }
}

pub async fn auth_verify2_fa(
    client: &configuration::Configuration,
    code: TwoFactorAuthCode,
) -> Result<Verify2FaResult, Error<Verify2FaError>> {
    match verify2_fa(client, code).await {
        Ok(result) => Ok(result),
        Err(e) => {
            eprintln!("Failed to verify 2FA: {}", e);
            log::debug!("Failed to verify 2FA: {}", e);
            Err(e)
        }
    }
}

pub async fn auth_verify2_fa_email(
    client: &configuration::Configuration,
    code: TwoFactorEmailCode,
) -> Result<Verify2FaEmailCodeResult, Error<Verify2FaEmailCodeError>> {
    match verify2_fa_email_code(client, code).await {
        Ok(result) => Ok(result),
        Err(e) => {
            eprintln!("Failed to verify 2FA email code: {}", e);
            log::debug!("Failed to verify 2FA email code: {}", e);
            Err(e)
        }
    }
}
