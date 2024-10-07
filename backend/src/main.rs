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

#[allow(unused_imports)]
mod tests {
    use super::*;
    use shared::{change_referee_club, fetch_referees, RefereeCreationDTO};

    #[tokio::test]
    async fn given_empty_db_when_fetching_referees_then_empty_list_is_returned() {
        clear_db().await;

        let referees = fetch_referees().await;
        assert!(referees.is_empty(), "Referees should be empty");
    }

    #[tokio::test]
    async fn given_empty_db_when_creating_referee_then_referee_is_returned() {
        clear_db().await;

        let referee_creation = RefereeCreationDTO {
            name: "John Doe".to_string(),
            club: "Club A".to_string(),
        };

        let referee_dto = shared::create_referee(referee_creation).await;
        assert!(referee_dto.is_ok(), "Referee should be created");

        let referees = fetch_referees().await;
        assert!(!referees.is_empty(), "Referees should not be empty");
        assert_eq!(referees.len(), 1, "Referees should have 1 referee");
        assert_eq!(
            referees[0].name, "John Doe",
            "Referee name should be John Doe"
        );
        assert_eq!(referees[0].club, "Club A", "Referee club should be Club A");
    }

    #[tokio::test]
    async fn given_referee_when_updating_club_then_club_is_updated() {
        clear_db().await;

        let referee_creation = RefereeCreationDTO {
            name: "John Doe".to_string(),
            club: "Club A".to_string(),
        };

        let referee_dto = shared::create_referee(referee_creation).await;
        assert!(referee_dto.is_ok(), "Referee should be created");

        let referee_dto = referee_dto.unwrap();
        let updated_club = "Club B".to_string();
        let updated_referee_dto =
            change_referee_club(&referee_dto.id.to_string(), &updated_club).await;
        assert!(
            updated_referee_dto.is_ok(),
            "Referee club should be updated"
        );

        let referee_dto = shared::fetch_referee(&referee_dto.id.to_string()).await;
        assert_eq!(
            referee_dto.club, updated_club,
            "Referee club should be updated"
        );
    }

    #[allow(dead_code)]
    async fn clear_db() {
        let db_url = std::env::var("DB_URL").expect("DB_URL not set");
        let connection_pool = PgPool::connect(&db_url).await.unwrap();

        sqlx::query("DELETE FROM rustddd.referees")
            .execute(&connection_pool)
            .await
            .unwrap();
    }
}
