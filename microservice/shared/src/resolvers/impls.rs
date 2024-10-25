use std::{cell::RefCell, rc::Rc};

use redis::Commands;
use restinterface::{
    fetch_referee, fetch_team, fetch_venue, RefereeDTO, RefereeIdDTO, TeamDTO, TeamIdDTO, VenueDTO,
    VenueIdDTO,
};

use crate::domain_ids::{RefereeId, TeamId, VenueId};

use super::traits::{RefereeResolver, TeamResolver, VenueResolver};

pub struct VenueResolverImpl {
    redis_conn: Rc<RefCell<redis::Connection>>,
}
pub struct TeamResolverImpl {
    redis_conn: Rc<RefCell<redis::Connection>>,
}
pub struct RefereeResolverImpl {
    redis_conn: Rc<RefCell<redis::Connection>>,
}

impl VenueResolverImpl {
    pub fn new(redis_conn: Rc<RefCell<redis::Connection>>) -> Self {
        Self { redis_conn }
    }
}

impl VenueResolver for VenueResolverImpl {
    type Error = String;

    async fn resolve(&self, venue_id: &VenueId) -> Result<VenueDTO, Self::Error> {
        let mut redis_conn_mut = self.redis_conn.borrow_mut();

        let key = format!("venue_{}", venue_id.0.to_string());
        let redis_result: Option<String> =
            redis_conn_mut.get(key.clone()).map_err(|e| e.to_string())?;
        if let Some(venue_json) = redis_result {
            // TODO: if deserialisation fails, we should try from REST endpoint
            let venue: VenueDTO = serde_json::from_str(&venue_json).map_err(|e| e.to_string())?;
            return Ok(venue);
        }

        let venue = fetch_venue(VenueIdDTO::from(*venue_id))
            .await
            .map_err(|e| e.to_string())?;
        let venue_serialised = serde_json::to_string(&venue).map_err(|e| e.to_string())?;

        // NOTE: need to handle this result this way due to "in edition 2024, the requirement `!: FromRedisValue` will fail"
        // see https://users.rust-lang.org/t/this-function-depends-on-never-type-fallback-being/120158
        let result: Result<(), redis::RedisError> =
            redis_conn_mut.set(key.clone(), venue_serialised);
        result.map_err(|e| e.to_string())?;
        Ok(venue)
    }
}

impl TeamResolverImpl {
    pub fn new(redis_conn: Rc<RefCell<redis::Connection>>) -> Self {
        Self { redis_conn }
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
    pub fn new(redis_conn: Rc<RefCell<redis::Connection>>) -> Self {
        Self { redis_conn }
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
