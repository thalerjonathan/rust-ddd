use microservices_shared::domain_ids::{FixtureId, RefereeId};
use mockall::automock;

#[allow(async_fn_in_trait)]
#[automock(type Error = String; type TxCtx = ();)]
pub trait AvailabilityRepository {
    type Error;
    type TxCtx;

    async fn declare_availability(
        &self,
        fixture: &FixtureId,
        referee: &RefereeId,
        tx_ctx: &mut Self::TxCtx,
    ) -> Result<(), Self::Error>;
    async fn withdraw_availability(
        &self,
        fixture: &FixtureId,
        referee: &RefereeId,
        tx_ctx: &mut Self::TxCtx,
    ) -> Result<(), Self::Error>;
    async fn get_availabilities_for_referee(
        &self,
        referee: &RefereeId,
        tx_ctx: &mut Self::TxCtx,
    ) -> Result<Vec<FixtureId>, Self::Error>;
    async fn is_available(
        &self,
        fixture: &FixtureId,
        referee: &RefereeId,
        tx_ctx: &mut Self::TxCtx,
    ) -> Result<bool, Self::Error>;
}
