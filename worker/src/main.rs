use botan_core::auth;
use botan_core::models::{EitherTwoFactorAuthCodeType, LoginCredentials};
use botan_core::vrchatapi_models::{EitherUserOrTwoFactor, TwoFactorAuthCode, TwoFactorEmailCode};

#[tokio::main]
async fn main() {
    env_logger::init();

    let username = std::env::var("USERNAME").expect("NO USERNAME");
    let password = std::env::var("PASSWORD").expect("NO PASSWORD");
    let credentials = Some(LoginCredentials {
        username,
        password: Some(password),
        auto_login_user_id: None,
    });

    match auth::auth_login_and_get_current_user(&credentials).await {
        //
        api_response if api_response.success => match api_response.data {
            Some(EitherUserOrTwoFactor::CurrentUser(user)) => {
                log::info!("CurrentUser: {}", user.display_name);
            }
            Some(EitherUserOrTwoFactor::RequiresTwoFactorAuth(two_factor_data)) => {
                log::info!("2fa type: {:?}", two_factor_data.requires_two_factor_auth);
                if let Ok(two_fa_code_str) = std::env::var("VRC_2FA_CODE") {
                    log::info!("2fa from env");
                    let two_fa_type_str =
                        std::env::var("VRC_2FA_TYPE").unwrap_or("2fa".to_string());

                    let code_to_verify = if two_fa_type_str == "email" {
                        EitherTwoFactorAuthCodeType::IsB(TwoFactorEmailCode {
                            code: two_fa_code_str,
                        })
                    } else {
                        EitherTwoFactorAuthCodeType::IsA(TwoFactorAuthCode {
                            code: two_fa_code_str,
                        })
                    };

                    match auth::auth_verify2_fa(&two_fa_type_str, code_to_verify).await {
                        verify_response if verify_response.success => {
                            log::info!("2FA.");
                        }
                        verify_response => {
                            log::error!("2FA failed: {}", verify_response.message);
                        }
                    }
                } else {
                    log::error!("no 2fa code");
                }
            }
            None => {
                log::error!("login failed, no data");
            }
        },
        api_response => {
            log::error!("login failed, no res:{}", api_response.message);
        }
    }
}
