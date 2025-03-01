use axum::http::Method;
use axum::{
    routing::{get, post},
    Router,
};

use microservices_shared::domain_events::{DomainEventCallbacksLoggerImpl, DomainEventConsumer};
use opentelemetry::{
    trace::{Span, Tracer},
    KeyValue,
};
use sqlx::PgPool;
use std::sync::Arc;
use venues::config::AppConfig;
use venues::ports::rest::venues::{
    create_venue_handler, get_all_venues_handler, get_venue_by_id_handler,
};
use venues::AppState;

#[tokio::main]
async fn main() {
    env_logger::init();

    let config = AppConfig::new_from_env();

    let tracer = microservices_shared::init_tracing(&config.otlp_endpoint, "venues");
    let mut span = tracer.start("application_start");
    span.set_attribute(KeyValue::new("server_host", config.server_host.clone()));

    let tracer_arc = Arc::new(tracer);

    let connection_pool = PgPool::connect(&config.db_url).await.unwrap();

    let domain_event_callbacks = Box::new(DomainEventCallbacksLoggerImpl::new(tracer_arc.clone()));
    let mut domain_event_consumer = DomainEventConsumer::new(
        &config.kafka_consumer_group,
        &config.kafka_url,
        &config.kafka_domain_events_topics,
        connection_pool.clone(),
        domain_event_callbacks,
    );

    let app_state = AppState {
        connection_pool: connection_pool.clone(),
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
        .route("/venues", post(create_venue_handler))
        .route("/venues/:id", get(get_venue_by_id_handler))
        .route("/venues/all", get(get_all_venues_handler))
        .layer(cors)
        .with_state(state_arc);

    let listener = tokio::net::TcpListener::bind(&config.server_host)
        .await
        .unwrap();

    tokio::spawn(async move {
        domain_event_consumer.run().await;
    });

    span.end();

    axum::serve(listener, app).await.unwrap();
}
