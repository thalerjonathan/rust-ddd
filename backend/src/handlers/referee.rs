use std::sync::Arc;

use axum::{
    extract::{Path, State},
    Json,
};
use log::{debug, info};
use shared::RefereeDTO;
use uuid::Uuid;

use super::shared::AppError;
use super::state::AppState;

pub async fn create_referee(
    State(state): State<Arc<AppState>>,
    Json(name): Json<String>,
) -> Result<Json<RefereeDTO>, AppError> {
    info!("Creating referee: {}", name);
    let referee = RefereeDTO {
        id: Uuid::new_v4(),
        name,
    };

    let result = sqlx::query!(
        "INSERT INTO rustddd.referees (referee_id, name) VALUES ($1, $2)",
        referee.id,
        referee.name
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
        "SELECT referee_id as id, name FROM rustddd.referees WHERE referee_id = $1",
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
        "SELECT referee_id as id, name FROM rustddd.referees"
    )
    .fetch_all(&state.connection_pool)
    .await
    .map_err(|e| AppError::from_error(&e.to_string()))?;

    Ok(Json(referees))
}
