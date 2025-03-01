use microservices_shared::{
    domain_event_repo::DomainEventOutboxRepository, domain_events::DomainEvent,
};

use crate::domain::{aggregates::venue::Venue, repositories::venue_repo::VenueRepository};

pub async fn create_venue<TxCtx>(
    name: &str,
    street: &str,
    zip: &str,
    city: &str,
    telephone: Option<String>,
    email: Option<String>,
    repo: &impl VenueRepository<TxCtx = TxCtx, Error = String>,
    domain_event_repo: &impl DomainEventOutboxRepository<TxCtx = TxCtx, Error = String>,
    tx_ctx: &mut TxCtx,
) -> Result<Venue, String> {
    let venue = Venue::new(&name, &street, &zip, &city, telephone, email);

    repo.save(&venue, tx_ctx).await.map_err(|e| e.to_string())?;

    domain_event_repo
        .store(
            DomainEvent::VenueCreated {
                venue_id: venue.id().clone(),
            },
            tx_ctx,
        )
        .await
        .map_err(|e| e.to_string())?;

    Ok(venue)
}
