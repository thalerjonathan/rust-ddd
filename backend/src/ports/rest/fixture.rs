use crate::adapters::db::fixture_repo_pg::FixtureRepositoryPg;
use crate::adapters::db::team_repo_pg::TeamRepositoryPg;
use crate::adapters::db::venue_repo_pg::VenueRepositoryPg;
use crate::application;
use crate::domain::aggregates::fixture::FixtureId;
use crate::domain::repositories::fixture_repo::FixtureRepository;
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

    // TODO: we must pass the same connection to each of them, otherwise each query would have different transactional context
    let fixture_repo = FixtureRepositoryPg::new(&state.connection_pool);
    let venue_repo = VenueRepositoryPg::new(&state.connection_pool);
    let team_repo = TeamRepositoryPg::new(&state.connection_pool);

    let fixture = application::fixture_services::create_fixture(
        fixture_creation.date,
        fixture_creation.venue_id.into(),
        fixture_creation.team_home_id.into(),
        fixture_creation.team_away_id.into(),
        &fixture_repo,
        &venue_repo,
        &team_repo,
    )
    .await
    .map_err(|e| AppError::from_error(&e.to_string()))?;

    debug!("Fixture created: {:?}", fixture);

    Ok(Json(fixture.into()))
}

pub async fn get_fixture_by_id_handler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<Option<FixtureDTO>>, AppError> {
    debug!("Getting fixture by id: {}", id);

    let repo = FixtureRepositoryPg::new(&state.connection_pool);

    let fixture = repo
        .find_by_id(&FixtureId::from(id))
        .await
        .map_err(|e| AppError::from_error(&e.to_string()))?;

    Ok(Json(fixture.map(|f| f.into())))
}

pub async fn get_all_fixtures_handler(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<FixtureDTO>>, AppError> {
    debug!("Getting all fixtures");

    let repo = FixtureRepositoryPg::new(&state.connection_pool);

    let fixtures = repo
        .get_all()
        .await
        .map_err(|e| AppError::from_error(&e.to_string()))?;

    debug!("Fixtures: {:?}", fixtures);

    Ok(Json(fixtures.into_iter().map(|f| f.into()).collect()))
}

#[cfg(test)]
mod fixture_tests {
    use chrono::Utc;
    use shared::{
        create_team, create_venue, fetch_fixture, fetch_fixtures, FixtureCreationDTO,
        TeamCreationDTO, VenueCreationDTO,
    };
    use sqlx::PgPool;

    #[tokio::test]
    async fn given_empty_db_when_fetch_fixture_then_empty_list_is_returned() {
        clear_tables().await;

        let fixtures = fetch_fixtures().await;
        assert!(fixtures.is_empty(), "Fixtures should be empty");
    }

    #[tokio::test]
    async fn given_empty_db_when_creating_fixture_then_fixture_is_returned() {
        clear_tables().await;

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

        // NOTE: we cannot use assert_eq! because loading the fixture from the db returns a DateTime without nanos
        assert_eq!(
            fixtures[0].date.timestamp_millis(),
            now.timestamp_millis(),
            "Fixture date should be now"
        );
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
        // NOTE: we cannot use assert_eq! because loading the fixture from the db returns a DateTime without nanos
        assert_eq!(
            fixture_dto.date.timestamp_millis(),
            now.timestamp_millis(),
            "Fixture date should be now"
        );
        assert_eq!(
            fixture_dto.venue.id, venue.id,
            "Fixture venue should be venue"
        );
    }

    async fn clear_tables() {
        let db_url = std::env::var("DB_URL").expect("DB_URL not set");
        let pool = PgPool::connect(&db_url).await.unwrap();
        sqlx::query!("DELETE FROM rustddd.fixtures")
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query!("DELETE FROM rustddd.venues")
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query!("DELETE FROM rustddd.teams")
            .execute(&pool)
            .await
            .unwrap();
    }
}
