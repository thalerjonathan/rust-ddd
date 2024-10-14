use std::marker::PhantomData;

use sqlx::{Postgres, Transaction};
use uuid::Uuid;

use crate::domain::{
    aggregates::venue::{Venue, VenueId},
    repositories::venue_repo::VenueRepository,
};

struct VenueDb {
    id: Uuid,
    name: String,
    street: String,
    zip: String,
    city: String,
    telephone: Option<String>,
    email: Option<String>,
}

pub struct VenueRepositoryPg {
    _lifetime: PhantomData<&'static Transaction<'static, Postgres>>,
}

impl VenueRepositoryPg {
    pub fn new() -> Self {
        Self {
            _lifetime: PhantomData,
        }
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

impl VenueRepository for VenueRepositoryPg {
    type Error = String;
    type Tx = Transaction<'static, Postgres>;

    async fn find_by_id(
        &self,
        id: &VenueId,
        tx: &mut Self::Tx,
    ) -> Result<Option<Venue>, Self::Error> {
        let venue: Option<VenueDb> = sqlx::query_as!(
                VenueDb,
            "SELECT venue_id as id, name, street, zip, city, telephone, email FROM rustddd.venues WHERE venue_id = $1",
            id.0
        )
        .fetch_optional(&mut **tx)
        .await
        .map_err(|e| e.to_string())?;

        Ok(venue.map(|v| v.into()))
    }

    async fn get_all(&self, tx: &mut Self::Tx) -> Result<Vec<Venue>, Self::Error> {
        let venues: Vec<VenueDb> = sqlx::query_as!(
            VenueDb,
            "SELECT venue_id as id, name, street, zip, city, telephone, email FROM rustddd.venues"
        )
        .fetch_all(&mut **tx)
        .await
        .map_err(|e| e.to_string())?;

        Ok(venues.into_iter().map(|v| v.into()).collect())
    }

    async fn save(&self, venue: &Venue, tx: &mut Self::Tx) -> Result<(), Self::Error> {
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
        .fetch_one(&mut **tx)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }
}
