#[derive(Debug, Clone)]
pub struct AppConfig {
    pub db_url: String,
    pub redis_url: String,
    pub kafka_url: String,
    pub kafka_domain_events_topic: String,
    pub kafka_consumer_group: String,
}

impl AppConfig {
    pub fn new_from_env() -> Self {
        AppConfig {
            db_url: get_from_env_or_panic("DB_URL"),
            redis_url: get_from_env_or_panic("REDIS_URL"),
            kafka_url: get_from_env_or_panic("KAFKA_URL"),
            kafka_domain_events_topic: get_from_env_or_panic("KAFKA_DOMAIN_EVENTS_TOPIC"),
            kafka_consumer_group: get_from_env_or_panic("KAFKA_CONSUMER_GROUP"),
        }
    }
}

pub fn get_from_env_or_panic(key: &str) -> String {
    std::env::var(key).unwrap_or_else(|err| panic!("Cannot find {} in env: {}", key, err))
}
