use crate::domain::aggregates::referee::{Referee, RefereeId};
use async_trait::async_trait;

#[async_trait]
pub trait RefereeRepository {
    type Error;
    type TxCtx;

    async fn find_by_id(
        &self,
        referee_id: RefereeId,
        tx_ctx: &mut Self::TxCtx,
    ) -> Result<Option<Referee>, Self::Error>;
    async fn get_all(&self, tx_ctx: &mut Self::TxCtx) -> Result<Vec<Referee>, Self::Error>;

    async fn save(&self, referee: &Referee, tx_ctx: &mut Self::TxCtx) -> Result<(), Self::Error>;
}
