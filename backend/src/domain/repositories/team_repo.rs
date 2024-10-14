use crate::domain::aggregates::team::{Team, TeamId};

pub trait TeamRepository {
    type Error;
    type TxCtx;

    async fn find_by_id(
        &self,
        id: &TeamId,
        tx_ctx: &mut Self::TxCtx,
    ) -> Result<Option<Team>, Self::Error>;
    async fn get_all(&self, tx_ctx: &mut Self::TxCtx) -> Result<Vec<Team>, Self::Error>;

    async fn save(&self, team: &Team, tx_ctx: &mut Self::TxCtx) -> Result<(), Self::Error>;
}
