#[derive(Debug, Clone)]
pub struct AppConfig {
    pub db_url: String,
    pub server_host: String,
}

impl AppConfig {
    pub fn new_from_env() -> Self {
        AppConfig {
            db_url: get_from_env_or_panic("DB_URL"),
            server_host: get_from_env_or_panic("HOST"),
        }
    }
}

pub fn get_from_env_or_panic(key: &str) -> String {
    std::env::var(key).unwrap_or_else(|err| panic!("Cannot find {} in env: {}", key, err))
}
