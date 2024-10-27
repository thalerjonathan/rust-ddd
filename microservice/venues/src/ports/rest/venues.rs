use std::sync::Arc;

use axum::{
    extract::{Path, State},
    Json,
};
use log::debug;
use restinterface::{app_error::AppError, VenueCreationDTO, VenueDTO, VenueIdDTO};

use crate::{
    adapters::db::venue_repo_pg::VenueRepositoryPg,
    application,
    domain::{aggregates::venue::Venue, repositories::venue_repo::VenueRepository},
    AppState,
};

impl From<Venue> for VenueDTO {
    fn from(venue: Venue) -> Self {
        VenueDTO {
            id: venue.id().into(),
            name: venue.name().to_string(),
            street: venue.street().to_string(),
            zip: venue.zip().to_string(),
            city: venue.city().to_string(),
            telephone: venue.telephone(),
            email: venue.email(),
        }
    }
}

pub async fn create_venue_handler(
    State(state): State<Arc<AppState>>,
    Json(venue_creation): Json<VenueCreationDTO>,
) -> Result<Json<VenueDTO>, AppError> {
    debug!("Creating venue: {:?}", venue_creation);

    let mut tx: sqlx::Transaction<'static, sqlx::Postgres> =
        state.connection_pool.begin().await.unwrap();

    let mut repo: VenueRepositoryPg = VenueRepositoryPg::new();
    let venue = application::venue_services::create_venue(
        &venue_creation.name,
        &venue_creation.street,
        &venue_creation.zip,
        &venue_creation.city,
        venue_creation.telephone,
        venue_creation.email,
        &mut repo,
        &state.domain_event_publisher,
        &mut tx,
    )
    .await
    .map_err(|e| AppError::from_error(&e.to_string()))?;

    tx.commit()
        .await
        .map_err(|e| AppError::from_error(&e.to_string()))?;

    let venue = VenueDTO::from(venue);

    debug!("Venue created: {:?}", venue);

    Ok(Json::from(venue))
}

pub async fn get_venue_by_id_handler(
    State(state): State<Arc<AppState>>,
    Path(venue_id): Path<VenueIdDTO>,
) -> Result<Json<Option<VenueDTO>>, AppError> {
    debug!("Getting venue by id: {}", venue_id.0);

    let mut tx = state.connection_pool.begin().await.unwrap();

    let repo = VenueRepositoryPg::new();
    // NOTE: we are not using an application service here, because the logic is so simple
    let venue = repo
        .find_by_id(venue_id.into(), &mut tx)
        .await
        .map_err(|e| AppError::from_error(&e.to_string()))?;

    tx.commit()
        .await
        .map_err(|e| AppError::from_error(&e.to_string()))?;

    debug!("Venue found: {:?}", venue);

    Ok(Json::from(venue.map(|v| v.into())))
}

pub async fn get_all_venues_handler(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<VenueDTO>>, AppError> {
    debug!("Getting all venues");

    let mut tx = state.connection_pool.begin().await.unwrap();

    let repo = VenueRepositoryPg::new();

    // NOTE: we are not using an application service here, because the logic is so simple
    let venues = repo
        .get_all(&mut tx)
        .await
        .map_err(|e| AppError::from_error(&e.to_string()))?;

    tx.commit()
        .await
        .map_err(|e| AppError::from_error(&e.to_string()))?;

    Ok(Json::from(
        venues
            .into_iter()
            .map(|v| v.into())
            .collect::<Vec<VenueDTO>>(),
    ))
}

#[cfg(test)]
mod venues_tests {
    use restinterface::{create_venue, fetch_venue, fetch_venues, VenueCreationDTO};
    use sqlx::PgPool;

    #[tokio::test]
    async fn given_empty_db_when_fetching_venues_then_empty_list_is_returned() {
        clear_tables().await;

        let venues = fetch_venues().await;
        assert!(venues.is_empty(), "Venues should be empty");
    }

    #[tokio::test]
    async fn given_empty_db_when_creating_venue_then_venue_is_returned() {
        clear_tables().await;

        let venue_creation = VenueCreationDTO {
            name: "Venue A".to_string(),
            street: "Street A".to_string(),
            zip: "12345".to_string(),
            city: "City A".to_string(),
            telephone: Some("1234567890".to_string()),
            email: Some("email@example.com".to_string()),
        };

        let venue = create_venue(&venue_creation).await;
        assert!(venue.is_ok(), "Venue should be created");

        let venue = venue.unwrap();
        assert_eq!(venue.name, "Venue A", "Venue name should be 'Venue A'");
        assert_eq!(
            venue.street, "Street A",
            "Venue street should be 'Street A'"
        );
        assert_eq!(venue.zip, "12345", "Venue zip should be '12345'");
        assert_eq!(venue.city, "City A", "Venue city should be 'City A'");
        assert_eq!(
            venue.telephone,
            Some("1234567890".to_string()),
            "Venue telephone should be '1234567890'"
        );
        assert_eq!(
            venue.email,
            Some("email@example.com".to_string()),
            "Venue email should be 'email@example.com'"
        );

        let fetched_venue = fetch_venue(venue.id.into()).await;
        assert!(fetched_venue.is_ok(), "Venue should be fetched");

        let fetched_venue = fetched_venue.unwrap();
        assert_eq!(
            fetched_venue.name, "Venue A",
            "Venue name should be 'Venue A'"
        );
        assert_eq!(
            fetched_venue.street, "Street A",
            "Venue street should be 'Street A'"
        );
        assert_eq!(fetched_venue.zip, "12345", "Venue zip should be '12345'");
        assert_eq!(
            fetched_venue.city, "City A",
            "Venue city should be 'City A'"
        );
        assert_eq!(
            fetched_venue.telephone,
            Some("1234567890".to_string()),
            "Venue telephone should be '1234567890'"
        );
        assert_eq!(
            fetched_venue.email,
            Some("email@example.com".to_string()),
            "Venue email should be 'email@example.com'"
        );

        let all_venues = fetch_venues().await;
        assert_eq!(all_venues.len(), 1, "There should be 1 venue");
        assert_eq!(
            all_venues[0].name, "Venue A",
            "Venue name should be 'Venue A'"
        );
        assert_eq!(
            all_venues[0].street, "Street A",
            "Venue street should be 'Street A'"
        );
        assert_eq!(all_venues[0].zip, "12345", "Venue zip should be '12345'");
        assert_eq!(
            all_venues[0].city, "City A",
            "Venue city should be 'City A'"
        );
    }

    async fn clear_tables() {
        let db_url = "postgres://postgres:postgres@localhost:5434/venues?application_name=rustddd&options=-c search_path%3Drustddd";
        let pool = PgPool::connect(&db_url).await.unwrap();
        sqlx::query!("DELETE FROM rustddd.venues")
            .execute(&pool)
            .await
            .unwrap();
    }
}
