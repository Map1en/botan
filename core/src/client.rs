use crate::models::response::ApiResponse;
use crate::models::{CurrentSession, LoginCredentials};
use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use reqwest::cookie::CookieStore;
use reqwest::header::HeaderValue;
use reqwest::Url;
use std::fs;
use std::io::Write;
use std::str::FromStr;
use std::sync::{LazyLock, RwLock};
use tokio::sync::Mutex;
pub use vrchatapi::apis::configuration::BasicAuth;
use vrchatapi::apis::configuration::Configuration;
use vrchatapi::apis::Error;

static CURRENT_SESSION: LazyLock<Mutex<Option<CurrentSession>>> =
    LazyLock::new(|| Mutex::new(None));

pub static GLOBAL_API_CLIENT: LazyLock<RwLock<VrcApiClient>> =
    LazyLock::new(|| RwLock::new(VrcApiClient::new()));

pub struct VrcApiClient {
    pub config: Configuration,
}

impl VrcApiClient {
    pub fn new() -> Self {
        let mut config = Configuration::default();
        const PKG_NAME: &str = env!("CARGO_PKG_NAME");
        const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

        pub(crate) fn get_default_user_agent() -> String {
            format!("{}/{}", PKG_NAME, PKG_VERSION)
        }
        config.user_agent = Some(get_default_user_agent());

        Self { config }
    }
}

pub async fn initialize_client_with_cookies(
    credentials: &Option<LoginCredentials>,
    cookies_path: &str,
    cookie_store: std::sync::Arc<reqwest::cookie::Jar>,
) -> Result<(), String> {
    let new_username = credentials.as_ref().map(|c| c.username.clone());

    let needs_reinit = {
        let current_session = CURRENT_SESSION.lock().await;
        match &*current_session {
            None => true,
            Some(session) => session.username != new_username,
        }
    };

    if !needs_reinit {
        log::info!("Client already initialized for current user, skipping");
        return Ok(());
    }

    let cookie_store = cookie_store;
    let url = Url::from_str("https://api.vrchat.cloud").expect("Invalid URL");

    match load_and_decrypt_data(cookies_path) {
        Ok(cookies) => {
            log::info!("cookie text: {}", cookies);
            let header_value = HeaderValue::from_str(&cookies).expect("Invalid cookie format");
            cookie_store.set_cookies(&mut std::iter::once(&header_value), &url);
            log::info!("Cookies loaded from store, proceeding with cookies");
        }
        Err(_) => {
            log::warn!("No cookies found in store, proceeding without cookies");
        }
    }

    {
        let mut global_client = GLOBAL_API_CLIENT.write().unwrap();

        if let Some(creds) = credentials {
            global_client.config.basic_auth =
                Some((creds.username.clone(), Some(creds.password.clone())));
        }

        global_client.config.client = reqwest::Client::builder()
            .cookie_provider(cookie_store)
            .build()
            .unwrap();
    }

    {
        let mut current_session = CURRENT_SESSION.lock().await;
        *current_session = Some(CurrentSession {
            username: new_username,
            cookies_path: cookies_path.to_string(),
        });
    }

    Ok(())
}

pub fn save_cookies_from_jar(
    cookie_store: &std::sync::Arc<reqwest::cookie::Jar>,
    cookies_path: &str,
) -> Result<(), String> {
    let url = reqwest::Url::parse("https://api.vrchat.cloud").unwrap();

    let cookies: Vec<String> = cookie_store
        .cookies(&url)
        .iter()
        .filter_map(|cookie| cookie.to_str().ok().map(|s| s.to_string()))
        .collect();

    if !cookies.is_empty() {
        let cookie_string = cookies.join("; ");

        encrypt_and_save_data(&cookie_string, cookies_path)
            .map_err(|e| format!("Failed to encrypt and save cookies: {}", e))?;

        log::info!("Cookies saved successfully to: {}", cookies_path);
    } else {
        log::warn!("No cookies found to save");
    }

    Ok(())
}

static SALT: &str = "fake_salt";

pub fn encrypt_and_save_data(data: &str, filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mc = new_magic_crypt!(SALT, 256);

    let encrypted_string = mc.encrypt_str_to_base64(data);
    let mut file = fs::File::create(filepath)?;
    file.write_all(encrypted_string.as_bytes())?;

    println!("save cookie to {}", filepath);
    Ok(())
}

pub fn load_and_decrypt_data(filepath: &str) -> Result<String, Box<dyn std::error::Error>> {
    let encrypted_string = fs::read_to_string(filepath)?;

    let mc = new_magic_crypt!(SALT, 256);
    let decrypted_string = mc.decrypt_base64_to_string(&encrypted_string)?;

    println!("load cookie from {}", filepath);
    Ok(decrypted_string)
}

pub fn create_error_response<T, E>(error: &Error<E>, base_message: &str) -> ApiResponse<T> {
    let (status_code, error_details) = match error {
        Error::ResponseError(response) => {
            let details = if let Ok(json_value) =
                serde_json::from_str::<serde_json::Value>(&response.content)
            {
                Some(json_value)
            } else {
                Some(serde_json::json!({
                    "raw_content": response.content,
                    "status": response.status.as_u16()
                }))
            };
            (response.status.as_u16(), details)
        }
        Error::Reqwest(reqwest_err) => {
            let status = reqwest_err.status().map(|s| s.as_u16()).unwrap_or(500);
            let details = Some(serde_json::json!({
                "type": "reqwest_error",
                "message": reqwest_err.to_string()
            }));
            (status, details)
        }
        _ => (
            500,
            Some(serde_json::json!({
                "type": "other_error",
                "message": format!("{}", error)
            })),
        ),
    };

    ApiResponse::error(
        status_code,
        format!("{}: {}", base_message, error),
        error_details,
    )
}
