#[derive(Debug, Clone)]
pub struct AppConfig {
    pub server_host: String,
    pub idp_host: String,
    pub idp_realm: String,
    pub client_id: String,
    pub redis_url: String,
    pub otlp_endpoint: String,
}

impl AppConfig {
    pub fn new_from_env() -> Self {
        AppConfig {
            server_host: get_from_env_or_panic("SERVER_HOST"),
            redis_url: get_from_env_or_panic("REDIS_URL"),
            idp_host: get_from_env_or_panic("IDP_HOST"),
            idp_realm: get_from_env_or_panic("IDP_REALM"),
            client_id: get_from_env_or_panic("CLIENT_ID"),
            otlp_endpoint: get_from_env_or_panic("OTLP_ENDPOINT"),
        }
    }
}

pub fn get_from_env_or_panic(key: &str) -> String {
    std::env::var(key).unwrap_or_else(|err| panic!("Cannot find {} in env: {}", key, err))
}
