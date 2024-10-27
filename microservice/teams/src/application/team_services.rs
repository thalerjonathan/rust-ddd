use log::debug;
use microservices_shared::domain_events::{DomainEvent, DomainEventPublisher};

use crate::domain::{aggregates::team::Team, repositories::team_repo::TeamRepository};

pub async fn create_team<TxCtx>(
    name: &str,
    club: &str,
    repo: &impl TeamRepository<TxCtx = TxCtx, Error = String>,
    domain_event_publisher: &Box<dyn DomainEventPublisher + Send + Sync>,
    tx_ctx: &mut TxCtx,
) -> Result<Team, String> {
    let team = Team::new(name, club);

    repo.save(&team, tx_ctx).await.map_err(|e| e.to_string())?;

    domain_event_publisher
        .publish_domain_event(DomainEvent::TeamCreated {
            team_id: team.id().clone(),
        })
        .await
        .map_err(|e| e.to_string())?;

    debug!("Team created: {:?}", team);

    Ok(team)
}
