use crate::client::{create_error_response, GLOBAL_API_CLIENT};
use crate::models::response::ApiResponse;
use crate::models::{EitherTwoFactorAuthCodeType, LoginCredentials, TwoFactorVerifyResult};
use crate::pipeline;
use std::sync::LazyLock;
use tokio::sync::RwLock;
use vrchatapi::apis::authentication_api::{get_current_user, VerifyAuthTokenError};
pub use vrchatapi::apis::configuration::BasicAuth;
use vrchatapi::apis::Error;

pub static GLOBAL_PIPELINE_MANAGER: LazyLock<RwLock<Option<pipeline::PipelineManager>>> =
    LazyLock::new(|| RwLock::new(None));

pub async fn auth_login_and_get_current_user(
    credentials: &Option<LoginCredentials>,
    is_first_login: &Option<bool>,
) -> ApiResponse<vrchatapi::models::EitherUserOrTwoFactor> {
    let cookie_store_arc = if let Some(true) = is_first_login {
        let cookie_store = {
            if let Ok(file) = std::fs::File::open("cookies.json") {
                let reader = std::io::BufReader::new(file);
                serde_json::from_reader(reader)
                    .unwrap_or_else(|_| reqwest_cookie_store::CookieStore::new(None))
            } else {
                reqwest_cookie_store::CookieStore::new(None)
            }
        };
        let cookie_store_arc =
            std::sync::Arc::new(reqwest_cookie_store::CookieStoreMutex::new(cookie_store));

        let mut global_client = GLOBAL_API_CLIENT.write().unwrap();
        if let Some(creds) = credentials {
            global_client.config.basic_auth =
                Some((creds.username.clone(), creds.password.clone()));
            log::info!("Updated basic auth for user: {}", creds.username);
        }
        global_client.config.client = reqwest::Client::builder()
            .cookie_provider(cookie_store_arc.clone())
            .build()
            .unwrap();

        Some(cookie_store_arc)
    } else {
        None
    };

    loop {
        let client_config = {
            let client = GLOBAL_API_CLIENT.read().unwrap();
            client.config.clone()
        };

        match get_current_user(&client_config).await {
            Ok(user_or_2fa) => match &user_or_2fa {
                vrchatapi::models::EitherUserOrTwoFactor::CurrentUser(current_user) => {
                    println!("Login successful for user: {}", current_user.display_name);
                    if let Some(cookie_store) = &cookie_store_arc {
                        let mut writer = std::fs::File::create("cookies.json")
                            .map(std::io::BufWriter::new)
                            .unwrap();
                        let store = cookie_store.lock().unwrap();
                        serde_json::to_writer(&mut writer, &*store).unwrap();
                        log::info!("Cookies saved successfully.");
                    };
                    match pipeline_auth().await {
                        Ok(token) => {
                            println!("token result: {:?}", token.clone());

                            let mut manager = pipeline::PipelineManager::new(token.token.clone());
                            manager.start().await;

                            {
                                let mut global_manager = GLOBAL_PIPELINE_MANAGER.write().await;
                                *global_manager = Some(manager);
                            }

                            println!("Pipeline service started");
                        }
                        Err(e) => {
                            log::error!("Pipeline auth failed: {:?}", e);
                        }
                    }

                    return ApiResponse::success(user_or_2fa, Some("Login successful".to_string()));
                }
                vrchatapi::models::EitherUserOrTwoFactor::RequiresTwoFactorAuth(u) => {
                    log::info!("2FA required: {:?}", u);
                    println!("Please enter your 2FA code:");

                    let mut guess = String::new();
                    if std::io::stdin().read_line(&mut guess).is_err() {
                        return ApiResponse::simple_error(
                            500,
                            "Failed to read 2FA code from stdin".to_string(),
                        );
                    }

                    let verify_result = auth_verify2_fa(
                        "2fa",
                        EitherTwoFactorAuthCodeType::IsA(vrchatapi::models::TwoFactorAuthCode {
                            code: guess.trim().to_string(),
                        }),
                    )
                    .await;

                    if verify_result.success {
                        log::info!("2FA verification successful. Retrying login...");
                        continue;
                    } else {
                        log::error!("2FA verification failed: {:?}", verify_result);
                        return ApiResponse::simple_error(
                            401,
                            "2FA verification failed".to_string(),
                        );
                    }
                }
            },
            Err(e) => {
                log::error!("Failed to login: {}", e);
                return create_error_response(&e, "Failed to login");
            }
        }
    }
}

pub async fn auth_verify2_fa(
    two_fa_type: &str,
    code: EitherTwoFactorAuthCodeType,
) -> ApiResponse<TwoFactorVerifyResult> {
    let client_config = {
        let client = GLOBAL_API_CLIENT.read().unwrap();
        client.config.clone()
    };

    match two_fa_type {
        "2fa" => {
            log::info!("aff{:?}", code);
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

pub async fn pipeline_auth(
) -> Result<vrchatapi::models::VerifyAuthTokenResult, Error<VerifyAuthTokenError>> {
    let client_config = {
        let client = GLOBAL_API_CLIENT.read().unwrap();
        client.config.clone()
    };

    match vrchatapi::apis::authentication_api::verify_auth_token(&client_config).await {
        Ok(result) => Ok(result),
        Err(e) => {
            log::error!("pipeline_auth failed: {:?}", e);
            Err(e)
        }
    }
}
