use crate::domain::aggregates::fixture::{Fixture, FixtureId};

pub trait FixtureRepository {
    type Error;

    async fn find_by_id(&self, id: &FixtureId) -> Result<Option<Fixture>, Self::Error>;
    async fn get_all(&self) -> Result<Vec<Fixture>, Self::Error>;

    async fn save(&self, fixture: &Fixture) -> Result<(), Self::Error>;
}
