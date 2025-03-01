use std::sync::Arc;

use axum::{
    extract::{Path, State},
    Json,
};
use log::info;
use microservices_shared::{
    domain_event_repo::DomainEventRepositoryPg,
    resolvers::impls::{FixtureResolverImpl, RefereeResolverImpl},
};
use opentelemetry::{
    trace::{Span, Tracer},
    KeyValue,
};
use restinterface::{AssignmentDTO, AssignmentStagingDTO, FixtureIdDTO, RefereeIdDTO};
use shared::app_error::AppError;
use tokio::sync::Mutex;

use crate::{
    adapters::db::assignment_repo_pg::AssignmentRepositoryPg,
    application::assignment_services::{
        commit_assignments, remove_committed_assignment, remove_staged_assignment,
        stage_assignment, validate_assignments,
    },
    domain::repositories::assignment_repo::AssignmentRepository,
    AppState,
};

pub async fn fetch_assignments_handler(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<AssignmentDTO>>, AppError> {
    info!("Fetching assignments");
    let _span = state.tracer.start("fetch_assignments");

    let mut tx = state.connection_pool.begin().await.unwrap();
    let assignment_repo = AssignmentRepositoryPg::new();
    let assignments = assignment_repo.get_all(&mut tx).await.unwrap();

    // NOTE: read-only, therefore dont commit TX

    Ok(Json(assignments.into_iter().map(|a| a.into()).collect()))
}

pub async fn stage_assignment_handler(
    State(state): State<Arc<AppState>>,
    Json(assignment_staging): Json<AssignmentStagingDTO>,
) -> Result<Json<AssignmentDTO>, AppError> {
    info!("Staging assignment: {:?}", assignment_staging);
    let mut span = state.tracer.start("stage_assignment");
    span.set_attribute(KeyValue::new(
        "assignment_fixture_id",
        assignment_staging.fixture_id.to_string(),
    ));
    span.set_attribute(KeyValue::new(
        "assignment_referee_id",
        assignment_staging.referee_id.to_string(),
    ));
    span.set_attribute(KeyValue::new(
        "assignment_referee_role",
        format!("{:?}", assignment_staging.referee_role),
    ));

    // NOTE: not emitting domain events emitted here

    let mut tx = state.connection_pool.begin().await.unwrap();

    let redis_conn = state
        .redis_client
        .get_connection()
        .map_err(|e| AppError::from_error(&e.to_string()))?;
    let redis_conn_arc_mutex = Arc::new(Mutex::new(redis_conn));

    let assignment_repo = AssignmentRepositoryPg::new();
    let fixture_resolver = FixtureResolverImpl::new(redis_conn_arc_mutex.clone());
    let referee_resolver = RefereeResolverImpl::new(redis_conn_arc_mutex.clone());

    let result = stage_assignment(
        &assignment_staging,
        &assignment_repo,
        &fixture_resolver,
        &referee_resolver,
        &mut tx,
    )
    .await
    .unwrap();
    tx.commit().await.unwrap();

    Ok(Json(result))
}

pub async fn remove_staged_assignment_handler(
    State(state): State<Arc<AppState>>,
    Path((fixture_id, referee_id)): Path<(FixtureIdDTO, RefereeIdDTO)>,
) -> Result<Json<()>, AppError> {
    info!("Deleting assignment: {:?} {:?}", fixture_id, referee_id);
    let mut span = state.tracer.start("remove_staged_assignment");
    span.set_attribute(KeyValue::new(
        "assignment_fixture_id",
        fixture_id.to_string(),
    ));
    span.set_attribute(KeyValue::new(
        "assignment_referee_id",
        referee_id.to_string(),
    ));

    // NOTE: not emitting domain events emitted here

    let mut tx = state
        .connection_pool
        .begin()
        .await
        .map_err(|e| AppError::from_error(&e.to_string()))?;

    let assignment_repo = AssignmentRepositoryPg::new();
    let result = remove_staged_assignment(
        fixture_id.into(),
        referee_id.into(),
        &assignment_repo,
        &mut tx,
    )
    .await
    .unwrap();
    tx.commit().await.unwrap();

    Ok(Json(result))
}

pub async fn remove_committed_assignment_handler(
    State(state): State<Arc<AppState>>,
    Path((fixture_id, referee_id)): Path<(FixtureIdDTO, RefereeIdDTO)>,
) -> Result<Json<()>, AppError> {
    info!("Deleting assignment: {:?} {:?}", fixture_id, referee_id);
    let mut span = state.tracer.start("remove_committed_assignment");
    span.set_attribute(KeyValue::new(
        "assignment_fixture_id",
        fixture_id.to_string(),
    ));
    span.set_attribute(KeyValue::new(
        "assignment_referee_id",
        referee_id.to_string(),
    ));

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

    let assignment_repo = AssignmentRepositoryPg::new();
    let fixture_resolver = FixtureResolverImpl::new(redis_conn_arc_mutex.clone());
    let domain_event_repo = DomainEventRepositoryPg::new();

    let result = remove_committed_assignment(
        fixture_id.into(),
        referee_id.into(),
        &assignment_repo,
        &fixture_resolver,
        &domain_event_repo,
        &mut tx,
    )
    .await
    .unwrap();
    tx.commit().await.unwrap();

    Ok(Json(result))
}

pub async fn validate_assignments_handler(
    State(state): State<Arc<AppState>>,
) -> Result<String, AppError> {
    info!("Validating assignments");
    let _span = state.tracer.start("validate_assignments");

    // NOTE: no domain events emitted here

    let mut tx = state
        .connection_pool
        .begin()
        .await
        .map_err(|e| AppError::from_error(&e.to_string()))?;
    let assignment_repo = AssignmentRepositoryPg::new();
    let result = validate_assignments(&assignment_repo, &mut tx)
        .await
        .unwrap();

    // NOTE: read-only, therefore dont commit TX

    Ok(result)
}

pub async fn commit_assignments_handler(
    State(state): State<Arc<AppState>>,
) -> Result<String, AppError> {
    info!("Committing assignments");
    let _span = state.tracer.start("commit_assignments");

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

    let assignment_repo = AssignmentRepositoryPg::new();
    let fixture_resolver = FixtureResolverImpl::new(redis_conn_arc_mutex.clone());
    let referee_resolver = RefereeResolverImpl::new(redis_conn_arc_mutex.clone());
    let domain_event_repo = DomainEventRepositoryPg::new();

    let result = commit_assignments(
        &assignment_repo,
        &fixture_resolver,
        &referee_resolver,
        &domain_event_repo,
        &mut tx,
    )
    .await
    .unwrap();
    tx.commit().await.unwrap();

    Ok(result)
}

#[cfg(test)]
mod assignments_tests {
    use restinterface::{
        commit_assignments, create_test_fixture, fetch_assignments, fetch_fixture,
        remove_committed_assignment, remove_staged_assignment, stage_assignment,
        validate_assignments, AssignmentRefereeRoleDTO, AssignmentStagingDTO, AssignmentStatusDTO,
        RefereeCreationDTO,
    };
    use sqlx::PgPool;

    #[tokio::test]
    async fn assignment_scenario() {
        clear_tables().await;

        // if empty, then fetch_assignments() should return an empty list
        let assignments = fetch_assignments().await;
        assert!(assignments.is_empty(), "Assignments should be empty");

        let (_fixture_creation, fixture_dto) = create_test_fixture().await;
        let referee_creation = RefereeCreationDTO {
            name: "John Doe".to_string(),
            club: "Club A".to_string(),
        };
        let first_referee_dto = restinterface::create_referee(&referee_creation)
            .await
            .unwrap();

        let second_referee_creation = RefereeCreationDTO {
            name: "Jane Smith".to_string(),
            club: "Club B".to_string(),
        };
        let second_referee_dto = restinterface::create_referee(&second_referee_creation)
            .await
            .unwrap();

        let first_assignment_creation = AssignmentStagingDTO {
            fixture_id: fixture_dto.id,
            referee_id: first_referee_dto.id,
            referee_role: AssignmentRefereeRoleDTO::First,
        };
        let first_ref_assignment_dto = stage_assignment(&first_assignment_creation).await.unwrap();
        assert_eq!(
            first_ref_assignment_dto.status,
            AssignmentStatusDTO::Staged,
            "Assignment status should be staged"
        );
        assert_eq!(
            first_ref_assignment_dto.fixture_id, fixture_dto.id,
            "Assignment fixture_id should be the same"
        );
        assert_eq!(
            first_ref_assignment_dto.referee_id, first_referee_dto.id,
            "Assignment referee_id should be the same"
        );

        let second_assignment_creation = AssignmentStagingDTO {
            fixture_id: fixture_dto.id,
            referee_id: second_referee_dto.id,
            referee_role: AssignmentRefereeRoleDTO::Second,
        };
        let second_ref_assignment_dto =
            stage_assignment(&second_assignment_creation).await.unwrap();
        assert_eq!(
            second_ref_assignment_dto.status,
            AssignmentStatusDTO::Staged,
            "Assignment status should be staged"
        );
        assert_eq!(
            second_ref_assignment_dto.fixture_id, fixture_dto.id,
            "Assignment fixture_id should be the same"
        );
        assert_eq!(
            second_ref_assignment_dto.referee_id, second_referee_dto.id,
            "Assignment referee_id should be the same"
        );

        let assignments = fetch_assignments().await;
        assert_eq!(assignments.len(), 2, "Assignments should have 2 elements");
        assert_eq!(
            assignments[0].status,
            AssignmentStatusDTO::Staged,
            "Assignment status should be staged"
        );
        assert_eq!(
            assignments[1].status,
            AssignmentStatusDTO::Staged,
            "Assignment status should be staged"
        );
        assert_eq!(
            assignments[0], first_ref_assignment_dto,
            "Assignment should be the same"
        );
        assert_eq!(
            assignments[1], second_ref_assignment_dto,
            "Assignment should be the same"
        );

        // delete the first assignment
        remove_staged_assignment(&first_ref_assignment_dto)
            .await
            .unwrap();

        let assignments = fetch_assignments().await;
        assert_eq!(assignments.len(), 1, "Assignments should have 1 element");
        assert_eq!(
            assignments[0].status,
            AssignmentStatusDTO::Staged,
            "Assignment status should be staged"
        );
        assert_eq!(
            assignments[0], second_ref_assignment_dto,
            "Assignment should be the same"
        );

        let first_ref_assignment_dto = stage_assignment(&first_assignment_creation).await.unwrap();

        // no conflicts, so validate_assignments() should return an empty string
        let result = validate_assignments().await.unwrap();
        assert_eq!(
            result, "Assignments validated",
            "Validate assignments should return an empty string"
        );

        // committing the assignments should work
        let result = commit_assignments().await.unwrap();
        assert_eq!(
            result, "Assignments committed",
            "Commit assignments should return an empty string"
        );

        tokio::time::sleep(std::time::Duration::from_secs(1)).await;

        // fetch the assignments again, all should be in status committed
        let assignments = fetch_assignments().await;
        assert_eq!(assignments.len(), 2, "Assignments should have 2 elements");
        assert_eq!(
            assignments[0].status,
            AssignmentStatusDTO::Committed,
            "Assignment status should be committed"
        );
        assert_eq!(
            assignments[1].status,
            AssignmentStatusDTO::Committed,
            "Assignment status should be committed"
        );

        // when refetching the fixture, the referees should be set
        let fixture_dto = fetch_fixture(fixture_dto.id).await.unwrap();
        assert_eq!(
            fixture_dto.first_referee,
            Some(first_referee_dto),
            "Fixture first_referee should be the same"
        );
        assert_eq!(
            fixture_dto.second_referee,
            Some(second_referee_dto.clone()),
            "Fixture second_referee should be the same"
        );

        remove_committed_assignment(&first_ref_assignment_dto)
            .await
            .unwrap();

        let assignments = fetch_assignments().await;
        assert_eq!(assignments.len(), 1, "Assignments should have 1 element");
        assert_eq!(
            assignments[0].status,
            AssignmentStatusDTO::Committed,
            "Assignment status should be committed"
        );

        // when refetching the fixture, the referees should be set
        let fixture_dto = fetch_fixture(fixture_dto.id).await.unwrap();
        assert_eq!(
            fixture_dto.first_referee, None,
            "Fixture first_referee should be None"
        );
        assert_eq!(
            fixture_dto.second_referee,
            Some(second_referee_dto),
            "Fixture second_referee should be the same"
        );
    }

    // NOTE: deleting tables in other microservices is a fundamental violation of Microservices architecture, however its the simplest way to clear the db for tests
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
