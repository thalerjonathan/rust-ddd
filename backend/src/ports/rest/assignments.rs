use std::sync::Arc;

use axum::{extract::State, Json};
use shared::{AssignmentCreationDTO, AssignmentDTO};

use super::{shared::AppError, state::AppState};

pub async fn fetch_assignments_handler(
    State(_state): State<Arc<AppState>>,
) -> Result<Json<Vec<AssignmentDTO>>, AppError> {
    Err(AppError::from_error(
        "fetch_assignments_handler not implemented",
    ))
}

pub async fn stage_assignment_handler(
    State(_state): State<Arc<AppState>>,
    Json(_assignments): Json<Vec<AssignmentCreationDTO>>,
) -> Result<Json<Vec<AssignmentDTO>>, AppError> {
    Err(AppError::from_error(
        "stage_assignment_handler not implemented",
    ))
}

pub async fn validate_assignments_handler(
    State(_state): State<Arc<AppState>>,
) -> Result<Json<Vec<AssignmentDTO>>, AppError> {
    Err(AppError::from_error(
        "validate_assignments_handler not implemented",
    ))
}

pub async fn commit_assignments_handler(
    State(_state): State<Arc<AppState>>,
) -> Result<Json<Vec<AssignmentDTO>>, AppError> {
    Err(AppError::from_error(
        "commit_assignments_handler not implemented",
    ))
}

#[cfg(test)]
mod assignments_tests {
    use shared::{
        commit_assignments, fetch_assignments, fetch_fixture, stage_assignment,
        validate_assignments, AssignmentCreationDTO, AssignmentRefereeRoleDTO, AssignmentStatusDTO,
        RefereeCreationDTO,
    };
    use sqlx::PgPool;

    use crate::ports::rest::shared::create_test_fixture;

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
        let first_referee_dto = shared::create_referee(&referee_creation).await.unwrap();

        let second_referee_creation = RefereeCreationDTO {
            name: "Jane Smith".to_string(),
            club: "Club B".to_string(),
        };
        let second_referee_dto = shared::create_referee(&second_referee_creation)
            .await
            .unwrap();

        let first_assignment_creation = AssignmentCreationDTO {
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

        let second_assignment_creation = AssignmentCreationDTO {
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

        // no conflicts, so validate_assignments() should return an empty string
        let result = validate_assignments().await.unwrap();
        assert_eq!(
            result, "",
            "Validate assignments should return an empty string"
        );

        // committing the assignments should work
        let result = commit_assignments().await.unwrap();
        assert_eq!(
            result, "",
            "Commit assignments should return an empty string"
        );

        // fetch the assignments again, all should be in status committed
        let assignments = fetch_assignments().await;
        assert_eq!(assignments.len(), 2, "Assignments should have 2 elements");
        assert_eq!(
            assignments[0].status,
            AssignmentStatusDTO::Committed,
            "Assignment status should be committed"
        );
        assert_eq!(
            assignments[0].fixture_id, fixture_dto.id,
            "Assignment fixture_id should be the same"
        );
        assert_eq!(
            assignments[0].referee_id, first_referee_dto.id,
            "Assignment referee_id should be the same"
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
            Some(second_referee_dto),
            "Fixture second_referee should be the same"
        );
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
