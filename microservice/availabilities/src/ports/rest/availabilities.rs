use std::sync::Arc;

use axum::extract::{Path, State};
use axum::{Extension, Json};
use log::{error, info};
use microservices_shared::domain_event_repo::DomainEventRepositoryPg;
use microservices_shared::resolvers::impls::{FixtureResolverImpl, RefereeResolverImpl};
use restinterface::{FixtureIdDTO, RefereeIdDTO};
use shared::app_error::AppError;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::adapters::db::availability_repo_pg::AvailabilityRepositoryPg;
use crate::application::availability_services::{
    declare_availability, get_availabilities_for_referee, withdraw_availability,
};
use crate::AppState;
use opentelemetry::{
    trace::{Span, Tracer},
    KeyValue,
};
pub async fn declare_availability_handler(
    State(state): State<Arc<AppState>>,
    Extension(instance_id): Extension<Uuid>,
    Path((fixture_id, referee_id)): Path<(FixtureIdDTO, RefereeIdDTO)>,
) -> Result<Json<()>, AppError> {
    info!(
        "Declaring availability for fixture: {:?} and referee: {:?}",
        fixture_id, referee_id
    );
    let mut span = state.tracer.start("declare_availability");
    span.set_attribute(KeyValue::new("fixture_id", fixture_id.to_string()));
    span.set_attribute(KeyValue::new("referee_id", referee_id.to_string()));

    let mut tx = state.connection_pool.begin().await.map_err(|e| {
        error!("Error beginning transaction: {:?}", e);
        AppError::from_error(&e.to_string())
    })?;

    let redis_conn = state
        .redis_client
        .get_connection()
        .map_err(|e| AppError::from_error(&e.to_string()))?;
    let redis_conn_arc_mutex = Arc::new(Mutex::new(redis_conn));

    let fixture_resolver = FixtureResolverImpl::new(redis_conn_arc_mutex.clone());
    let referee_resolver = RefereeResolverImpl::new(redis_conn_arc_mutex.clone());
    let availability_repo = AvailabilityRepositoryPg::new();
    let domain_event_repo = DomainEventRepositoryPg::new(instance_id);

    declare_availability(
        fixture_id.into(),
        referee_id.into(),
        &fixture_resolver,
        &referee_resolver,
        &availability_repo,
        &domain_event_repo,
        &mut tx,
    )
    .await
    .map_err(|e| AppError::from_error(&e.to_string()))?;

    tx.commit()
        .await
        .map_err(|e| AppError::from_error(&e.to_string()))?;

    Ok(Json(()))
}

pub async fn withdraw_availability_handler(
    State(state): State<Arc<AppState>>,
    Extension(instance_id): Extension<Uuid>,
    Path((fixture_id, referee_id)): Path<(FixtureIdDTO, RefereeIdDTO)>,
) -> Result<Json<()>, AppError> {
    info!(
        "Withdrawing availability for fixture: {:?} and referee: {:?}",
        fixture_id, referee_id
    );
    let mut span = state.tracer.start("withdraw_availability");
    span.set_attribute(KeyValue::new("fixture_id", fixture_id.to_string()));
    span.set_attribute(KeyValue::new("referee_id", referee_id.to_string()));

    let mut tx = state.connection_pool.begin().await.map_err(|e| {
        error!("Error beginning transaction: {:?}", e);
        AppError::from_error(&e.to_string())
    })?;

    let redis_conn = state
        .redis_client
        .get_connection()
        .map_err(|e| AppError::from_error(&e.to_string()))?;
    let redis_conn_arc_mutex = Arc::new(Mutex::new(redis_conn));

    let fixture_resolver = FixtureResolverImpl::new(redis_conn_arc_mutex.clone());
    let referee_resolver = RefereeResolverImpl::new(redis_conn_arc_mutex.clone());
    let availability_repo = AvailabilityRepositoryPg::new();
    let domain_event_repo = DomainEventRepositoryPg::new(instance_id);

    withdraw_availability(
        fixture_id.into(),
        referee_id.into(),
        &fixture_resolver,
        &referee_resolver,
        &availability_repo,
        &domain_event_repo,
        &mut tx,
    )
    .await
    .map_err(|e| AppError::from_error(&e.to_string()))?;

    tx.commit()
        .await
        .map_err(|e| AppError::from_error(&e.to_string()))?;

    Ok(Json(()))
}

pub async fn fetch_availabilities_for_referee_handler(
    State(state): State<Arc<AppState>>,
    Path(referee_id): Path<RefereeIdDTO>,
) -> Result<Json<Vec<FixtureIdDTO>>, AppError> {
    info!("Fetching availabilities for referee: {:?}", referee_id);
    let mut span = state.tracer.start("fetch_availabilities_for_referee");
    span.set_attribute(KeyValue::new("referee_id", referee_id.to_string()));

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
        let fixtures_db_url = "postgres://postgres:postgres@localhost:5436/fixtures?application_name=rustddd&options=-c search_path%3Drustddd";
        let assignments_db_url = "postgres://postgres:postgres@localhost:5438/assignments?application_name=rustddd&options=-c search_path%3Drustddd";
        let availabilities_db_url = "postgres://postgres:postgres@localhost:5437/availabilities?application_name=rustddd&options=-c search_path%3Drustddd";
        let referees_db_url = "postgres://postgres:postgres@localhost:5433/referees?application_name=rustddd&options=-c search_path%3Drustddd";

        let availabilities_pool = PgPool::connect(&availabilities_db_url).await.unwrap();
        let fixtures_pool = PgPool::connect(&fixtures_db_url).await.unwrap();
        let assignments_pool = PgPool::connect(&assignments_db_url).await.unwrap();
        let referees_pool = PgPool::connect(&referees_db_url).await.unwrap();
        sqlx::query("DELETE FROM rustddd.assignments")
            .execute(&assignments_pool)
            .await
            .unwrap();

        sqlx::query("DELETE FROM rustddd.availabilities")
            .execute(&availabilities_pool)
            .await
            .unwrap();

        sqlx::query("DELETE FROM rustddd.fixtures")
            .execute(&fixtures_pool)
            .await
            .unwrap();

        sqlx::query("DELETE FROM rustddd.referees")
            .execute(&referees_pool)
            .await
            .unwrap();
    }
}
