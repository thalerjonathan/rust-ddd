use microservices_shared::domain_ids::VenueId;
use sqlx::{FromRow, Postgres, Transaction};
use uuid::Uuid;

use crate::domain::{aggregates::venue::Venue, repositories::venue_repo::VenueRepository};

#[derive(FromRow)]
struct VenueDb {
    id: Uuid,
    name: String,
    street: String,
    zip: String,
    city: String,
    telephone: Option<String>,
    email: Option<String>,
}

pub struct VenueRepositoryPg {}

impl VenueRepositoryPg {
    pub fn new() -> Self {
        Self {}
    }
}

impl From<VenueDb> for Venue {
    fn from(venue: VenueDb) -> Self {
        Venue::from_id(
            VenueId::from(venue.id),
            venue.name,
            venue.street,
            venue.zip,
            venue.city,
            venue.telephone,
            venue.email,
        )
    }
}

impl VenueRepository for VenueRepositoryPg {
    type Error = String;
    type TxCtx = Transaction<'static, Postgres>;

    async fn find_by_id(
        &self,
        venue_id: VenueId,
        tx_ctx: &mut Self::TxCtx,
    ) -> Result<Option<Venue>, Self::Error> {
        let venue: Option<VenueDb> = sqlx::query_as(
            "SELECT venue_id as id, name, street, zip, city, telephone, email
            FROM rustddd.venues 
            WHERE venue_id = $1"
        )
        .bind(venue_id.0)
        .fetch_optional(&mut **tx_ctx)
        .await
        .map_err(|e| e.to_string())?;

        Ok(venue.map(|v| v.into()))
    }

    async fn get_all(&self, tx_ctx: &mut Self::TxCtx) -> Result<Vec<Venue>, Self::Error> {
        let venues: Vec<VenueDb> = sqlx::query_as(
            "SELECT venue_id as id, name, street, zip, city, telephone, email 
            FROM rustddd.venues
            ORDER BY name ASC"
        )
        .fetch_all(&mut **tx_ctx)
        .await
        .map_err(|e| e.to_string())?;

        Ok(venues.into_iter().map(|v| v.into()).collect())
    }

    async fn save(&self, venue: &Venue, tx_ctx: &mut Self::TxCtx) -> Result<(), Self::Error> {
        // NOTE: no upsert, because Venue is not allowed to change after creation
        let _result = sqlx::query(
            "INSERT INTO rustddd.venues (venue_id, name, street, zip, city, telephone, email) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *"
        )
        .bind(venue.id().0)
        .bind(venue.name())
        .bind(venue.street())
        .bind(venue.zip())
        .bind(venue.city())
        .bind(venue.telephone())
        .bind(venue.email())
        .fetch_one(&mut **tx_ctx)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }
}
