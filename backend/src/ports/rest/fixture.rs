use crate::adapters::db::fixture_repo_pg::FixtureRepositoryPg;
use crate::adapters::db::team_repo_pg::TeamRepositoryPg;
use crate::adapters::db::venue_repo_pg::VenueRepositoryPg;
use crate::application;
use crate::domain::aggregates::fixture::FixtureId;
use crate::domain::aggregates::venue::VenueId;
use crate::domain::repositories::fixture_repo::FixtureRepository;
use crate::ports::rest::state::AppState;
use axum::extract::{Path, State};
use axum::Json;
use chrono::{DateTime, Utc};
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

    let mut tx = state.connection_pool.begin().await.unwrap();

    let fixture_repo = FixtureRepositoryPg::new();
    let mut venue_repo = VenueRepositoryPg::new();
    let mut team_repo = TeamRepositoryPg::new();

    let fixture = application::fixture_services::create_fixture(
        fixture_creation.date,
        fixture_creation.venue_id.into(),
        fixture_creation.team_home_id.into(),
        fixture_creation.team_away_id.into(),
        &fixture_repo,
        &mut venue_repo,
        &mut team_repo,
        &mut tx,
    )
    .await
    .map_err(|e| AppError::from_error(&e.to_string()))?;

    tx.commit()
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

    let mut tx = state.connection_pool.begin().await.unwrap();

    let repo = FixtureRepositoryPg::new();
    let fixture = repo
        .find_by_id(&FixtureId::from(id), &mut tx)
        .await
        .map_err(|e| AppError::from_error(&e.to_string()))?;

    Ok(Json(fixture.map(|f| f.into())))
}

pub async fn get_all_fixtures_handler(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<FixtureDTO>>, AppError> {
    debug!("Getting all fixtures");

    let mut tx = state.connection_pool.begin().await.unwrap();

    let repo = FixtureRepositoryPg::new();

    let fixtures = repo
        .get_all(&mut tx)
        .await
        .map_err(|e| AppError::from_error(&e.to_string()))?;

    debug!("Fixtures: {:?}", fixtures);

    Ok(Json(fixtures.into_iter().map(|f| f.into()).collect()))
}

pub async fn update_fixture_date_handler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(date): Json<DateTime<Utc>>,
) -> Result<Json<FixtureDTO>, AppError> {
    debug!("Updating fixture date: {}", id);

    let mut tx = state.connection_pool.begin().await.unwrap();

    let fixture_repo = FixtureRepositoryPg::new();

    let fixture = application::fixture_services::update_fixture_date(
        FixtureId::from(id),
        date,
        &fixture_repo,
        &mut tx,
    )
    .await
    .map_err(|e| AppError::from_error(&e.to_string()))?;

    tx.commit()
        .await
        .map_err(|e| AppError::from_error(&e.to_string()))?;
    Ok(Json(fixture.into()))
}

pub async fn update_fixture_venue_handler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(venue_id): Json<Uuid>,
) -> Result<Json<FixtureDTO>, AppError> {
    debug!("Updating fixture venue: {}", id);

    let mut tx = state.connection_pool.begin().await.unwrap();

    let fixture_repo = FixtureRepositoryPg::new();
    let venue_repo = VenueRepositoryPg::new();

    let fixture = application::fixture_services::update_fixture_venue(
        FixtureId::from(id),
        VenueId::from(venue_id),
        &fixture_repo,
        &venue_repo,
        &mut tx,
    )
    .await
    .map_err(|e| AppError::from_error(&e.to_string()))?;

    tx.commit()
        .await
        .map_err(|e| AppError::from_error(&e.to_string()))?;

    Ok(Json(fixture.into()))
}

pub async fn cancel_fixture_handler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<FixtureDTO>, AppError> {
    debug!("Cancelling fixture: {}", id);

    let mut tx = state.connection_pool.begin().await.unwrap();

    let fixture_repo = FixtureRepositoryPg::new();

    let fixture =
        application::fixture_services::cancel_fixture(FixtureId::from(id), &fixture_repo, &mut tx)
            .await
            .map_err(|e| AppError::from_error(&e.to_string()))?;

    tx.commit()
        .await
        .map_err(|e| AppError::from_error(&e.to_string()))?;

    Ok(Json(fixture.into()))
}

