use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::domain::{
    aggregates::referee::{Referee, RefereeId},
    repositories::referee_repo::RefereeRepository,
};

pub struct RefereeRepositoryPg<'a> {
    pool: &'a Pool<Postgres>,
}

struct RefereeDb {
    id: Uuid,
    name: String,
    club: String,
}

impl<'a> RefereeRepositoryPg<'a> {
    pub fn new(pool: &'a Pool<Postgres>) -> Self {
        Self { pool }
    }
}

impl<'a> RefereeRepository for RefereeRepositoryPg<'a> {
    type Error = String;

    async fn find_by_id(&self, id: &RefereeId) -> Result<Option<Referee>, Self::Error> {
        let referee: Option<RefereeDb> = sqlx::query_as!(
            RefereeDb,
            "SELECT referee_id as id, name, club FROM rustddd.referees WHERE referee_id = $1",
            id.0
        )
        .fetch_optional(self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(referee.map(|r| Referee::from_id(r.id, r.name, r.club)))
    }

    async fn get_all(&self) -> Result<Vec<Referee>, Self::Error> {
        let referees: Vec<RefereeDb> = sqlx::query_as!(
            RefereeDb,
            "SELECT referee_id as id, name, club FROM rustddd.referees"
        )
        .fetch_all(self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(referees
            .into_iter()
            .map(|r| Referee::from_id(r.id, r.name, r.club))
            .collect())
    }

    async fn save(&self, referee: &Referee) -> Result<(), Self::Error> {
        // NOTE: we do an upsert, which is only updating the club field, because only this one is allowed to change
        let _result = sqlx::query!(
            "INSERT INTO rustddd.referees (referee_id, name, club) 
            VALUES ($1, $2, $3)
            ON CONFLICT (referee_id) DO UPDATE SET club = $3",
            referee.id().0,
            referee.name(),
            referee.club()
        )
        .execute(self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }
}
