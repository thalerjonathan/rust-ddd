use crate::domain::aggregates::venue::{Venue, VenueId};

pub trait VenueRepository {
    type Error;
    type TxCtx;

    async fn find_by_id(
        &self,
        id: &VenueId,
        tx_ctx: &mut Self::TxCtx,
    ) -> Result<Option<Venue>, Self::Error>;
    async fn get_all(&self, tx_ctx: &mut Self::TxCtx) -> Result<Vec<Venue>, Self::Error>;

    async fn save(&self, venue: &Venue, tx_ctx: &mut Self::TxCtx) -> Result<(), Self::Error>;
}
