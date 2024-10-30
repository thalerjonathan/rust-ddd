use axum::async_trait;
use log::info;
use microservices_shared::{
    domain_events::DomainEventCallbacks,
    domain_ids::{FixtureId, RefereeId, TeamId, VenueId},
};
use redis::Commands;
use sqlx::types::chrono::{DateTime, Utc};

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

    async fn on_venue_created(&mut self, venue_id: VenueId) {
        info!("Received Domain Event: Venue created: {:?}", venue_id);
    }

    async fn on_fixture_created(&mut self, fixture_id: FixtureId) {
        info!("Received Domain Event: Fixture created: {:?}", fixture_id);
    }

    async fn on_fixture_date_changed(&mut self, fixture_id: FixtureId, date: DateTime<Utc>) {
        info!(
            "Received Domain Event: Fixture date changed: {:?} -> {}",
            fixture_id, date
        );
    }

    async fn on_fixture_venue_changed(&mut self, fixture_id: FixtureId, venue_id: VenueId) {
        info!(
            "Received Domain Event: Fixture venue changed: {:?} -> {:?}",
            fixture_id, venue_id
        );
    }

    async fn on_fixture_cancelled(&mut self, fixture_id: FixtureId) {
        info!("Received Domain Event: Fixture cancelled: {:?}", fixture_id);
    }

    async fn on_availability_declared(&mut self, fixture_id: FixtureId, referee_id: RefereeId) {
        info!(
            "Received Domain Event: Availability declared: {:?} -> {:?}",
            fixture_id, referee_id
        );
    }

    async fn on_availability_withdrawn(&mut self, fixture_id: FixtureId, referee_id: RefereeId) {
        info!(
            "Received Domain Event: Availability withdrawn: {:?} -> {:?}",
            fixture_id, referee_id
        );
    }
}
