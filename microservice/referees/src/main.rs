use axum::async_trait;
use axum::http::Method;
use axum::{
    routing::{get, post},
    Router,
};
use clap::Parser;

use log::info;
use microservices_shared::domain_events::{
    DomainEventCallbacks, DomainEventConsumer, KafkaDomainEventProducer,
};
use microservices_shared::domain_ids::RefereeId;
use rdkafka::producer::FutureProducer;
use rdkafka::util::get_rdkafka_version;
use rdkafka::ClientConfig;
use redis::Commands;
use referees::config::AppConfig;
use referees::ports::rest::referee::{
    create_referee_handler, get_all_referees_handler, get_referee_by_id_handler,
    update_referee_club_handler,
};
use referees::AppState;
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    server_host: String,
}

struct DomainEventCallbacksImpl {
    redis_conn: redis::Connection,
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let args = Args::parse();
    let config = AppConfig::new_from_env();

    let connection_pool = PgPool::connect(&config.db_url).await.unwrap();
    let redis_client = redis::Client::open(config.redis_url).unwrap();

    let (version_n, version_s) = get_rdkafka_version();
    info!("rd_kafka_version: 0x{:08x}, {}", version_n, version_s);

    let kafka_producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", config.kafka_url.clone())
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Kafka producer creation error");

    let domain_event_producer =
        KafkaDomainEventProducer::new(kafka_producer, &config.kafka_domain_events_topic);

    let redis_conn = redis_client.get_connection().unwrap();
    let domain_event_callbacks = Box::new(DomainEventCallbacksImpl { redis_conn });
    let mut domain_event_consumer = DomainEventConsumer::new(
        &config.kafka_consumer_group,
        &config.kafka_url,
        &config.kafka_domain_events_topic,
        domain_event_callbacks,
    );

    let app_state = AppState {
        connection_pool,
        redis_client: redis_client.clone(),
        domain_event_publisher: Box::new(domain_event_producer),
    };
    let state_arc = Arc::new(app_state);

    let cors = tower_http::cors::CorsLayer::new()
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers(tower_http::cors::Any)
        .allow_origin(tower_http::cors::Any);

    let app = Router::new()
        .route("/referees", post(create_referee_handler))
        .route("/referees/:id", get(get_referee_by_id_handler))
        .route("/referees/all", get(get_all_referees_handler))
        .route("/referees/:id/club", post(update_referee_club_handler))
        .layer(cors)
        .with_state(state_arc);

    let listener = tokio::net::TcpListener::bind(&args.server_host)
        .await
        .unwrap();

    tokio::spawn(async move {
        domain_event_consumer.run().await;
    });

    axum::serve(listener, app).await.unwrap();
}

#[async_trait]
impl DomainEventCallbacks for DomainEventCallbacksImpl {
    async fn on_referee_created(&mut self, referee_id: RefereeId) {
        info!("Received Domain Event: Referee created: {:?}", referee_id);
    }

    async fn on_referee_club_changed(&mut self, referee_id: RefereeId, club_name: String) {
        info!(
            "Received Domain Event: Referee club changed: {:?} -> {}",
            referee_id, club_name
        );

        info!("Invalidating cache entry for referee: {:?}", referee_id);

        // NOTE: invalidate the cache entry for the referee
        let key = format!("referee_{}", referee_id.0.to_string());
        let _result: Result<(), redis::RedisError> = self.redis_conn.del(key);
    }
}
