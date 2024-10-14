use crate::domain::aggregates::venue::{Venue, VenueId};

pub trait VenueRepository {
    type Error;
    type Tx;

    async fn find_by_id(
        &self,
        id: &VenueId,
        tx: &mut Self::Tx,
    ) -> Result<Option<Venue>, Self::Error>;
    async fn get_all(&self, tx: &mut Self::Tx) -> Result<Vec<Venue>, Self::Error>;

    async fn save(&self, venue: &Venue, tx: &mut Self::Tx) -> Result<(), Self::Error>;
}
