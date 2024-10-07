use crate::config::AppConfig;
use axum::http::Method;
use axum::{
    routing::{get, post},
    Router,
};

use ports::rest::referee::{
    create_referee, get_all_referees, get_referee_by_id, update_referee_club,
};
use ports::rest::state::AppState;
use ports::rest::venues::*;
use sqlx::PgPool;
use std::sync::Arc;

mod adapters;
mod application;
mod config;
mod domain;
mod ports;

#[tokio::main]
async fn main() {
    env_logger::init();
    let app_cfg = AppConfig::new_from_env();
    let connection_pool = PgPool::connect(&app_cfg.db_url).await.unwrap();

    let app_state = AppState { connection_pool };
    let state_arc = Arc::new(app_state);

    let cors = tower_http::cors::CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers(tower_http::cors::Any)
        .allow_origin(tower_http::cors::Any);

    let app = Router::new()
        .route("/referee", post(create_referee))
        .route("/referee/:id", get(get_referee_by_id))
        .route("/referees", get(get_all_referees))
        .route("/referee/:id/club", post(update_referee_club))
        .layer(cors)
        .with_state(state_arc);

    let listener = tokio::net::TcpListener::bind(&app_cfg.server_host)
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
