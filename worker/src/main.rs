use botan_core::models::{EitherTwoFactorAuthCodeType, LoginCredentials};
use botan_core::vrchatapi_models::{EitherUserOrTwoFactor, TwoFactorAuthCode, TwoFactorEmailCode};
use botan_core::{auth, database}; // 新增 database
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    rustls::crypto::ring::default_provider()
        .install_default()
        .expect("Failed to install rustls crypto provider");

    if let Err(e) = database::init_database().await {
        log::error!("Failed to initialize database: {}", e);
        std::process::exit(1);
    }
    println!("Database initialized successfully");

    let username = std::env::var("USERNAME").expect("NO USERNAME");
    let password = std::env::var("PASSWORD").expect("NO PASSWORD");
    let credentials = Some(LoginCredentials {
        username,
        password: Some(password),
        auto_login_user_id: None,
    });

    let auth_result = authenticate(&credentials).await;
    if !auth_result {
        log::error!("Authentication failed");
        std::process::exit(1);
    }

    // waiting
    wait_for_shutdown().await;

    println!("Application shutdown complete");
}

async fn authenticate(credentials: &Option<LoginCredentials>) -> bool {
    match auth::auth_login_and_get_current_user(credentials, &Some(true)).await {
        api_response if api_response.success => match api_response.data {
            Some(EitherUserOrTwoFactor::CurrentUser(user)) => {
                log::info!("Login successful: {}", user.display_name);
                true
            }
            Some(EitherUserOrTwoFactor::RequiresTwoFactorAuth(_)) => handle_2fa(credentials).await,
            None => {
                log::error!("No user data returned");
                false
            }
        },
        api_response => {
            log::error!("Authentication failed: {}", api_response.message);
            false
        }
    }
}

async fn handle_2fa(credentials: &Option<LoginCredentials>) -> bool {
    if let Ok(two_fa_code_str) = std::env::var("VRC_2FA_CODE") {
        let two_fa_type_str = std::env::var("VRC_2FA_TYPE").unwrap_or("2fa".to_string());
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
                match auth::auth_login_and_get_current_user(credentials, &Some(false)).await {
                    login_response if login_response.success => {
                        if let Some(EitherUserOrTwoFactor::CurrentUser(user)) = login_response.data
                        {
                            log::info!("Login successful after 2FA: {}", user.display_name);
                            return true;
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
    log::error!("2FA authentication failed");
    false
}

async fn wait_for_shutdown() {
    println!("Service is running. Waiting for shutdown signal...");

    tokio::select! {
        _ = tokio::signal::ctrl_c() => {
            // println!("Received Ctrl+C signal");
        }
        _ = wait_for_system_signals() => {
            println!("Received system shutdown signal");
        }
    }
}

async fn wait_for_system_signals() {
    #[cfg(unix)]
    {
        use tokio::signal::unix::{signal, SignalKind};
        let mut sigterm =
            signal(SignalKind::terminate()).expect("Failed to create SIGTERM handler");
        sigterm.recv().await;
    }

    // #[cfg(windows)]
    // {
    //     std::future::pending::<()>().await;
    // }
}
