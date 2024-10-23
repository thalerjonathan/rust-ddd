use chrono::{DateTime, Utc};
use microservices_shared::domain_ids::{FixtureId, TeamId, VenueId};
use mockall::automock;

use crate::domain::aggregates::fixture::Fixture;

#[allow(async_fn_in_trait)]
#[automock(type Error = String; type TxCtx = ();)]
pub trait FixtureRepository {
    type Error;
    type TxCtx;

    async fn find_by_id(
        &self,
        fixture_id: FixtureId,
        tx_ctx: &mut Self::TxCtx,
    ) -> Result<Option<Fixture>, Self::Error>;
    async fn get_all(&self, tx_ctx: &mut Self::TxCtx) -> Result<Vec<Fixture>, Self::Error>;
    async fn find_by_day_and_venue(
        &self,
        date: &DateTime<Utc>,
        venue_id: VenueId,
        tx_ctx: &mut Self::TxCtx,
    ) -> Result<Vec<Fixture>, Self::Error>;
    async fn find_by_day_and_team(
        &self,
        date: &DateTime<Utc>,
        team_id: TeamId,
        tx_ctx: &mut Self::TxCtx,
    ) -> Result<Vec<Fixture>, Self::Error>;
    async fn save(&self, fixture: &Fixture, tx_ctx: &mut Self::TxCtx) -> Result<(), Self::Error>;
}
