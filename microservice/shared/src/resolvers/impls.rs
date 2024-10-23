use restinterface::{
    fetch_referee, fetch_team, fetch_venue, RefereeDTO, RefereeIdDTO, TeamDTO, TeamIdDTO, VenueDTO,
    VenueIdDTO,
};

use crate::domain_ids::{RefereeId, TeamId, VenueId};

use super::traits::{RefereeResolver, TeamResolver, VenueResolver};

pub struct VenueResolverImpl;
pub struct TeamResolverImpl;
pub struct RefereeResolverImpl;

impl VenueResolverImpl {
    pub fn new() -> Self {
        Self
    }
}

impl VenueResolver for VenueResolverImpl {
    type Error = String;

    async fn resolve(&self, venue_id: &VenueId) -> Result<VenueDTO, Self::Error> {
        // TODO: add caching via Redis
        fetch_venue(VenueIdDTO::from(*venue_id))
            .await
            .map_err(|e| e.to_string())
    }
}

impl TeamResolverImpl {
    pub fn new() -> Self {
        Self
    }
}

impl TeamResolver for TeamResolverImpl {
    type Error = String;

    async fn resolve(&self, team_id: &TeamId) -> Result<TeamDTO, Self::Error> {
        // TODO: add caching via Redis
        fetch_team(TeamIdDTO::from(*team_id))
            .await
            .map_err(|e| e.to_string())
    }
}

impl RefereeResolverImpl {
    pub fn new() -> Self {
        Self
    }
}

impl RefereeResolver for RefereeResolverImpl {
    type Error = String;

    async fn resolve(&self, referee_id: &RefereeId) -> Result<RefereeDTO, Self::Error> {
        // TODO: add caching via Redis
        fetch_referee(RefereeIdDTO::from(*referee_id))
            .await
            .map_err(|e| e.to_string())
    }
}
