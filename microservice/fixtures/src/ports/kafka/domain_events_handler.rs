use axum::async_trait;
use chrono::{DateTime, Utc};
use log::info;
use microservices_shared::{
    domain_events::DomainEventCallbacks,
    domain_ids::{FixtureId, RefereeId, TeamId, VenueId},
};

pub struct DomainEventCallbacksImpl {}

impl DomainEventCallbacksImpl {
    pub fn new() -> Self {
        Self {}
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
}
