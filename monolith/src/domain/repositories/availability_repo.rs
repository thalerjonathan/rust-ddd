use mockall::automock;

use crate::domain::aggregates::{fixture::{Fixture, FixtureId}, referee::Referee};

#[automock(type Error = String; type TxCtx = ();)]
pub trait AvailabilityRepository {
    type Error;
    type TxCtx;

    async fn declare_availability(&self, fixture: &Fixture, referee: &Referee, tx_ctx: &mut Self::TxCtx) -> Result<(), Self::Error>;
    async fn withdraw_availability(&self, fixture: &Fixture, referee: &Referee, tx_ctx: &mut Self::TxCtx) -> Result<(), Self::Error>;
    async fn get_availabilities_for_referee(&self, referee: &Referee, tx_ctx: &mut Self::TxCtx) -> Result<Vec<FixtureId>, Self::Error>;
    async fn is_available(&self, fixture: &Fixture, referee: &Referee, tx_ctx: &mut Self::TxCtx) -> Result<bool, Self::Error>;
}
