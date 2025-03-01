use mockall::automock;
use restinterface::{FixtureDTO, RefereeDTO, TeamDTO, VenueDTO};

use crate::domain_ids::{FixtureId, RefereeId, TeamId, VenueId};

#[allow(async_fn_in_trait)]
#[automock(type Error = String;)]
pub trait VenueResolver {
    type Error;
    async fn resolve(&self, venue_id: &VenueId) -> Result<VenueDTO, Self::Error>;
}

#[allow(async_fn_in_trait)]
#[automock(type Error = String;)]
pub trait TeamResolver {
    type Error;
    async fn resolve(&self, team_id: &TeamId) -> Result<TeamDTO, Self::Error>;
}

#[allow(async_fn_in_trait)]
#[automock(type Error = String;)]
pub trait RefereeResolver {
    type Error;
    async fn resolve(&self, referee_id: &RefereeId) -> Result<RefereeDTO, Self::Error>;
}

#[allow(async_fn_in_trait)]
#[automock(type Error = String;)]
pub trait FixtureResolver {
    type Error;
    async fn resolve(&self, fixture_id: &FixtureId) -> Result<FixtureDTO, Self::Error>;
}
