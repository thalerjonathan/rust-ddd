use log::debug;
use microservices_shared::domain_events::{DomainEvent, DomainEventPublisher};

use crate::domain::{aggregates::venue::Venue, repositories::venue_repo::VenueRepository};

pub async fn create_venue<TxCtx>(
    name: &str,
    street: &str,
    zip: &str,
    city: &str,
    telephone: Option<String>,
    email: Option<String>,
    repo: &impl VenueRepository<TxCtx = TxCtx, Error = String>,
    domain_event_publisher: &Box<dyn DomainEventPublisher + Send + Sync>,
    tx_ctx: &mut TxCtx,
) -> Result<Venue, String> {
    let venue = Venue::new(&name, &street, &zip, &city, telephone, email);

    repo.save(&venue, tx_ctx).await.map_err(|e| e.to_string())?;

    domain_event_publisher
        .publish_domain_event(DomainEvent::VenueCreated {
            venue_id: venue.id().clone(),
        })
        .await
        .map_err(|e| e.to_string())?;

    debug!("Venue created: {:?}", venue);
    Ok(venue)
}
