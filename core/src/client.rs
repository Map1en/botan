use vrchatapi::apis::configuration::Configuration;

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
