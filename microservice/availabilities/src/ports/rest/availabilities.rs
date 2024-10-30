use std::sync::Arc;

use axum::extract::{Path, State};
use axum::Json;
use log::{debug, error};
use microservices_shared::resolvers::impls::{FixtureResolverImpl, RefereeResolverImpl};
use restinterface::app_error::AppError;
use restinterface::{FixtureIdDTO, RefereeIdDTO};
use tokio::sync::Mutex;

use crate::adapters::db::availability_repo_pg::AvailabilityRepositoryPg;
use crate::application::availability_services::{
    declare_availability, get_availabilities_for_referee, withdraw_availability,
};
use crate::AppState;

pub async fn declare_availability_handler(
    State(state): State<Arc<AppState>>,
    Path((fixture_id, referee_id)): Path<(FixtureIdDTO, RefereeIdDTO)>,
) -> Result<Json<()>, AppError> {
    debug!(
        "Declaring availability for fixture: {:?} and referee: {:?}",
        fixture_id, referee_id
    );

    microservices_shared::domain_events::run_domain_event_publisher_transactional(
        &state.domain_event_publisher,
        async {
            let mut tx = state.connection_pool.begin().await.map_err(|e| {
                error!("Error beginning transaction: {:?}", e);
                e.to_string()
            })?;

            let redis_conn = state
                .redis_client
                .get_connection()
                .map_err(|e| e.to_string())?;
            let redis_conn_arc_mutex = Arc::new(Mutex::new(redis_conn));

            let fixture_resolver = FixtureResolverImpl::new(redis_conn_arc_mutex.clone());
            let referee_resolver = RefereeResolverImpl::new(redis_conn_arc_mutex.clone());
            let availability_repo = AvailabilityRepositoryPg::new();

            declare_availability(
                fixture_id.into(),
                referee_id.into(),
                &fixture_resolver,
                &referee_resolver,
                &availability_repo,
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

pub async fn withdraw_availability_handler(
    State(state): State<Arc<AppState>>,
    Path((fixture_id, referee_id)): Path<(FixtureIdDTO, RefereeIdDTO)>,
) -> Result<Json<()>, AppError> {
    debug!(
        "Withdrawing availability for fixture: {:?} and referee: {:?}",
        fixture_id, referee_id
    );

    microservices_shared::domain_events::run_domain_event_publisher_transactional(
        &state.domain_event_publisher,
        async {
            let mut tx = state.connection_pool.begin().await.map_err(|e| {
                error!("Error beginning transaction: {:?}", e);
                e.to_string()
            })?;

            let redis_conn = state
                .redis_client
                .get_connection()
                .map_err(|e| e.to_string())?;
            let redis_conn_arc_mutex = Arc::new(Mutex::new(redis_conn));

            let fixture_resolver = FixtureResolverImpl::new(redis_conn_arc_mutex.clone());
            let referee_resolver = RefereeResolverImpl::new(redis_conn_arc_mutex.clone());
            let availability_repo = AvailabilityRepositoryPg::new();

            withdraw_availability(
                fixture_id.into(),
                referee_id.into(),
                &fixture_resolver,
                &referee_resolver,
                &availability_repo,
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

pub async fn fetch_availabilities_for_referee_handler(
    State(state): State<Arc<AppState>>,
    Path(referee_id): Path<RefereeIdDTO>,
) -> Result<Json<Vec<FixtureIdDTO>>, AppError> {
    debug!("Fetching availabilities for referee: {:?}", referee_id);

    let mut tx = state.connection_pool.begin().await.map_err(|e| {
        error!("Error beginning transaction: {:?}", e);
        AppError::from_error(&e.to_string())
    })?;

    let redis_conn = state
        .redis_client
        .get_connection()
        .map_err(|e| AppError::from_error(&e.to_string()))?;
    let redis_conn_arc_mutex = Arc::new(Mutex::new(redis_conn));

    let referee_resolver = RefereeResolverImpl::new(redis_conn_arc_mutex.clone());
    let availability_repo = AvailabilityRepositoryPg::new();

    let availabilities = get_availabilities_for_referee(
        referee_id.into(),
        &referee_resolver,
        &availability_repo,
        &mut tx,
    )
    .await
    .map_err(|e| AppError::from_error(&e.to_string()))?;

    tx.commit()
        .await
        .map_err(|e| AppError::from_error(&e.to_string()))?;

    Ok(Json(
        availabilities
            .into_iter()
            .map(|fixture_id| fixture_id.into())
            .collect(),
    ))
}

#[cfg(test)]
mod availabilities_tests {
    use restinterface::{
        declare_availability, fetch_availabilities_for_referee, withdraw_availability,
        RefereeCreationDTO,
    };
    use sqlx::PgPool;

    #[tokio::test]
    async fn availability_scenario() {
        clear_tables().await;

        let referee_creation = RefereeCreationDTO {
            name: "John Doe".to_string(),
            club: "Club A".to_string(),
        };
        let referee_dto = restinterface::create_referee(&referee_creation)
            .await
            .unwrap();

        let availabilities = fetch_availabilities_for_referee(referee_dto.id)
            .await
            .unwrap();
        assert!(availabilities.is_empty(), "Availabilities should be empty");

        let (_fixture_creation, fixture_dto) = restinterface::create_test_fixture().await;

        let _availability_declaration_result = declare_availability(fixture_dto.id, referee_dto.id)
            .await
            .unwrap();

        let availabilities = fetch_availabilities_for_referee(referee_dto.id)
            .await
            .unwrap();
        assert_eq!(
            availabilities.len(),
            1,
            "Availabilities should have 1 element"
        );
        assert_eq!(
            availabilities[0],
            fixture_dto.id.into(),
            "Fixture ID should match"
        );

        let _availability_withdrawal_result = withdraw_availability(fixture_dto.id, referee_dto.id)
            .await
            .unwrap();

        let availabilities = fetch_availabilities_for_referee(referee_dto.id)
            .await
            .unwrap();
        assert!(availabilities.is_empty(), "Availabilities should be empty");
    }

    async fn clear_tables() {
        let db_url = "postgres://postgres:postgres@localhost:5432/rustddd?application_name=rustddd&options=-c search_path%3Drustddd"; //std::env::var("DB_URL").expect("DB_URL not set");
        let connection_pool = PgPool::connect(&db_url).await.unwrap();

        sqlx::query("DELETE FROM rustddd.assignments")
            .execute(&connection_pool)
            .await
            .unwrap();

        sqlx::query("DELETE FROM rustddd.availabilities")
            .execute(&connection_pool)
            .await
            .unwrap();

        sqlx::query("DELETE FROM rustddd.fixtures")
            .execute(&connection_pool)
            .await
            .unwrap();

        sqlx::query("DELETE FROM rustddd.referees")
            .execute(&connection_pool)
            .await
            .unwrap();
    }
}
