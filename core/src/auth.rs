use vrchatapi::apis::authentication_api::{get_current_user, GetCurrentUserError};
pub use vrchatapi::apis::configuration::BasicAuth;
use vrchatapi::apis::{configuration, Error};

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
