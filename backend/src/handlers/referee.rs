use std::sync::Arc;

use axum::{
    extract::{Path, State},
    Json,
};
use log::{debug, info};
use shared::{RefereeCreationDTO, RefereeDTO};
use uuid::Uuid;

use super::shared::AppError;
use super::state::AppState;

pub async fn create_referee(
    State(state): State<Arc<AppState>>,
    Json(ref_creation): Json<RefereeCreationDTO>,
) -> Result<Json<RefereeDTO>, AppError> {
    info!("Creating referee: {}", ref_creation.name);
    let referee = RefereeDTO {
        id: Uuid::new_v4(),
        name: ref_creation.name,
        club: ref_creation.club,
    };

    let result = sqlx::query!(
        "INSERT INTO rustddd.referees (referee_id, name, club) VALUES ($1, $2, $3)",
        referee.id,
        referee.name,
        referee.club
    )
    .execute(&state.connection_pool)
    .await
    .map_err(|e| AppError::from_error(&e.to_string()))?;

    debug!("Referee created: {}", result.rows_affected());

    Ok(Json(referee))
}

pub async fn get_referee_by_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<Option<RefereeDTO>>, AppError> {
    info!("Getting referee by id: {}", id);

    let referee = sqlx::query_as!(
        RefereeDTO,
        "SELECT referee_id as id, name, club FROM rustddd.referees WHERE referee_id = $1",
        id
    )
    .fetch_optional(&state.connection_pool)
    .await
    .map_err(|e| AppError::from_error(&e.to_string()))?;

    Ok(Json(referee))
}

pub async fn get_all_referees(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<RefereeDTO>>, AppError> {
    info!("Getting all referees");

    let referees = sqlx::query_as!(
        RefereeDTO,
        "SELECT referee_id as id, name, club FROM rustddd.referees"
    )
    .fetch_all(&state.connection_pool)
    .await
    .map_err(|e| AppError::from_error(&e.to_string()))?;

    Ok(Json(referees))
}

pub async fn update_referee_club(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(club): Json<String>,
) -> Result<Json<String>, AppError> {
    info!("Updating referee club: {}", id);

    let result = sqlx::query!(
        "UPDATE rustddd.referees SET club = $1 WHERE referee_id = $2",
        club,
        id
    )
    .execute(&state.connection_pool)
    .await
    .map_err(|e| AppError::from_error(&e.to_string()))?;

    debug!("Referee club changed: {}", result.rows_affected());

    Ok(Json(club))
}
