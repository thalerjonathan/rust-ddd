use mockall::automock;
use restinterface::{RefereeDTO, TeamDTO, VenueDTO};

use crate::domain_ids::{RefereeId, TeamId, VenueId};

#[allow(async_fn_in_trait)]
#[automock(type Error = String; type TxCtx = ();)]
pub trait VenueResolver {
    type Error;
    async fn resolve(&self, venue_id: &VenueId) -> Result<VenueDTO, Self::Error>;
}

#[allow(async_fn_in_trait)]
#[automock(type Error = String; type TxCtx = ();)]
pub trait TeamResolver {
    type Error;
    async fn resolve(&self, team_id: &TeamId) -> Result<TeamDTO, Self::Error>;
}

#[allow(async_fn_in_trait)]
#[automock(type Error = String; type TxCtx = ();)]
pub trait RefereeResolver {
    type Error;
    async fn resolve(&self, referee_id: &RefereeId) -> Result<RefereeDTO, Self::Error>;
}
