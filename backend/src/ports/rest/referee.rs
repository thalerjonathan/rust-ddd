use std::sync::Arc;

use axum::{
    extract::{Path, State},
    Json,
};
use log::debug;
use shared::{RefereeCreationDTO, RefereeDTO};
use uuid::Uuid;

use crate::{
    adapters::db::referee_repo_pg::RefereeRepositoryPg,
    application,
    domain::{
        aggregates::referee::{Referee, RefereeId},
        repositories::referee_repo::RefereeRepository,
    },
};

use super::shared::AppError;
use super::state::AppState;

impl From<Referee> for RefereeDTO {
    fn from(referee: Referee) -> Self {
        RefereeDTO {
            id: referee.id().0,
            name: referee.name().to_string(),
            club: referee.club().to_string(),
        }
    }
}

pub async fn create_referee(
    State(state): State<Arc<AppState>>,
    Json(ref_creation): Json<RefereeCreationDTO>,
) -> Result<Json<RefereeDTO>, AppError> {
    debug!("Creating referee: {:?}", ref_creation);

    let repo = RefereeRepositoryPg::new(&state.connection_pool);

    let referee = application::referee_services::create_referee(
        &ref_creation.name,
        &ref_creation.club,
        &repo,
    )
    .await
    .map_err(|e| AppError::from_error(&e.to_string()))?;

    let referee = RefereeDTO {
        id: referee.id().0,
        name: referee.name().to_string(),
        club: referee.club().to_string(),
    };

    debug!("Referee created: {:?}", referee);

    Ok(Json(referee))
}

pub async fn get_referee_by_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<Option<RefereeDTO>>, AppError> {
    debug!("Getting referee by id: {}", id);

    let repo = RefereeRepositoryPg::new(&state.connection_pool);

    // NOTE: we are not using an application service here, because the logic is so simple
    let referee = repo
        .find_by_id(&RefereeId::from(id))
        .await
        .map_err(|e| AppError::from_error(&e.to_string()))?;

    debug!("Referee found: {:?}", referee);

    Ok(Json(referee.map(|r| r.into())))
}

pub async fn get_all_referees(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<RefereeDTO>>, AppError> {
    debug!("Getting all referees");

    let repo = RefereeRepositoryPg::new(&state.connection_pool);

    // NOTE: we are not using an application service here, because the logic is so simple
    let referees = repo
        .get_all()
        .await
        .map_err(|e| AppError::from_error(&e.to_string()))?;

    Ok(Json(referees.into_iter().map(|r| r.into()).collect()))
}

pub async fn update_referee_club(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(club): Json<String>,
) -> Result<Json<String>, AppError> {
    debug!("Updating referee club: {}", id);

    let repo = RefereeRepositoryPg::new(&state.connection_pool);

    let result = application::referee_services::update_referee_club(&id, &club, &repo)
        .await
        .map_err(|e| AppError::from_error(&e.to_string()))?;

    debug!("Referee club changed: {:?}", result);

    Ok(Json(club))
}
