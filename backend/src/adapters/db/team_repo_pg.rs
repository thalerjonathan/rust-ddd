use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::domain::{
    aggregates::team::{Team, TeamId},
    repositories::team_repo::TeamRepository,
};

struct TeamDb {
    id: Uuid,
    name: String,
    club: String,
}

pub struct TeamRepositoryPg<'a> {
    pool: &'a Pool<Postgres>,
}

impl<'a> TeamRepositoryPg<'a> {
    pub fn new(pool: &'a Pool<Postgres>) -> Self {
        Self { pool }
    }
}

impl<'a> From<TeamDb> for Team {
    fn from(team: TeamDb) -> Self {
        Team::from_id(team.id, team.name, team.club)
    }
}

impl<'a> TeamRepository for TeamRepositoryPg<'a> {
    type Error = String;

    async fn find_by_id(&self, id: &TeamId) -> Result<Option<Team>, Self::Error> {
        let team: Option<TeamDb> = sqlx::query_as!(
            TeamDb,
            "SELECT team_id as id, name, club FROM rustddd.teams WHERE team_id = $1",
            id.0
        )
        .fetch_optional(self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(team.map(|t| t.into()))
    }

    async fn get_all(&self) -> Result<Vec<Team>, Self::Error> {
        let teams: Vec<TeamDb> = sqlx::query_as!(
            TeamDb,
            "SELECT team_id as id, name, club FROM rustddd.teams"
        )
        .fetch_all(self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(teams.into_iter().map(|t| t.into()).collect())
    }

    async fn save(&self, team: &Team) -> Result<(), Self::Error> {
        // NOTE: no upsert, because Team is not allowed to change after creation
        sqlx::query!(
            "INSERT INTO rustddd.teams (team_id, name, club) VALUES ($1, $2, $3)",
            team.id().0,
            team.name().to_string(),
            team.club().to_string()
        )
        .execute(self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }
}
