use microservices_shared::domain_ids::{FixtureId, RefereeId};
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

use crate::domain::repositories::availability_repo::AvailabilityRepository;

struct AvailabilityDb {
    pub fixture_id: Uuid,
}

pub struct AvailabilityRepositoryPg();

impl AvailabilityRepositoryPg {
    pub fn new() -> Self {
        Self {}
    }
}

impl AvailabilityRepository for AvailabilityRepositoryPg {
    type Error = String;
    type TxCtx = Transaction<'static, Postgres>;

    async fn declare_availability(
        &self,
        fixture_id: &FixtureId,
        referee_id: &RefereeId,
        tx_ctx: &mut Self::TxCtx,
    ) -> Result<(), Self::Error> {
        sqlx::query!(
            "INSERT INTO rustddd.availabilities (fixture_id, referee_id) VALUES ($1, $2)",
            fixture_id.0,
            referee_id.0
        )
        .execute(&mut **tx_ctx)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn withdraw_availability(
        &self,
        fixture_id: &FixtureId,
        referee_id: &RefereeId,
        tx_ctx: &mut Self::TxCtx,
    ) -> Result<(), Self::Error> {
        sqlx::query!(
            "DELETE FROM rustddd.availabilities WHERE fixture_id = $1 AND referee_id = $2",
            fixture_id.0,
            referee_id.0
        )
        .execute(&mut **tx_ctx)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn get_availabilities_for_referee(
        &self,
        referee_id: &RefereeId,
        tx_ctx: &mut Self::TxCtx,
    ) -> Result<Vec<FixtureId>, Self::Error> {
        let availabilities = sqlx::query_as!(
            AvailabilityDb,
            "SELECT fixture_id FROM rustddd.availabilities WHERE referee_id = $1",
            referee_id.0
        )
        .fetch_all(&mut **tx_ctx)
        .await
        .map_err(|e| e.to_string())?;

        Ok(availabilities
            .into_iter()
            .map(|a| a.fixture_id.into())
            .collect())
    }

    async fn is_available(
        &self,
        fixture_id: &FixtureId,
        referee_id: &RefereeId,
        tx_ctx: &mut Self::TxCtx,
    ) -> Result<bool, Self::Error> {
        let result = sqlx::query!(
            "SELECT COUNT(*) FROM rustddd.availabilities WHERE fixture_id = $1 AND referee_id = $2",
            fixture_id.0,
            referee_id.0
        )
        .fetch_one(&mut **tx_ctx)
        .await
        .map_err(|e| e.to_string())?;

        Ok(result.count.unwrap_or(0) > 0)
    }
}
