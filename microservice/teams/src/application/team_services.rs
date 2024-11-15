use microservices_shared::{
    domain_event_repo::DomainEventOutboxRepository, domain_events::DomainEvent,
};

use crate::domain::{aggregates::team::Team, repositories::team_repo::TeamRepository};

pub async fn create_team<TxCtx>(
    name: &str,
    club: &str,
    repo: &impl TeamRepository<TxCtx = TxCtx, Error = String>,
    domain_event_repo: &impl DomainEventOutboxRepository<TxCtx = TxCtx, Error = String>,
    tx_ctx: &mut TxCtx,
) -> Result<Team, String> {
    let team = Team::new(name, club);

    repo.save(&team, tx_ctx).await.map_err(|e| e.to_string())?;

    domain_event_repo
        .store(
            DomainEvent::TeamCreated {
                team_id: team.id().clone(),
            },
            tx_ctx,
        )
        .await
        .map_err(|e| e.to_string())?;

    Ok(team)
}
