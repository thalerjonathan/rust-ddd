use assignments::config::AppConfig;
use assignments::ports::rest::assignments::{
    commit_assignments_handler, fetch_assignments_handler, remove_committed_assignment_handler,
    remove_staged_assignment_handler, stage_assignment_handler, validate_assignments_handler,
};

use assignments::AppState;
use axum::http::Method;
use axum::routing::{delete, put};
use axum::{
    routing::{get, post},
    Router,
};
use clap::Parser;

use microservices_shared::domain_events::{
    DomainEventCallbacksLoggerImpl, DomainEventConsumer, KafkaDomainEventProducer,
};
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    server_host: String,
    #[arg(short, long)]
    kafka_tx_id: String,
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let args = Args::parse();
    let config = AppConfig::new_from_env();

    let connection_pool = PgPool::connect(&config.db_url).await.unwrap();
    let redis_client = redis::Client::open(config.redis_url).unwrap();

    let domain_event_producer = KafkaDomainEventProducer::new(
        &config.kafka_url,
        &config.kafka_domain_events_topic,
        &args.kafka_tx_id,
    );
    let domain_event_callbacks = Box::new(DomainEventCallbacksLoggerImpl::new());
    let mut domain_event_consumer = DomainEventConsumer::new(
        &config.kafka_consumer_group,
        &config.kafka_url,
        &config.kafka_domain_events_topic,
        domain_event_callbacks,
    );

    let app_state = AppState {
        connection_pool,
        redis_client,
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
        .route("/assignments", get(fetch_assignments_handler))
        .route("/assignments", put(stage_assignment_handler))
        .route(
            "/assignments/staged/:fixture_id/:referee_id",
            delete(remove_staged_assignment_handler),
        )
        .route(
            "/assignments/committed/:fixture_id/:referee_id",
            delete(remove_committed_assignment_handler),
        )
        .route("/assignments/validate", post(validate_assignments_handler))
        .route("/assignments/commit", post(commit_assignments_handler))
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
