use axum::http::Method;
use axum::{
    routing::{get, post},
    Router,
};
use clap::Parser;

use sqlx::PgPool;
use std::sync::Arc;
use teams::ports::rest::teams::{
    create_team_handler, get_all_teams_handler, get_team_by_id_handler,
};
use teams::AppState;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    db_url: String,
    #[arg(short, long)]
    server_host: String,
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let args = Args::parse();

    let connection_pool = PgPool::connect(&args.db_url).await.unwrap();
    let app_state = AppState { connection_pool };
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

    axum::serve(listener, app).await.unwrap();
}
