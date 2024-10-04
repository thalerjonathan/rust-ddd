use crate::domain::aggregates::referee::{Referee, RefereeId};

pub trait RefereeRepository {
    type Error;

    async fn find_by_id(&self, id: &RefereeId) -> Result<Option<Referee>, Self::Error>;
    async fn get_all(&self) -> Result<Vec<Referee>, Self::Error>;

    async fn save(&self, referee: &Referee) -> Result<(), Self::Error>;
}
