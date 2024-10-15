use mockall::automock;

use crate::domain::aggregates::{fixture::FixtureId, referee::RefereeId};

#[automock(type Error = String; type TxCtx = ();)]
pub trait AvailabilityRepository {
    type Error;
    type TxCtx;

    async fn declare_availability(&self, fixture_id: FixtureId, referee_id: RefereeId, tx_ctx: &mut Self::TxCtx) -> Result<(), Self::Error>;
    async fn withdraw_availability(&self, fixture_id: FixtureId, referee_id: RefereeId, tx_ctx: &mut Self::TxCtx) -> Result<(), Self::Error>;
    async fn get_availabilities_for_referee(&self, referee_id: RefereeId, tx_ctx: &mut Self::TxCtx) -> Result<Vec<FixtureId>, Self::Error>;
}
