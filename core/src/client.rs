use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use reqwest::cookie::CookieStore;
use reqwest::header::HeaderValue;
use reqwest::Url;
use std::fs;
use std::io::Write;
use std::str::FromStr;
use std::sync::{LazyLock, RwLock};
use tokio::sync::Mutex;
use vrchatapi::apis::configuration::Configuration;

use crate::models::{CurrentSession, LoginCredentials}; // 引入 Write trait

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

    let cookie_store = std::sync::Arc::new(reqwest::cookie::Jar::default());
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
