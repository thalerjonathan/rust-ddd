use std::{future::Future, sync::Arc};

use log::debug;
use redis::Commands;
use restinterface::{
    fetch_referee, fetch_team, fetch_venue, RefereeDTO, RefereeIdDTO, TeamDTO, TeamIdDTO, VenueDTO,
    VenueIdDTO,
};
use serde::{de::DeserializeOwned, Serialize};
use tokio::sync::Mutex;

use crate::domain_ids::{RefereeId, TeamId, VenueId};

use super::traits::{RefereeResolver, TeamResolver, VenueResolver};

// NOTE: unfortunately we need to use a tokio Mutex with an Arc due to async code in REST handlers - otherwise axum would complain about "trait bound no longer satisfied"
// see https://stackoverflow.com/questions/76307624/unexplained-trait-bound-no-longer-satisfied-when-modifying-axum-handler-body

pub struct VenueResolverImpl {
    redis_conn: Arc<Mutex<redis::Connection>>,
}
pub struct TeamResolverImpl {
    redis_conn: Arc<Mutex<redis::Connection>>,
}
pub struct RefereeResolverImpl {
    redis_conn: Arc<Mutex<redis::Connection>>,
}

impl VenueResolverImpl {
    pub fn new(redis_conn: Arc<Mutex<redis::Connection>>) -> Self {
        Self { redis_conn }
    }
}

impl VenueResolver for VenueResolverImpl {
    type Error = String;

    async fn resolve(&self, venue_id: &VenueId) -> Result<VenueDTO, Self::Error> {
        let mut redis_conn_mut = self.redis_conn.lock().await;

        let key = format!("venue_{}", venue_id.0.to_string());
        run_cached(&key, &mut redis_conn_mut, || {
            fetch_venue(VenueIdDTO::from(*venue_id))
        })
        .await
    }
}

impl TeamResolverImpl {
    pub fn new(redis_conn: Arc<Mutex<redis::Connection>>) -> Self {
        Self { redis_conn }
    }
}

impl TeamResolver for TeamResolverImpl {
    type Error = String;

    async fn resolve(&self, team_id: &TeamId) -> Result<TeamDTO, Self::Error> {
        let mut redis_conn_mut = self.redis_conn.lock().await;

        let key = format!("team_{}", team_id.0.to_string());
        run_cached(&key, &mut redis_conn_mut, || {
            fetch_team(TeamIdDTO::from(*team_id))
        })
        .await
    }
}

impl RefereeResolverImpl {
    pub fn new(redis_conn: Arc<Mutex<redis::Connection>>) -> Self {
        Self { redis_conn }
    }
}

impl RefereeResolver for RefereeResolverImpl {
    type Error = String;

    async fn resolve(&self, referee_id: &RefereeId) -> Result<RefereeDTO, Self::Error> {
        let mut redis_conn_mut = self.redis_conn.lock().await;

        let key = format!("referee_{}", referee_id.0.to_string());
        run_cached(&key, &mut redis_conn_mut, || {
            fetch_referee(RefereeIdDTO::from(*referee_id))
        })
        .await
    }
}

async fn run_cached<Dto, F, Fut>(
    key: &str,
    redis_conn: &mut redis::Connection,
    fetch_fn: F,
) -> Result<Dto, String>
where
    Dto: Serialize + DeserializeOwned,
    F: Fn() -> Fut,
    // NOTE: we need to return reqwest::Error as we have no other option because to transform the error
    // we would need to await the future, which would break the async block
    Fut: Future<Output = Result<Dto, reqwest::Error>>,
{
    let redis_result: Option<String> = redis_conn.get(key).map_err(|e| e.to_string())?;
    if let Some(json) = redis_result {
        debug!("Found {} in Redis: {}", key, json);

        // TODO: if deserialisation fails, we should try from REST endpoint
        let dto = serde_json::from_str(&json).map_err(|e| e.to_string())?;
        return Ok(dto);
    }

    debug!("Not found in Redis: {} - fetching from REST", key);

    let dto = fetch_fn().await.map_err(|e| e.to_string())?;
    let serialised_dto = serde_json::to_string(&dto).map_err(|e| e.to_string())?;
    // NOTE: need to handle this result this way due to "in edition 2024, the requirement `!: FromRedisValue` will fail"
    // see https://users.rust-lang.org/t/this-function-depends-on-never-type-fallback-being/120158
    let result: Result<(), redis::RedisError> = redis_conn.set(key, serialised_dto);
    result.map_err(|e| e.to_string())?;

    Ok(dto)
}
