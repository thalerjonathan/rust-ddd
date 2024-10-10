use chrono::{DateTime, Utc};

use crate::domain::aggregates::{
    fixture::{Fixture, FixtureId},
    team::TeamId,
    venue::VenueId,
};

pub trait FixtureRepository {
    type Error;

    async fn find_by_id(&self, id: &FixtureId) -> Result<Option<Fixture>, Self::Error>;
    async fn get_all(&self) -> Result<Vec<Fixture>, Self::Error>;
    async fn find_by_day_and_venue(
        &self,
        date: &DateTime<Utc>,
        venue_id: &VenueId,
    ) -> Result<Vec<Fixture>, Self::Error>;
    async fn find_by_day_and_team(
        &self,
        date: &DateTime<Utc>,
        team_id: &TeamId,
    ) -> Result<Vec<Fixture>, Self::Error>;
    async fn save(&self, fixture: &Fixture) -> Result<(), Self::Error>;
}
