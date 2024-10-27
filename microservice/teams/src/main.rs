use axum::http::Method;
use axum::{
    routing::{get, post},
    Router,
};
use clap::Parser;

use log::info;
use microservices_shared::domain_events::{DomainEventConsumer, KafkaDomainEventProducer};
use rdkafka::producer::FutureProducer;
use rdkafka::util::get_rdkafka_version;
use rdkafka::ClientConfig;
use sqlx::PgPool;
use std::sync::Arc;
use teams::config::AppConfig;
use teams::ports::kafka::domain_events_handler::DomainEventCallbacksImpl;
use teams::ports::rest::teams::{
    create_team_handler, get_all_teams_handler, get_team_by_id_handler,
};
use teams::AppState;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    server_host: String,
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let args = Args::parse();
    let config = AppConfig::new_from_env();

    let connection_pool = PgPool::connect(&config.db_url).await.unwrap();

    let (version_n, version_s) = get_rdkafka_version();
    info!("rd_kafka_version: 0x{:08x}, {}", version_n, version_s);

    let kafka_producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", config.kafka_url.clone())
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Kafka producer creation error");

    let domain_event_producer =
        KafkaDomainEventProducer::new(kafka_producer, &config.kafka_domain_events_topic);
    let domain_event_callbacks = Box::new(DomainEventCallbacksImpl::new());
    let mut domain_event_consumer = DomainEventConsumer::new(
        &config.kafka_consumer_group,
        &config.kafka_url,
        &config.kafka_domain_events_topic,
        domain_event_callbacks,
    );

    let app_state = AppState {
        connection_pool,
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
        .route("/teams", post(create_team_handler))
        .route("/teams/:id", get(get_team_by_id_handler))
        .route("/teams/all", get(get_all_teams_handler))
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
