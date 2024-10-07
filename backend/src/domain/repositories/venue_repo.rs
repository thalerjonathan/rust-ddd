use crate::domain::aggregates::venue::{Venue, VenueId};

pub trait VenueRepository {
    type Error;

    async fn find_by_id(&self, id: &VenueId) -> Result<Option<Venue>, Self::Error>;
    async fn get_all(&self) -> Result<Vec<Venue>, Self::Error>;

    async fn save(&self, venue: &Venue) -> Result<(), Self::Error>;
}
