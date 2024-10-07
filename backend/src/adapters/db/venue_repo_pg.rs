use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::domain::{
    aggregates::venue::{Venue, VenueId},
    repositories::venue_repo::VenueRepository,
};

pub struct VenueRepositoryPg<'a> {
    pool: &'a Pool<Postgres>,
}

pub struct VenueDb {
    id: Uuid,
    name: String,
    street: String,
    zip: String,
    city: String,
    telephone: Option<String>,
    email: Option<String>,
}

impl<'a> VenueRepositoryPg<'a> {
    pub fn new(pool: &'a Pool<Postgres>) -> Self {
        Self { pool }
    }
}

impl<'a> VenueRepository for VenueRepositoryPg<'a> {
    type Error = String;

    async fn find_by_id(&self, id: &VenueId) -> Result<Option<Venue>, Self::Error> {
        let venue: Option<VenueDb> = sqlx::query_as!(
            VenueDb,
            "SELECT venue_id as id, name, street, zip, city, telephone, email FROM rustddd.venues WHERE venue_id = $1",
            id.0
        )
        .fetch_optional(self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(venue.map(|v| v.into()))
    }

    async fn get_all(&self) -> Result<Vec<Venue>, Self::Error> {
        let venues: Vec<VenueDb> = sqlx::query_as!(
            VenueDb,
            "SELECT venue_id as id, name, street, zip, city, telephone, email FROM rustddd.venues"
        )
        .fetch_all(self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(venues.into_iter().map(|v| v.into()).collect())
    }

    async fn save(&self, venue: &Venue) -> Result<(), Self::Error> {
        // NOTE: no upsert, because Venue is not allowed to change after creation
        let _result = sqlx::query!(
            "INSERT INTO rustddd.venues (venue_id, name, street, zip, city, telephone, email) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *",
            venue.id().0,
            venue.name(),
            venue.street(),
            venue.zip(),
            venue.city(),
            venue.telephone(),
            venue.email()
        )
        .fetch_one(self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }
}

impl From<VenueDb> for Venue {
    fn from(venue: VenueDb) -> Self {
        Venue::from_id(
            venue.id,
            venue.name,
            venue.street,
            venue.zip,
            venue.city,
            venue.telephone,
            venue.email,
        )
    }
}
