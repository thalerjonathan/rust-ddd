use mockall::automock;

use crate::domain::aggregates::team::{Team, TeamId};

#[allow(async_fn_in_trait)]
#[automock(type Error = String; type TxCtx = ();)]
pub trait TeamRepository {
    type Error;
    type TxCtx;

    async fn find_by_id(
        &self,
        team_id: TeamId,
        tx_ctx: &mut Self::TxCtx,
    ) -> Result<Option<Team>, Self::Error>;
    async fn get_all(&self, tx_ctx: &mut Self::TxCtx) -> Result<Vec<Team>, Self::Error>;

    async fn save(&self, team: &Team, tx_ctx: &mut Self::TxCtx) -> Result<(), Self::Error>;
}