#[cfg(test)]
mod fixture_tests {
    use chrono::Utc;
    use shared::{
        cancel_fixture, change_fixture_date, change_fixture_venue, create_team, create_venue,
        fetch_fixture, fetch_fixtures, FixtureCreationDTO, FixtureDTO, FixtureStatusDTO,
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

        let (fixture_creation, _fixture_dto) = create_fixture().await;

        let fixtures = fetch_fixtures().await;
        assert!(!fixtures.is_empty(), "Fixtures should not be empty");
        assert_eq!(fixtures.len(), 1, "Fixtures should have 1 fixture");

        // NOTE: we cannot use assert_eq! because loading the fixture from the db returns a DateTime without nanos
        assert_eq!(
            fixtures[0].date.timestamp_millis(),
            fixture_creation.date.timestamp_millis(),
            "Fixture date should be now"
        );
        assert_eq!(
            fixtures[0].venue.id, fixture_creation.venue_id,
            "Fixture venue should be venue"
        );
        assert_eq!(
            fixtures[0].team_home.id, fixture_creation.team_home_id,
            "Fixture home team should be team_home"
        );
        assert_eq!(
            fixtures[0].team_away.id, fixture_creation.team_away_id,
            "Fixture away team should be team_away"
        );
        assert_eq!(
            fixtures[0].status,
            FixtureStatusDTO::Scheduled,
            "Fixture status should be scheduled"
        );

        let fixture_dto = fetch_fixture(fixtures[0].id.into()).await.unwrap();
        assert_eq!(
            fixture_dto.id, fixtures[0].id,
            "Fixture id should be the same"
        );
        // NOTE: we cannot use assert_eq! because loading the fixture from the db returns a DateTime without nanos
        assert_eq!(
            fixture_dto.date.timestamp_millis(),
            fixture_creation.date.timestamp_millis(),
            "Fixture date should be now"
        );
        assert_eq!(
            fixture_dto.venue.id, fixture_creation.venue_id,
            "Fixture venue should be venue"
        );
    }

    #[tokio::test]
    async fn given_fixture_when_changing_date_then_fixture_with_new_date_is_returned() {
        clear_tables().await;

        let (_fixture_creation, fixture_dto) = create_fixture().await;

        let new_date = Utc::now();
        let fixture_dto = change_fixture_date(fixture_dto.id.into(), new_date).await;
        assert!(fixture_dto.is_ok(), "Fixture should be changed");
        let fixture_dto = fetch_fixture(fixture_dto.unwrap().id.into())
            .await
            .unwrap();
        assert_eq!(
            fixture_dto.date.timestamp_millis(),
            new_date.timestamp_millis(),
            "Fixture date should have changed"
        );
    }

    #[tokio::test]
    async fn given_fixture_when_changing_venue_then_fixture_with_new_venue_is_returned() {
        clear_tables().await;

        let (_fixture_creation, fixture_dto) = create_fixture().await;

        let new_venue = create_venue(&VenueCreationDTO {
            name: "Venue B".to_string(),
            street: "Street B".to_string(),
            zip: "12345".to_string(),
            city: "City B".to_string(),
            telephone: Some("1234567890".to_string()),
            email: Some("email@example.com".to_string()),
        })
        .await
        .unwrap();

        let fixture_dto =
            change_fixture_venue(fixture_dto.id.into(), new_venue.id.into()).await;
        assert!(fixture_dto.is_ok(), "Fixture venue change should be ok");
        let fixture_dto = fetch_fixture(fixture_dto.unwrap().id.into())
            .await
            .unwrap();
        assert_eq!(
            fixture_dto.venue.id, new_venue.id,
            "Fixture venue should have changed"
        );
    }

    #[tokio::test]
    async fn given_fixture_when_cancel_then_fixture_with_cancelled_is_returned() {
        clear_tables().await;

        let (_fixture_creation, fixture_dto) = create_fixture().await;

        let fixture_dto = cancel_fixture(fixture_dto.id.into()).await;
        assert!(fixture_dto.is_ok(), "Fixture cancellation should be ok");
        let fixture_dto = fetch_fixture(fixture_dto.unwrap().id.into())
            .await
            .unwrap();
        assert_eq!(
            fixture_dto.status,
            FixtureStatusDTO::Cancelled,
            "Fixture status should have cancelled"
        );
    }

    async fn create_fixture() -> (FixtureCreationDTO, FixtureDTO) {
        let team_home = create_team(&TeamCreationDTO {
            name: "Team A".to_string(),
            club: "Club A".to_string(),
        })
        .await
        .unwrap();

        let team_away = create_team(&TeamCreationDTO {
            name: "Team B".to_string(),
            club: "Club B".to_string(),
        })
        .await
        .unwrap();

        let venue = create_venue(&VenueCreationDTO {
            name: "Venue A".to_string(),
            street: "Street A".to_string(),
            zip: "12345".to_string(),
            city: "City A".to_string(),
            telephone: Some("1234567890".to_string()),
            email: Some("email@example.com".to_string()),
        })
        .await
        .unwrap();

        let fixture_creation = FixtureCreationDTO {
            date: Utc::now(),
            venue_id: venue.id,
            team_home_id: team_home.id,
            team_away_id: team_away.id,
        };

        let fixture_dto = shared::create_fixture(&fixture_creation).await;
        assert!(fixture_dto.is_ok(), "Fixture should be created");

        (fixture_creation, fixture_dto.unwrap())
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
