use axum::async_trait;
use log::info;
use microservices_shared::{
    domain_events::DomainEventCallbacks,
    domain_ids::{RefereeId, TeamId},
};
use redis::Commands;

pub struct DomainEventCallbacksImpl {
    redis_conn: redis::Connection,
}

impl DomainEventCallbacksImpl {
    pub fn new(redis_conn: redis::Connection) -> Self {
        Self { redis_conn }
    }
}

#[async_trait]
impl DomainEventCallbacks for DomainEventCallbacksImpl {
    async fn on_referee_created(&mut self, referee_id: RefereeId) {
        info!("Received Domain Event: Referee created: {:?}", referee_id);
    }

    async fn on_referee_club_changed(&mut self, referee_id: RefereeId, club_name: String) {
        info!(
            "Received Domain Event: Referee club changed: {:?} -> {}",
            referee_id, club_name
        );

        info!("Invalidating cache entry for referee: {:?}", referee_id);

        // NOTE: invalidate the cache entry for the referee
        let key = format!("referee_{}", referee_id.0.to_string());
        let _result: Result<(), redis::RedisError> = self.redis_conn.del(key);
    }

    async fn on_team_created(&mut self, team_id: TeamId) {
        info!("Received Domain Event: Team created: {:?}", team_id);
    }
}
