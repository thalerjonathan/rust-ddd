#[cfg(test)]
mod venues_tests {
    use shared::{create_venue, fetch_venue, fetch_venues, VenueCreationDTO};
    use sqlx::PgPool;
    #[tokio::test]
    async fn given_empty_db_when_fetching_venues_then_empty_list_is_returned() {
        clear_venue_table().await;

        let venues = fetch_venues().await;
        assert!(venues.is_empty(), "Venues should be empty");
    }

    #[tokio::test]
    async fn given_empty_db_when_creating_venue_then_venue_is_returned() {
        clear_venue_table().await;

        let venue_creation = VenueCreationDTO {
            name: "Venue A".to_string(),
            street: "Street A".to_string(),
            zip: "12345".to_string(),
            city: "City A".to_string(),
            telephone: Some("1234567890".to_string()),
            email: Some("email@example.com".to_string()),
        };

        let venue = create_venue(venue_creation).await;
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

        let fetched_venue = fetch_venue(&venue.id.to_string()).await;
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

    async fn clear_venue_table() {
        let db_url = std::env::var("DB_URL").expect("DB_URL not set");
        let connection_pool = PgPool::connect(&db_url).await.unwrap();

        sqlx::query("DELETE FROM rustddd.venues")
            .execute(&connection_pool)
            .await
            .unwrap();
    }
}
