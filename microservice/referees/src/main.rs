use axum::http::Method;
use axum::Extension;
use axum::{
    routing::{get, post},
    Router,
};
use clap::Parser;

use microservices_shared::domain_events::DomainEventConsumer;
use opentelemetry::{
    trace::{Span, Tracer},
    KeyValue,
};
use referees::config::AppConfig;
use referees::ports::kafka::domain_events_handler::DomainEventCallbacksImpl;
use referees::ports::rest::referee::{
    create_referee_handler, get_all_referees_handler, get_referee_by_id_handler,
    update_referee_club_handler,
};
use referees::AppState;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    server_host: String,
    #[arg(short, long)]
    instance_id: String,
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let args = Args::parse();
    let config = AppConfig::new_from_env();
    let instance_id = Uuid::parse_str(&args.instance_id).unwrap();

    let tracer = microservices_shared::init_tracing(&config.otlp_endpoint, "referees");
    let mut span = tracer.start("application_start");
    span.set_attribute(KeyValue::new("server_host", args.server_host.clone()));

    let tracer_arc = Arc::new(tracer);

    let connection_pool = PgPool::connect(&config.db_url).await.unwrap();
    let redis_client = redis::Client::open(config.redis_url).unwrap();

    let redis_conn = redis_client.get_connection().unwrap();
    let domain_event_callbacks = Box::new(DomainEventCallbacksImpl::new(
        redis_conn,
        tracer_arc.clone(),
    ));
    let mut domain_event_consumer = DomainEventConsumer::new(
        &config.kafka_consumer_group,
        &config.kafka_url,
        &config.kafka_domain_events_topics,
        connection_pool.clone(),
        instance_id.clone(),
        domain_event_callbacks,
    );

    let app_state = AppState {
        connection_pool: connection_pool.clone(),
        redis_client: redis_client.clone(),
        tracer: tracer_arc,
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
        .layer(Extension(instance_id.clone()))
        .with_state(state_arc);

    let listener = tokio::net::TcpListener::bind(&args.server_host)
        .await
        .unwrap();

    tokio::spawn(async move {
        domain_event_consumer.run().await;
    });

    span.end();

    axum::serve(listener, app).await.unwrap();
}
