use sqlx::{Postgres, Transaction};
use uuid::Uuid;

use crate::domain::{
    aggregates::team::{Team, TeamId},
    repositories::team_repo::TeamRepository,
};

pub struct TeamRepositoryPg();

struct TeamDb {
    pub id: Uuid,
    pub name: String,
    pub club: String,
}

impl From<TeamDb> for Team {
    fn from(team: TeamDb) -> Self {
        Team::from_id(team.id, team.name, team.club)
    }
}

impl TeamRepositoryPg {
    pub fn new() -> Self {
        Self {}
    }
}

impl TeamRepository for TeamRepositoryPg {
    type Error = String;
    type TxCtx = Transaction<'static, Postgres>;

    async fn find_by_id(
        &self,
        id: &TeamId,
        tx_ctx: &mut Self::TxCtx,
    ) -> Result<Option<Team>, Self::Error> {
        let team: Option<TeamDb> = sqlx::query_as!(
            TeamDb,
            "SELECT team_id as id, name, club FROM rustddd.teams WHERE team_id = $1",
            id.0
        )
        .fetch_optional(&mut **tx_ctx)
        .await
        .map_err(|e| e.to_string())?;

        Ok(team.map(|t| t.into()))
    }

    async fn get_all(&self, tx_ctx: &mut Self::TxCtx) -> Result<Vec<Team>, Self::Error> {
        let teams: Vec<TeamDb> = sqlx::query_as!(
            TeamDb,
            "SELECT team_id as id, name, club FROM rustddd.teams"
        )
        .fetch_all(&mut **tx_ctx)
        .await
        .map_err(|e| e.to_string())?;

        Ok(teams.into_iter().map(|t| t.into()).collect())
    }

    async fn save(&self, team: &Team, tx_ctx: &mut Self::TxCtx) -> Result<(), Self::Error> {
        // NOTE: no upsert, because Team is not allowed to change after creation
        sqlx::query!(
            "INSERT INTO rustddd.teams (team_id, name, club) VALUES ($1, $2, $3)",
            team.id().0,
            team.name().to_string(),
            team.club().to_string()
        )
        .execute(&mut **tx_ctx)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }
}
