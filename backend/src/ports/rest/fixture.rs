use crate::ports::rest::state::AppState;
use axum::extract::{Path, State};
use axum::Json;
use log::debug;
use shared::{FixtureCreationDTO, FixtureDTO};
use std::sync::Arc;
use uuid::Uuid;

use super::shared::AppError;

pub async fn create_fixture_handler(
    State(state): State<Arc<AppState>>,
    Json(fixture_creation): Json<FixtureCreationDTO>,
) -> Result<Json<FixtureDTO>, AppError> {
    debug!("Creating fixture: {:?}", fixture_creation);
    Err(AppError::from_error("Not implemented"))
}

pub async fn get_fixture_by_id_handler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<Option<FixtureDTO>>, AppError> {
    debug!("Getting fixture by id: {}", id);
    Err(AppError::from_error("Not implemented"))
}

pub async fn get_all_fixtures_handler(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<FixtureDTO>>, AppError> {
    debug!("Getting all fixtures");
    Err(AppError::from_error("Not implemented"))
}

#[cfg(test)]
mod fixture_tests {
    use chrono::Utc;
    use shared::{
        create_team, create_venue, fetch_fixture, fetch_fixtures, FixtureCreationDTO, FixtureDTO,
        TeamCreationDTO, VenueCreationDTO,
    };
    use sqlx::PgPool;

    #[tokio::test]
    async fn given_empty_db_when_creating_fixture_then_empty_list_is_returned() {
        clear_fixtures_venues_teams_table().await;

        let fixtures = fetch_fixtures().await;
        assert!(fixtures.is_empty(), "Fixtures should be empty");
    }

    #[tokio::test]
    async fn given_empty_db_when_creating_fixture_then_fixture_is_returned() {
        clear_fixtures_venues_teams_table().await;

        let team_home = create_team(TeamCreationDTO {
            name: "Team A".to_string(),
            club: "Club A".to_string(),
        })
        .await
        .unwrap();

        let team_away = create_team(TeamCreationDTO {
            name: "Team B".to_string(),
            club: "Club B".to_string(),
        })
        .await
        .unwrap();

        let venue = create_venue(VenueCreationDTO {
            name: "Venue A".to_string(),
            street: "Street A".to_string(),
            zip: "12345".to_string(),
            city: "City A".to_string(),
            telephone: Some("1234567890".to_string()),
            email: Some("email@example.com".to_string()),
        })
        .await
        .unwrap();

        let now = Utc::now();

        let fixture_creation = FixtureCreationDTO {
            date: now,
            venue_id: venue.id,
            team_home_id: team_home.id,
            team_away_id: team_away.id,
        };

        let fixture_dto = shared::create_fixture(fixture_creation).await;
        assert!(fixture_dto.is_ok(), "Fixture should be created");

        let fixtures = fetch_fixtures().await;
        assert!(!fixtures.is_empty(), "Fixtures should not be empty");
        assert_eq!(fixtures.len(), 1, "Fixtures should have 1 fixture");

        assert_eq!(fixtures[0].date, now, "Fixture date should be now");
        assert_eq!(
            fixtures[0].venue.id, venue.id,
            "Fixture venue should be venue"
        );
        assert_eq!(
            fixtures[0].team_home.id, team_home.id,
            "Fixture home team should be team_home"
        );
        assert_eq!(
            fixtures[0].team_away.id, team_away.id,
            "Fixture away team should be team_away"
        );

        let fixture_dto = fetch_fixture(&fixtures[0].id.to_string()).await.unwrap();
        assert_eq!(
            fixture_dto.id, fixtures[0].id,
            "Fixture id should be the same"
        );
        assert_eq!(fixture_dto.date, now, "Fixture date should be now");
        assert_eq!(
            fixture_dto.venue.id, venue.id,
            "Fixture venue should be venue"
        );
    }

    async fn clear_fixtures_venues_teams_table() {
        let db_url = std::env::var("DB_URL").expect("DB_URL not set");
        let pool = PgPool::connect(&db_url).await.unwrap();
        sqlx::query!("DELETE FROM fixtures")
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query!("DELETE FROM venues")
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query!("DELETE FROM teams")
            .execute(&pool)
            .await
            .unwrap();
    }
}
