use crate::domain::aggregates::team::{Team, TeamId};

pub trait TeamRepository {
    type Error;

    async fn find_by_id(&self, id: &TeamId) -> Result<Option<Team>, Self::Error>;
    async fn get_all(&self) -> Result<Vec<Team>, Self::Error>;

    async fn save(&self, team: &Team) -> Result<(), Self::Error>;
}
