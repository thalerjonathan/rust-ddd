use crate::domain::aggregates::venue::{Venue, VenueId};

pub trait VenueRepository<Tx> {
    type Error;

    async fn find_by_id(&self, id: &VenueId, tx: &mut Tx) -> Result<Option<Venue>, Self::Error>;
    async fn get_all(&self, tx: &mut Tx) -> Result<Vec<Venue>, Self::Error>;

    async fn save(&self, venue: &Venue, tx: &mut Tx) -> Result<(), Self::Error>;
}
