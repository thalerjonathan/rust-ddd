use axum::async_trait;
use log::info;
use microservices_shared::{
    domain_events::DomainEventCallbacks,
    domain_ids::{RefereeId, TeamId},
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
}
