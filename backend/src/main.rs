use crate::config::AppConfig;
use crate::handlers::referee::{create_referee, get_all_referees, get_referee_by_id};
use crate::handlers::state::AppState;
use axum::{
    routing::{get, post},
    Router,
};

use sqlx::PgPool;
use std::sync::Arc;
mod config;
mod handlers;
mod models;

#[tokio::main]
async fn main() {
    env_logger::init();
    let app_cfg = AppConfig::new_from_env();
    let connection_pool = PgPool::connect(&app_cfg.db_url).await.unwrap();

    let app_state = AppState { connection_pool };
    let state_arc = Arc::new(app_state);

    let app = Router::new()
        .route("/referee", post(create_referee))
        .route("/referee/:id", get(get_referee_by_id))
        .route("/referees", get(get_all_referees))
        .with_state(state_arc);

    let listener = tokio::net::TcpListener::bind(&app_cfg.server_host)
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
