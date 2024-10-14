use log::debug;

use crate::domain::{aggregates::venue::Venue, repositories::venue_repo::VenueRepository};

pub async fn create_venue<DbTx>(
    name: &str,
    street: &str,
    zip: &str,
    city: &str,
    telephone: Option<String>,
    email: Option<String>,
    repo: &mut impl VenueRepository<Tx = DbTx, Error = String>,
    tx: &mut DbTx,
) -> Result<Venue, String> {
    let venue = Venue::new(&name, &street, &zip, &city, telephone, email);

    repo.save(&venue, tx).await.map_err(|e| e.to_string())?;

    debug!("Venue created: {:?}", venue);
    Ok(venue)
}
