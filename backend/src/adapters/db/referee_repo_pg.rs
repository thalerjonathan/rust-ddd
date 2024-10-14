use sqlx::{Postgres, Transaction};
use uuid::Uuid;

use crate::domain::{
    aggregates::referee::{Referee, RefereeId},
    repositories::referee_repo::RefereeRepository,
};

pub struct RefereeRepositoryPg();

struct RefereeDb {
    pub id: Uuid,
    pub name: String,
    pub club: String,
}

impl From<RefereeDb> for Referee {
    fn from(referee: RefereeDb) -> Self {
        Referee::from_id(referee.id, referee.name, referee.club)
    }
}

impl RefereeRepositoryPg {
    pub fn new() -> Self {
        Self {}
    }
}

impl RefereeRepository for RefereeRepositoryPg {
    type Error = String;
    type TxCtx = Transaction<'static, Postgres>;

    async fn find_by_id(
        &self,
        id: &RefereeId,
        tx_ctx: &mut Self::TxCtx,
    ) -> Result<Option<Referee>, Self::Error> {
        let referee: Option<RefereeDb> = sqlx::query_as!(
            RefereeDb,
            "SELECT referee_id as id, name, club FROM rustddd.referees WHERE referee_id = $1",
            id.0
        )
        .fetch_optional(&mut **tx_ctx)
        .await
        .map_err(|e| e.to_string())?;

        Ok(referee.map(|r| Referee::from_id(r.id, r.name, r.club)))
    }

    async fn get_all(&self, tx_ctx: &mut Self::TxCtx) -> Result<Vec<Referee>, Self::Error> {
        let referees: Vec<RefereeDb> = sqlx::query_as!(
            RefereeDb,
            "SELECT referee_id as id, name, club FROM rustddd.referees"
        )
        .fetch_all(&mut **tx_ctx)
        .await
        .map_err(|e| e.to_string())?;

        Ok(referees
            .into_iter()
            .map(|r| Referee::from_id(r.id, r.name, r.club))
            .collect())
    }

    async fn save(&self, referee: &Referee, tx_ctx: &mut Self::TxCtx) -> Result<(), Self::Error> {
        // NOTE: we do an upsert, which is only updating the club field, because only this one is allowed to change
        let _result = sqlx::query!(
            "INSERT INTO rustddd.referees (referee_id, name, club) 
            VALUES ($1, $2, $3)
            ON CONFLICT (referee_id) DO UPDATE SET club = $3",
            referee.id().0,
            referee.name(),
            referee.club()
        )
        .execute(&mut **tx_ctx)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }
}
