use microservices_shared::{
    domain_event_repo::DomainEventOutboxRepository,
    domain_events::DomainEvent,
    domain_ids::{FixtureId, RefereeId},
    resolvers::traits::{FixtureResolver, RefereeResolver},
};

use crate::domain::repositories::availability_repo::AvailabilityRepository;

pub async fn declare_availability<TxCtx>(
    fixture_id: FixtureId,
    referee_id: RefereeId,
    fixture_resolver: &impl FixtureResolver<Error = String>,
    referee_resolver: &impl RefereeResolver<Error = String>,
    availability_repo: &impl AvailabilityRepository<TxCtx = TxCtx, Error = String>,
    domain_event_repo: &impl DomainEventOutboxRepository<TxCtx = TxCtx, Error = String>,
    tx_ctx: &mut TxCtx,
) -> Result<(), String> {
    let _fixture = fixture_resolver
        .resolve(&fixture_id)
        .await
        .expect(&format!("Fixture {:?} not resolved", fixture_id));
    let _referee = referee_resolver
        .resolve(&referee_id)
        .await
        .expect(&format!("Referee {:?} not resolved", referee_id));

    if availability_repo
        .is_available(&fixture_id, &referee_id, tx_ctx)
        .await?
    {
        return Err(
            "Referee is already available for this fixture - cannot declare availability"
                .to_string(),
        );
    }

    availability_repo
        .declare_availability(&fixture_id, &referee_id, tx_ctx)
        .await?;

    domain_event_repo
        .store(
            DomainEvent::AvailabilityDeclared {
                fixture_id,
                referee_id,
            },
            tx_ctx,
        )
        .await?;

    Ok(())
}

pub async fn withdraw_availability<TxCtx>(
    fixture_id: FixtureId,
    referee_id: RefereeId,
    fixture_resolver: &impl FixtureResolver<Error = String>,
    referee_resolver: &impl RefereeResolver<Error = String>,
    availability_repo: &impl AvailabilityRepository<TxCtx = TxCtx, Error = String>,
    domain_event_repo: &impl DomainEventOutboxRepository<TxCtx = TxCtx, Error = String>,
    tx_ctx: &mut TxCtx,
) -> Result<(), String> {
    let _fixture = fixture_resolver
        .resolve(&fixture_id)
        .await
        .expect(&format!("Fixture {:?} not resolved", fixture_id));
    let _referee = referee_resolver
        .resolve(&referee_id)
        .await
        .expect(&format!("Referee {:?} not resolved", referee_id));

    if !availability_repo
        .is_available(&fixture_id, &referee_id, tx_ctx)
        .await?
    {
        return Err(
            "Referee is not available for this fixture - cannot withdraw availability".to_string(),
        );
    }

    availability_repo
        .withdraw_availability(&fixture_id, &referee_id, tx_ctx)
        .await?;

    domain_event_repo
        .store(
            DomainEvent::AvailabilityWithdrawn {
                fixture_id,
                referee_id,
            },
            tx_ctx,
        )
        .await?;

    Ok(())
}

pub async fn get_availabilities_for_referee<TxCtx>(
    referee_id: RefereeId,
    referee_resolver: &impl RefereeResolver<Error = String>,
    availability_repo: &impl AvailabilityRepository<TxCtx = TxCtx, Error = String>,
    tx_ctx: &mut TxCtx,
) -> Result<Vec<FixtureId>, String> {
    let _referee = referee_resolver
        .resolve(&referee_id)
        .await
        .expect(&format!("Referee {:?} not resolved", referee_id));

    let availabilities = availability_repo
        .get_availabilities_for_referee(&referee_id, tx_ctx)
        .await?;

    Ok(availabilities)
}
