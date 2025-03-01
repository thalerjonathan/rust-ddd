use microservices_shared::domain_ids::{FixtureId, RefereeId};

use crate::domain::aggregates::assignment::Assignment;

#[allow(async_fn_in_trait)]
pub trait AssignmentRepository {
    type Error;
    type TxCtx;

    async fn get_all(&self, tx_ctx: &mut Self::TxCtx) -> Result<Vec<Assignment>, Self::Error>;
    async fn find_by_fixture_and_referee(
        &self,
        fixture_id: FixtureId,
        referee_id: RefereeId,
        tx_ctx: &mut Self::TxCtx,
    ) -> Result<Option<Assignment>, Self::Error>;

    async fn delete(
        &self,
        assignment: &Assignment,
        tx_ctx: &mut Self::TxCtx,
    ) -> Result<(), Self::Error>;
    async fn save(
        &self,
        assignment: &Assignment,
        tx_ctx: &mut Self::TxCtx,
    ) -> Result<(), Self::Error>;

    async fn find_all_staged(
        &self,
        tx_ctx: &mut Self::TxCtx,
    ) -> Result<Vec<Assignment>, Self::Error>;
}
