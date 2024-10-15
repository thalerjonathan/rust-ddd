use std::sync::Arc;

use axum::{
    extract::{Path, State},
    Json,
};
use log::debug;
use shared::{TeamCreationDTO, TeamDTO};
use uuid::Uuid;

use crate::{
    adapters::db::team_repo_pg::TeamRepositoryPg,
    application::team_services::create_team,
    domain::repositories::team_repo::TeamRepository,
};

use super::{shared::AppError, state::AppState};

pub async fn create_team_handler(
    State(state): State<Arc<AppState>>,
    Json(team_creation): Json<TeamCreationDTO>,
) -> Result<Json<TeamDTO>, AppError> {
    debug!("Creating team: {:?}", team_creation);

    let mut tx = state.connection_pool.begin().await.unwrap();

    let repo = TeamRepositoryPg::new();

    let team = create_team(&team_creation.name, &team_creation.club, &repo, &mut tx)
        .await
        .map_err(|e| AppError::from_error(&e))?;

    tx.commit()
        .await
        .map_err(|e| AppError::from_error(&e.to_string()))?;

    Ok(Json(team.into()))
}

pub async fn get_team_by_id_handler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<Option<TeamDTO>>, AppError> {
    debug!("Fetching team by id: {:?}", id);

    let mut tx = state.connection_pool.begin().await.unwrap();

    let repo = TeamRepositoryPg::new();

    let team = repo
        .find_by_id(id.into(), &mut tx)
        .await
        .map_err(|e| AppError::from_error(&e))?;

    Ok(Json(team.map(|t| t.into())))
}

pub async fn get_all_teams_handler(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<TeamDTO>>, AppError> {
    debug!("Fetching all teams");

    let mut tx = state.connection_pool.begin().await.unwrap();

    let repo = TeamRepositoryPg::new();

    let teams = repo
        .get_all(&mut tx)
        .await
        .map_err(|e| AppError::from_error(&e))?;

    Ok(Json(teams.into_iter().map(|t| t.into()).collect()))
}

#[cfg(test)]
mod team_tests {
    use shared::{create_team, fetch_team, fetch_teams, TeamCreationDTO};
    use sqlx::PgPool;

    #[tokio::test]
    async fn given_empty_db_when_fetching_teams_then_empty_list_is_returned() {
        clear_tables().await;

        let teams = fetch_teams().await;
        assert!(teams.is_empty(), "Teams should be empty");
    }

    #[tokio::test]
    async fn given_empty_db_when_creating_team_then_team_is_returned() {
        clear_tables().await;

        let team_creation = TeamCreationDTO {
            name: "Team A".to_string(),
            club: "Club A".to_string(),
        };

        let team = create_team(&team_creation).await;
        assert!(team.is_ok(), "Team should be created");

        let team = team.unwrap();
        assert_eq!(team.name, "Team A", "Team name should be 'Team A'");
        assert_eq!(team.club, "Club A", "Team club should be 'Club A'");

        let teams = fetch_teams().await;
        assert_eq!(teams.len(), 1, "There should be 1 team");
        assert_eq!(teams[0].name, "Team A", "Team name should be 'Team A'");
        assert_eq!(teams[0].club, "Club A", "Team club should be 'Club A'");

        let fetched_team = fetch_team(team.id.into()).await;
        assert!(fetched_team.is_ok(), "Team should be fetched");

        let fetched_team = fetched_team.unwrap();
        assert_eq!(fetched_team.name, "Team A", "Team name should be 'Team A'");
        assert_eq!(fetched_team.club, "Club A", "Team club should be 'Club A'");
    }

    async fn clear_tables() {
        // NOTE: need to clear also fixtures, otherwise the foreign key constraint will prevent the deletion
        let db_url = std::env::var("DB_URL").expect("DB_URL not set");
        let pool = PgPool::connect(&db_url).await.unwrap();
        sqlx::query!("DELETE FROM rustddd.fixtures")
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query!("DELETE FROM rustddd.teams")
            .execute(&pool)
            .await
            .unwrap();
    }
}
