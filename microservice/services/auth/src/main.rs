use auth::config::AppConfig;

use auth::AppState;
use auth::handlers::{login_handler, status_handler};
use axum::http::Method;
use axum::{
    Router,
    routing::{get, post},
};

use microservices_shared::token::TokenManager;
use opentelemetry::{
    KeyValue,
    trace::{Span, Tracer},
};
use tokio::sync::Mutex;

use std::sync::Arc;

#[tokio::main]
async fn main() {
    env_logger::init();

    let config = AppConfig::new_from_env();

    let tracer = microservices_shared::init_tracing(&config.otlp_endpoint, "auth");
    let mut span = tracer.start("application_start");
    span.set_attribute(KeyValue::new("server_host", config.server_host.clone()));

    let tracer_arc = Arc::new(tracer);

    let redis_client = redis::Client::open(config.redis_url).unwrap();
    let redis_conn = redis_client.get_connection().unwrap();

    let token_manager = TokenManager::new(&config.idp_host, &config.idp_realm, &config.client_id)
        .await
        .unwrap();

    let app_state = AppState {
        token_manager,
        redis_conn: Mutex::new(redis_conn),
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
        .route("/auth/login", post(login_handler))
        .route("/auth/status", get(status_handler))
        .layer(cors)
        .with_state(state_arc);

    let listener = tokio::net::TcpListener::bind(&config.server_host)
        .await
        .unwrap();

    span.end();

    axum::serve(listener, app).await.unwrap();
}
