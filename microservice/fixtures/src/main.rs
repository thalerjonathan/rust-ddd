use axum::http::Method;
use axum::{
    routing::{get, post},
    Router,
};
use clap::Parser;

use fixtures::ports::rest::fixtures::{
    cancel_fixture_handler, create_fixture_handler, get_all_fixtures_handler,
    get_fixture_by_id_handler, update_fixture_date_handler, update_fixture_venue_handler,
};
use fixtures::AppState;
use sqlx::PgPool;
use std::sync::Arc;

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
        .route("/fixtures", post(create_fixture_handler))
        .route("/fixtures/:id", get(get_fixture_by_id_handler))
        .route("/fixtures/all", get(get_all_fixtures_handler))
        .route("/fixtures/:id/date", post(update_fixture_date_handler))
        .route("/fixtures/:id/venue", post(update_fixture_venue_handler))
        .route("/fixtures/:id/cancel", post(cancel_fixture_handler))
        .layer(cors)
        .with_state(state_arc);

    let listener = tokio::net::TcpListener::bind(&args.server_host)
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
