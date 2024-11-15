use std::sync::Arc;

use axum::{
    extract::{Path, State},
    Extension, Json,
};
use log::info;
use microservices_shared::domain_event_repo::DomainEventRepositoryPg;
use opentelemetry::{
    trace::{Span, Tracer},
    KeyValue,
};
use restinterface::{TeamCreationDTO, TeamDTO, TeamIdDTO};
use shared::app_error::AppError;
use uuid::Uuid;

use crate::{
    adapters::db::team_repo_pg::TeamRepositoryPg,
    application::team_services::create_team,
    domain::{aggregates::team::Team, repositories::team_repo::TeamRepository},
    AppState,
};

impl From<Team> for TeamDTO {
    fn from(team: Team) -> Self {
        Self {
            id: team.id().into(),
            name: team.name().to_string(),
            club: team.club().to_string(),
        }
    }
}

pub async fn create_team_handler(
    State(state): State<Arc<AppState>>,
    Extension(instance_id): Extension<Uuid>,
    Json(team_creation): Json<TeamCreationDTO>,
) -> Result<Json<TeamDTO>, AppError> {
    info!("Creating team: {:?}", team_creation);
    let mut span = state.tracer.start("create_team");
    span.set_attribute(KeyValue::new("team_name", team_creation.name.clone()));
    span.set_attribute(KeyValue::new("team_club", team_creation.club.clone()));

    let mut tx = state
        .connection_pool
        .begin()
        .await
        .map_err(|e| AppError::from_error(&e.to_string()))?;

    let repo = TeamRepositoryPg::new();
    let domain_event_repo = DomainEventRepositoryPg::new(instance_id);

    let team = create_team(
        &team_creation.name,
        &team_creation.club,
        &repo,
        &domain_event_repo,
        &mut tx,
    )
    .await
    .map_err(|e| AppError::from_error(&e.to_string()))?;

    tx.commit()
        .await
        .map_err(|e| AppError::from_error(&e.to_string()))?;

    Ok(Json(team.into()))
}

pub async fn get_team_by_id_handler(
    State(state): State<Arc<AppState>>,
    Path(team_id): Path<TeamIdDTO>,
) -> Result<Json<Option<TeamDTO>>, AppError> {
    info!("Fetching team by id: {:?}", team_id.0);
    let mut span = state.tracer.start("get_team_by_id");
    span.set_attribute(KeyValue::new("team_id", team_id.0.to_string()));

    let mut tx = state.connection_pool.begin().await.unwrap();

    let repo = TeamRepositoryPg::new();

    let team = repo
        .find_by_id(team_id.into(), &mut tx)
        .await
        .map_err(|e| AppError::from_error(&e))?;

    Ok(Json(team.map(|t| t.into())))
}

pub async fn get_all_teams_handler(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<TeamDTO>>, AppError> {
    info!("Fetching all teams");
    let _span = state.tracer.start("get_all_teams");

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
    use restinterface::{create_team, fetch_team, fetch_teams, TeamCreationDTO};
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
        let db_url = "postgres://postgres:postgres@localhost:5435/teams?application_name=rustddd&options=-c search_path%3Drustddd";
        let pool = PgPool::connect(&db_url).await.unwrap();
        sqlx::query!("DELETE FROM rustddd.teams")
            .execute(&pool)
            .await
            .unwrap();
    }
}
