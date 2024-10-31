use crate::adapters::db::fixture_repo_pg::FixtureRepositoryPg;
use crate::domain::aggregates::fixture::Fixture;
use crate::domain::repositories::fixture_repo::FixtureRepository;
use crate::{application, AppState};
use axum::extract::{Path, State};
use axum::Json;
use chrono::{DateTime, Utc};
use log::debug;
use microservices_shared::domain_ids::VenueId;
use microservices_shared::resolvers::impls::{
    RefereeResolverImpl, TeamResolverImpl, VenueResolverImpl,
};
use microservices_shared::resolvers::traits::{RefereeResolver, TeamResolver, VenueResolver};
use restinterface::{FixtureCreationDTO, FixtureDTO, FixtureIdDTO};
use shared::app_error::AppError;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

pub async fn create_fixture_handler(
    State(state): State<Arc<AppState>>,
    Json(fixture_creation): Json<FixtureCreationDTO>,
) -> Result<Json<FixtureDTO>, AppError> {
    debug!("Creating fixture: {:?}", fixture_creation);

    let fixture = microservices_shared::domain_events::run_domain_event_publisher_transactional(
        &state.domain_event_publisher,
        async {
            let mut tx = state
                .connection_pool
                .begin()
                .await
                .map_err(|e| e.to_string())?;

            let redis_conn = state
                .redis_client
                .get_connection()
                .map_err(|e| e.to_string())?;
            let redis_conn_arc_mutex = Arc::new(Mutex::new(redis_conn));

            let fixture_repo = FixtureRepositoryPg::new();
            let venue_resolver = VenueResolverImpl::new(redis_conn_arc_mutex.clone());
            let team_resolver = TeamResolverImpl::new(redis_conn_arc_mutex.clone());

            let fixture = application::fixture_services::create_fixture(
                fixture_creation.date,
                fixture_creation.venue_id.into(),
                fixture_creation.team_home_id.into(),
                fixture_creation.team_away_id.into(),
                &fixture_repo,
                &venue_resolver,
                &team_resolver,
                &state.domain_event_publisher,
                &mut tx,
            )
            .await
            .map_err(|e| e.to_string())?;

            tx.commit().await.map_err(|e| e.to_string())?;

            Ok(fixture)
        },
    )
    .await
    .map_err(|e| AppError::from_error(&e.to_string()))?;

    Ok(Json(fixture))
}

pub async fn get_fixture_by_id_handler(
    State(state): State<Arc<AppState>>,
    Path(fixture_id): Path<FixtureIdDTO>,
) -> Result<Json<Option<FixtureDTO>>, AppError> {
    debug!("Getting fixture by id: {}", fixture_id.0);

    let mut tx = state
        .connection_pool
        .begin()
        .await
        .map_err(|e| AppError::from_error(&e.to_string()))?;

    let redis_conn = state
        .redis_client
        .get_connection()
        .map_err(|e| AppError::from_error(&e.to_string()))?;
    let redis_conn_arc_mutex = Arc::new(Mutex::new(redis_conn));

    let repo = FixtureRepositoryPg::new();
    let result = repo
        .find_by_id(fixture_id.into(), &mut tx)
        .await
        .map_err(|e| AppError::from_error(&e.to_string()))?;

    debug!("Fixture: {:?}", result);

    let venue_resolver = VenueResolverImpl::new(redis_conn_arc_mutex.clone());
    let team_resolver = TeamResolverImpl::new(redis_conn_arc_mutex.clone());
    let referee_resolver = RefereeResolverImpl::new(redis_conn_arc_mutex.clone());

    match result {
        Some(fixture) => Ok(Json(Some(
            resolve_fixture_dto(fixture, &venue_resolver, &team_resolver, &referee_resolver)
                .await
                .map_err(|e| AppError::from_error(&e.to_string()))?,
        ))),
        None => Ok(Json(None)),
    }
}

