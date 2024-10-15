use std::sync::Arc;

use axum::extract::{Path, State};
use shared::{FixtureIdDTO, RefereeIdDTO};

use super::shared::AppError;
use super::state::AppState;

pub async fn declare_availability_handler(
    State(state): State<Arc<AppState>>,
    Path(fixture_id): Path<FixtureIdDTO>,
    Path(referee_id): Path<RefereeIdDTO>,
) -> Result<(), AppError> {
    Err(AppError::from_error("declare_availability_handler not implemented"))
}

pub async fn withdraw_availability_handler(
    State(state): State<Arc<AppState>>,
    Path(fixture_id): Path<FixtureIdDTO>,
    Path(referee_id): Path<RefereeIdDTO>,
) -> Result<(), AppError> {
    Err(AppError::from_error("withdraw_availability_handler not implemented"))
}

pub async fn fetch_availabilities_for_referee_handler(
    State(state): State<Arc<AppState>>,
    Path(referee_id): Path<RefereeIdDTO>,
) -> Result<(), AppError> {
    Err(AppError::from_error("fetch_availabilities_for_referee_handler not implemented"))
}

#[cfg(test)]
mod availabilities_tests {
    use shared::{declare_availability, fetch_availabilities_for_referee, withdraw_availability, RefereeCreationDTO};
    use sqlx::PgPool;

    use crate::ports::rest::shared::create_test_fixture;

    #[tokio::test]
    async fn availability_scenario() {
        clear_tables().await;

        let referee_creation = RefereeCreationDTO {
            name: "John Doe".to_string(),
            club: "Club A".to_string(),
        };
        let referee_dto = shared::create_referee(&referee_creation).await.unwrap();

        let availabilities = fetch_availabilities_for_referee(referee_dto.id).await.unwrap();
        assert!(availabilities.is_empty(), "Availabilities should be empty");

        let (_fixture_creation, fixture_dto) = create_test_fixture().await;

        let availability_declaration_result = declare_availability(fixture_dto.id.into(), referee_dto.id.into()).await;
        assert!(availability_declaration_result.is_ok(), "Availability should be created");

        let availabilities = fetch_availabilities_for_referee(referee_dto.id).await.unwrap();
        assert_eq!(availabilities.len(), 1, "Availabilities should have 1 element");
        assert_eq!(availabilities[0], fixture_dto.id.into(), "Fixture ID should match");

        let availability_withdrawal_result = withdraw_availability(fixture_dto.id.into(), referee_dto.id.into()).await;
        assert!(availability_withdrawal_result.is_ok(), "Availability should be withdrawn");

        let availabilities = fetch_availabilities_for_referee(referee_dto.id).await.unwrap();
        assert!(availabilities.is_empty(), "Availabilities should be empty");
    }

    async fn clear_tables() {
        let db_url = std::env::var("DB_URL").expect("DB_URL not set");
        let connection_pool = PgPool::connect(&db_url).await.unwrap();

        sqlx::query("DELETE FROM rustddd.referees")
            .execute(&connection_pool)
            .await
            .unwrap();

        sqlx::query("DELETE FROM rustddd.fixtures")
            .execute(&connection_pool)
            .await
            .unwrap();

        sqlx::query("DELETE FROM rustddd.availabilities")
            .execute(&connection_pool)
            .await
            .unwrap();
    }
}
