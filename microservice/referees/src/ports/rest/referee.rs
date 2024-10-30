use std::sync::Arc;

use axum::{
    extract::{Path, State},
    Json,
};
use log::{debug, error};
use restinterface::{app_error::AppError, RefereeCreationDTO, RefereeDTO, RefereeIdDTO};

use crate::{
    adapters::db::referee_repo_pg::RefereeRepositoryPg,
    application,
    domain::{aggregates::referee::Referee, repositories::referee_repo::RefereeRepository},
    AppState,
};

impl From<Referee> for RefereeDTO {
    fn from(referee: Referee) -> Self {
        RefereeDTO {
            id: referee.id().into(),
            name: referee.name().to_string(),
            club: referee.club().to_string(),
        }
    }
}

pub async fn create_referee_handler(
    State(state): State<Arc<AppState>>,
    Json(ref_creation): Json<RefereeCreationDTO>,
) -> Result<Json<RefereeDTO>, AppError> {
    debug!("Creating referee: {:?}", ref_creation);

    let referee = microservices_shared::domain_events::run_domain_event_publisher_transactional(
        &state.domain_event_publisher,
        async {
            let mut tx = state.connection_pool.begin().await.map_err(|e| {
                error!("Error beginning transaction: {:?}", e);
                e.to_string()
            })?;

            let repo = RefereeRepositoryPg::new();

            let referee = application::referee_services::create_referee(
                &ref_creation.name,
                &ref_creation.club,
                &repo,
                &state.domain_event_publisher,
                &mut tx,
            )
            .await
            .map_err(|e| e.to_string())?;

            tx.commit().await.map_err(|e| e.to_string())?;

            Ok(referee)
        },
    )
    .await
    .map_err(|e| AppError::from_error(&e.to_string()))?;

    let referee = RefereeDTO::from(referee);

    debug!("Referee created: {:?}", referee);

    Ok(Json(referee))
}

pub async fn get_referee_by_id_handler(
    State(state): State<Arc<AppState>>,
    Path(referee_id): Path<RefereeIdDTO>,
) -> Result<Json<Option<RefereeDTO>>, AppError> {
    debug!("Getting referee by id: {}", referee_id.0);

    let mut tx = state
        .connection_pool
        .begin()
        .await
        .map_err(|e| AppError::from_error(&e.to_string()))?;

    let repo = RefereeRepositoryPg::new();

    // NOTE: we are not using an application service here, because the logic is so simple
    let referee = repo
        .find_by_id(referee_id.into(), &mut tx)
        .await
        .map_err(|e| AppError::from_error(&e.to_string()))?;

    debug!("Referee found: {:?}", referee);

    Ok(Json(referee.map(|r| r.into())))
}

pub async fn get_all_referees_handler(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<RefereeDTO>>, AppError> {
    debug!("Getting all referees");

    let mut tx = state
        .connection_pool
        .begin()
        .await
        .map_err(|e| AppError::from_error(&e.to_string()))?;

    let repo = RefereeRepositoryPg::new();

    // NOTE: we are not using an application service here, because the logic is so simple
    let referees = repo
        .get_all(&mut tx)
        .await
        .map_err(|e| AppError::from_error(&e.to_string()))?;

    Ok(Json(referees.into_iter().map(|r| r.into()).collect()))
}

pub async fn update_referee_club_handler(
    State(state): State<Arc<AppState>>,
    Path(referee_id): Path<RefereeIdDTO>,
    Json(club): Json<String>,
) -> Result<Json<String>, AppError> {
    debug!("Updating referee club: {}", referee_id.0);

    let club = microservices_shared::domain_events::run_domain_event_publisher_transactional(
        &state.domain_event_publisher,
        async {
            let mut tx = state.connection_pool.begin().await.map_err(|e| {
                error!("Error beginning transaction: {:?}", e);
                e.to_string()
            })?;

            let repo = RefereeRepositoryPg::new();

            let _ = application::referee_services::update_referee_club(
                referee_id.into(),
                &club,
                &repo,
                &state.domain_event_publisher,
                &mut tx,
            )
            .await
            .map_err(|e| e.to_string())?;

            tx.commit().await.map_err(|e| e.to_string())?;

            Ok(club)
        },
    )
    .await
    .map_err(|e| AppError::from_error(&e.to_string()))?;

    Ok(Json(club))
}

#[cfg(test)]
mod referee_tests {
    use restinterface::{change_referee_club, fetch_referees, RefereeCreationDTO};
    use sqlx::PgPool;

    #[tokio::test]
    async fn given_empty_db_when_fetching_referees_then_empty_list_is_returned() {
        clear_referee_table().await;

        let referees = fetch_referees().await;
        assert!(referees.is_empty(), "Referees should be empty");
    }

    #[tokio::test]
    async fn given_empty_db_when_creating_referee_then_referee_is_returned() {
        clear_referee_table().await;

        let referee_creation = RefereeCreationDTO {
            name: "John Doe".to_string(),
            club: "Club A".to_string(),
        };

        let referee_dto = restinterface::create_referee(&referee_creation).await;
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
        clear_referee_table().await;

        let referee_creation = RefereeCreationDTO {
            name: "John Doe".to_string(),
            club: "Club A".to_string(),
        };

        let referee_dto = restinterface::create_referee(&referee_creation).await;
        assert!(referee_dto.is_ok(), "Referee should be created");

        let referee_dto = referee_dto.unwrap();
        let updated_club = "Club B".to_string();
        let updated_referee_dto = change_referee_club(referee_dto.id.into(), &updated_club).await;
        assert!(
            updated_referee_dto.is_ok(),
            "Referee club should be updated"
        );

        let referee_dto = restinterface::fetch_referee(referee_dto.id.into()).await;
        assert_eq!(
            referee_dto.unwrap().club,
            updated_club,
            "Referee club should be updated"
        );
    }

    async fn clear_referee_table() {
        let db_url = "postgres://postgres:postgres@localhost:5433/referees?application_name=rustddd&options=-c search_path%3Drustddd";
        let connection_pool = PgPool::connect(&db_url).await.unwrap();

        sqlx::query("DELETE FROM rustddd.referees")
            .execute(&connection_pool)
            .await
            .unwrap();
    }
}
