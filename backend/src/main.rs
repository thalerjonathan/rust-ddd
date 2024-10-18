use crate::config::AppConfig;
use axum::http::Method;
use axum::routing::{delete, put};
use axum::{
    routing::{get, post},
    Router,
};

use ports::rest::assignments::{
    commit_assignments_handler, fetch_assignments_handler, remove_committed_assignment_handler, remove_staged_assignment_handler, stage_assignment_handler, validate_assignments_handler
};
use ports::rest::availabilities::{
    declare_availability_handler, fetch_availabilities_for_referee_handler,
    withdraw_availability_handler,
};
use ports::rest::fixture::{
    cancel_fixture_handler, create_fixture_handler, get_all_fixtures_handler,
    get_fixture_by_id_handler, update_fixture_date_handler, update_fixture_venue_handler,
};
use ports::rest::referee::*;
use ports::rest::state::AppState;
use ports::rest::team::{create_team_handler, get_all_teams_handler, get_team_by_id_handler};
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
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::OPTIONS])
        .allow_headers(tower_http::cors::Any)
        .allow_origin(tower_http::cors::Any);

    let app = Router::new()
        .route("/referee", post(create_referee_handler))
        .route("/referee/:id", get(get_referee_by_id_handler))
        .route("/referees", get(get_all_referees_handler))
        .route("/referee/:id/club", post(update_referee_club_handler))
        .route("/venue", post(create_venue_handler))
        .route("/venue/:id", get(get_venue_by_id_handler))
        .route("/venues", get(get_all_venues_handler))
        .route("/team", post(create_team_handler))
        .route("/team/:id", get(get_team_by_id_handler))
        .route("/teams", get(get_all_teams_handler))
        .route("/fixture", post(create_fixture_handler))
        .route("/fixture/:id", get(get_fixture_by_id_handler))
        .route("/fixtures", get(get_all_fixtures_handler))
        .route("/fixture/:id/date", post(update_fixture_date_handler))
        .route("/fixture/:id/venue", post(update_fixture_venue_handler))
        .route("/fixture/:id/cancel", post(cancel_fixture_handler))
        .route(
            "/availabilities/declare/fixture/:fixture_id/referee/:referee_id",
            post(declare_availability_handler),
        )
        .route(
            "/availabilities/withdraw/fixture/:fixture_id/referee/:referee_id",
            post(withdraw_availability_handler),
        )
        .route(
            "/availabilities/referee/:referee_id",
            get(fetch_availabilities_for_referee_handler),
        )
        .route("/assignments", get(fetch_assignments_handler))
        .route("/assignments", put(stage_assignment_handler))
        .route("/assignments/staged/:fixture_id/:referee_id", delete(remove_staged_assignment_handler))
        .route("/assignments/committed/:fixture_id/:referee_id", delete(remove_committed_assignment_handler))
        .route("/assignments/validate", post(validate_assignments_handler))
        .route("/assignments/commit", post(commit_assignments_handler))
        .layer(cors)
        .with_state(state_arc);

    let listener = tokio::net::TcpListener::bind(&app_cfg.server_host)
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