pub async fn get_all_fixtures_handler(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<FixtureDTO>>, AppError> {
    debug!("Getting all fixtures");

    let mut tx = state
        .connection_pool
        .begin()
        .await
        .map_err(|e| AppError::from_error(&e.to_string()))?;

    let redis_conn = state
        .redis_client
        .get_connection()
        .map_err(|e| AppError::from_error(&e.to_string()))?;
    let redis_conn_arc_mutex = Arc::new(Mutex::new(redis_conn));

    let repo = FixtureRepositoryPg::new();

    let fixtures = repo
        .get_all(&mut tx)
        .await
        .map_err(|e| AppError::from_error(&e.to_string()))?;

    debug!("Fixtures: {:?}", fixtures);

    let venue_resolver = VenueResolverImpl::new(redis_conn_arc_mutex.clone());
    let team_resolver = TeamResolverImpl::new(redis_conn_arc_mutex.clone());
    let referee_resolver = RefereeResolverImpl::new(redis_conn_arc_mutex.clone());

    let mut resolved_fixtures = Vec::new();
    for fixture in fixtures {
        resolved_fixtures.push(
            resolve_fixture_dto(fixture, &venue_resolver, &team_resolver, &referee_resolver)
                .await
                .map_err(|e| AppError::from_error(&e.to_string()))?,
        );
    }

    Ok(Json(resolved_fixtures))
}

pub async fn update_fixture_date_handler(
    State(state): State<Arc<AppState>>,
    Path(fixture_id): Path<FixtureIdDTO>,
    Json(date): Json<DateTime<Utc>>,
) -> Result<Json<()>, AppError> {
    debug!("Updating fixture date: {}", fixture_id.0);

    microservices_shared::domain_events::run_domain_event_publisher_transactional(
        &state.domain_event_publisher,
        async {
            let mut tx = state
                .connection_pool
                .begin()
                .await
                .map_err(|e| e.to_string())?;

            let fixture_repo = FixtureRepositoryPg::new();

            let _ = application::fixture_services::update_fixture_date(
                fixture_id.into(),
                date,
                &fixture_repo,
                &state.domain_event_publisher,
                &mut tx,
            )
            .await
            .map_err(|e| e.to_string())?;

            tx.commit().await.map_err(|e| e.to_string())?;

            Ok(())
        },
    )
    .await
    .map_err(|e| AppError::from_error(&e.to_string()))?;

    Ok(Json(()))
}

pub async fn update_fixture_venue_handler(
    State(state): State<Arc<AppState>>,
    Path(fixture_id): Path<FixtureIdDTO>,
    Json(venue_id): Json<Uuid>,
) -> Result<Json<()>, AppError> {
    debug!("Updating fixture venue: {}", fixture_id.0);

    microservices_shared::domain_events::run_domain_event_publisher_transactional(
        &state.domain_event_publisher,
        async {
            let mut tx = state
                .connection_pool
                .begin()
                .await
                .map_err(|e| e.to_string())?;

            let redis_conn = state
                .redis_client
                .get_connection()
                .map_err(|e| e.to_string())?;
            let redis_conn_arc_mutex = Arc::new(Mutex::new(redis_conn));

            let fixture_repo = FixtureRepositoryPg::new();
            let venue_resolver = VenueResolverImpl::new(redis_conn_arc_mutex.clone());

            let _ = application::fixture_services::update_fixture_venue(
                fixture_id.into(),
                VenueId::from(venue_id),
                &fixture_repo,
                &venue_resolver,
                &state.domain_event_publisher,
                &mut tx,
            )
            .await
            .map_err(|e| e.to_string())?;

            tx.commit().await.map_err(|e| e.to_string())?;

            Ok(())
        },
    )
    .await
    .map_err(|e| AppError::from_error(&e.to_string()))?;

    Ok(Json(()))
}

pub async fn cancel_fixture_handler(
    State(state): State<Arc<AppState>>,
    Path(fixture_id): Path<FixtureIdDTO>,
) -> Result<Json<()>, AppError> {
    debug!("Cancelling fixture: {}", fixture_id.0);

    microservices_shared::domain_events::run_domain_event_publisher_transactional(
        &state.domain_event_publisher,
        async {
            let mut tx = state
                .connection_pool
                .begin()
                .await
                .map_err(|e| e.to_string())?;

            let fixture_repo = FixtureRepositoryPg::new();

            let _ = application::fixture_services::cancel_fixture(
                fixture_id.into(),
                &fixture_repo,
                &state.domain_event_publisher,
                &mut tx,
            )
            .await
            .map_err(|e| e.to_string())?;

            tx.commit().await.map_err(|e| e.to_string())?;

            Ok(())
        },
    )
    .await
    .map_err(|e| AppError::from_error(&e.to_string()))?;

    Ok(Json(()))
}

async fn resolve_fixture_dto(
    fixture: Fixture,
    venue_resolver: &impl VenueResolver<Error = String>,
    team_resolver: &impl TeamResolver<Error = String>,
    referee_resolver: &impl RefereeResolver<Error = String>,
) -> Result<FixtureDTO, AppError> {
    let venue = venue_resolver
        .resolve(&fixture.venue_id())
        .await
        .map_err(|e| AppError::from_error(&e))?;
    let team_home = team_resolver
        .resolve(&fixture.team_home_id())
        .await
        .map_err(|e| AppError::from_error(&e))?;
    let team_away = team_resolver
        .resolve(&fixture.team_away_id())
        .await
        .map_err(|e| AppError::from_error(&e))?;

    let first_referee = match fixture.first_referee_id() {
        Some(referee) => {
            let referee = referee_resolver
                .resolve(&referee)
                .await
                .map_err(|e| AppError::from_error(&e.to_string()))?;
            Some(referee)
        }
        None => None,
    };

    let second_referee = match fixture.second_referee_id() {
        Some(referee) => {
            let referee = referee_resolver
                .resolve(&referee)
                .await
                .map_err(|e| AppError::from_error(&e.to_string()))?;
            Some(referee)
        }
        None => None,
    };

    Ok(FixtureDTO {
        id: fixture.id().into(),
        team_home,
        team_away,
        venue,
        date: fixture.date().clone(),
        status: fixture.status().clone().into(),
        first_referee,
        second_referee,
    })
}

#[cfg(test)]
mod fixture_tests {
    use chrono::Utc;
    use restinterface::{
        cancel_fixture, change_fixture_date, change_fixture_venue, create_test_fixture,
        create_venue, fetch_fixture, fetch_fixtures, FixtureStatusDTO, VenueCreationDTO,
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

        let (fixture_creation, _fixture_dto) = create_test_fixture().await;

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

        let (_fixture_creation, fixture_dto) = create_test_fixture().await;

        let new_date = Utc::now();
        let result = change_fixture_date(fixture_dto.id.into(), new_date).await;
        assert!(result.is_ok(), "Fixture should be changed");
        let fixture_dto = fetch_fixture(fixture_dto.id.into()).await.unwrap();
        assert_eq!(
            fixture_dto.date.timestamp_millis(),
            new_date.timestamp_millis(),
            "Fixture date should have changed"
        );
    }

    #[tokio::test]
    async fn given_fixture_when_changing_venue_then_fixture_with_new_venue_is_returned() {
        clear_tables().await;

        let (_fixture_creation, fixture_dto) = create_test_fixture().await;

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

        let result = change_fixture_venue(fixture_dto.id.into(), new_venue.id.into()).await;
        assert!(result.is_ok(), "Fixture venue change should be ok");
        let fixture_dto = fetch_fixture(fixture_dto.id.into()).await.unwrap();
        assert_eq!(
            fixture_dto.venue.id, new_venue.id,
            "Fixture venue should have changed"
        );
    }

    #[tokio::test]
    async fn given_fixture_when_cancel_then_fixture_with_cancelled_is_returned() {
        clear_tables().await;

        let (_fixture_creation, fixture_dto) = create_test_fixture().await;

        let result = cancel_fixture(fixture_dto.id.into()).await;
        assert!(result.is_ok(), "Fixture cancellation should be ok");
        let fixture_dto = fetch_fixture(fixture_dto.id.into()).await.unwrap();
        assert_eq!(
            fixture_dto.status,
            FixtureStatusDTO::Cancelled,
            "Fixture status should have cancelled"
        );
    }

    // NOTE: deleting tables in other microservices is a fundamental violation of Microservices architecture, however its the simplest way to clear the db for tests
    async fn clear_tables() {
        let fixtures_db_url = "postgres://postgres:postgres@localhost:5436/fixtures?application_name=rustddd&options=-c search_path%3Drustddd";
        let teams_db_url = "postgres://postgres:postgres@localhost:5435/teams?application_name=rustddd&options=-c search_path%3Drustddd";
        let venues_db_url = "postgres://postgres:postgres@localhost:5434/venues?application_name=rustddd&options=-c search_path%3Drustddd";

        let fixtures_pool = PgPool::connect(&fixtures_db_url).await.unwrap();
        let teams_pool = PgPool::connect(&teams_db_url).await.unwrap();
        let venues_pool = PgPool::connect(&venues_db_url).await.unwrap();

        sqlx::query!("DELETE FROM rustddd.fixtures")
            .execute(&fixtures_pool)
            .await
            .unwrap();

        sqlx::query("DELETE FROM rustddd.venues")
            .execute(&venues_pool)
            .await
            .unwrap();

        sqlx::query("DELETE FROM rustddd.teams")
            .execute(&teams_pool)
            .await
            .unwrap();
    }
}
