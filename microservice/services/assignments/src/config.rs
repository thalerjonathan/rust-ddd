#[derive(Debug, Clone)]
pub struct AppConfig {
    pub server_host: String,
    pub db_url: String,
    pub redis_url: String,
    pub kafka_url: String,
    pub kafka_domain_events_topics: Vec<String>,
    pub kafka_consumer_group: String,
    pub otlp_endpoint: String,
}

impl AppConfig {
    pub fn new_from_env() -> Self {
        let kafka_domain_events_topics_str = get_from_env_or_panic("KAFKA_DOMAIN_EVENTS_TOPICS");
        let kafka_domain_events_topics: Vec<String> = kafka_domain_events_topics_str
            .split(",")
            .map(|str| str.to_string())
            .collect();

        AppConfig {
            server_host: get_from_env_or_panic("SERVER_HOST"),
            db_url: get_from_env_or_panic("DB_URL"),
            redis_url: get_from_env_or_panic("REDIS_URL"),
            kafka_url: get_from_env_or_panic("KAFKA_URL"),
            kafka_domain_events_topics,
            kafka_consumer_group: get_from_env_or_panic("KAFKA_CONSUMER_GROUP"),
            otlp_endpoint: get_from_env_or_panic("OTLP_ENDPOINT"),
        }
    }
}

pub fn get_from_env_or_panic(key: &str) -> String {
    std::env::var(key).unwrap_or_else(|err| panic!("Cannot find {} in env: {}", key, err))
}
