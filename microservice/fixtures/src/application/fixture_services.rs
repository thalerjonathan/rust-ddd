use chrono::{DateTime, Utc};
use microservices_shared::{
    domain_events::{DomainEvent, DomainEventPublisher},
    domain_ids::{FixtureId, TeamId, VenueId},
    resolvers::traits::{TeamResolver, VenueResolver},
};
use restinterface::FixtureDTO;

use crate::domain::{aggregates::fixture::Fixture, repositories::fixture_repo::FixtureRepository};

pub async fn create_fixture<TxCtx>(
    date: DateTime<Utc>,
    venue_id: VenueId,
    team_home_id: TeamId,
    team_away_id: TeamId,
    fixture_repo: &impl FixtureRepository<TxCtx = TxCtx, Error = String>,
    venue_resolver: &impl VenueResolver<Error = String>,
    team_resolver: &impl TeamResolver<Error = String>,
    domain_event_publisher: &Box<dyn DomainEventPublisher + Send + Sync>,
    tx_ctx: &mut TxCtx,
) -> Result<FixtureDTO, String> {
    if team_home_id == team_away_id {
        return Err("Team home and team away cannot be the same".to_string());
    }

    let venue = venue_resolver
        .resolve(&venue_id)
        .await
        .expect(&format!("Venue {:?} not resolved", venue_id));
    let team_home = team_resolver
        .resolve(&team_home_id)
        .await
        .expect(&format!("Team home {:?} not resolved", team_home_id));
    let team_away = team_resolver
        .resolve(&team_away_id)
        .await
        .expect(&format!("Team away {:?} not resolved", team_away_id));

    // we simplify the constraint to no other fixture at the same venue on the same day
    let fixtures = fixture_repo
        .find_by_day_and_venue(&date, venue_id, tx_ctx)
        .await?;
    if !fixtures.is_empty() {
        return Err("There is already a fixture at the same venue on the same day".to_string());
    }

    // we simplify the constraint to no other fixture at the same day for the same team
    let fixtures = fixture_repo
        .find_by_day_and_team(&date, team_home_id, tx_ctx)
        .await?;
    if !fixtures.is_empty() {
        return Err("There is already a fixture at the same day for the home team".to_string());
    }
    let fixtures = fixture_repo
        .find_by_day_and_team(&date, team_away_id, tx_ctx)
        .await?;
    if !fixtures.is_empty() {
        return Err("There is already a fixture at the same day for the away team".to_string());
    }

    let fixture = Fixture::new(date, venue_id, team_home_id, team_away_id, None, None);

    fixture_repo
        .save(&fixture, tx_ctx)
        .await
        .map_err(|e| e.to_string())?;

    domain_event_publisher
        .publish_domain_event(DomainEvent::FixtureCreated {
            fixture_id: fixture.id().clone(),
        })
        .await
        .map_err(|e| e.to_string())?;

    Ok(FixtureDTO {
        id: fixture.id().into(),
        date: fixture.date().clone(),
        status: fixture.status().clone().into(),
        venue: venue.into(),
        team_home: team_home.into(),
        team_away: team_away.into(),
        first_referee: None,
        second_referee: None,
    })
}

pub async fn update_fixture_date<TxCtx>(
    fixture_id: FixtureId,
    date: DateTime<Utc>,
    fixture_repo: &impl FixtureRepository<TxCtx = TxCtx, Error = String>,
    domain_event_publisher: &Box<dyn DomainEventPublisher + Send + Sync>,
    tx_ctx: &mut TxCtx,
) -> Result<(), String> {
    let mut fixture = fixture_repo
        .find_by_id(fixture_id, tx_ctx)
        .await?
        .expect("Fixture not found");

    fixture.change_date(date);

    fixture_repo
        .save(&fixture, tx_ctx)
        .await
        .map_err(|e| e.to_string())?;

    domain_event_publisher
        .publish_domain_event(DomainEvent::FixtureDateChanged {
            fixture_id: fixture.id().clone(),
            date,
        })
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

pub async fn update_fixture_venue<TxCtx>(
    fixture_id: FixtureId,
    venue_id: VenueId,
    fixture_repo: &impl FixtureRepository<TxCtx = TxCtx, Error = String>,
    venue_resolver: &impl VenueResolver<Error = String>,
    domain_event_publisher: &Box<dyn DomainEventPublisher + Send + Sync>,
    tx_ctx: &mut TxCtx,
) -> Result<(), String> {
    let mut fixture = fixture_repo
        .find_by_id(fixture_id, tx_ctx)
        .await?
        .expect("Fixture not found");

    let _venue = venue_resolver
        .resolve(&venue_id)
        .await
        .expect(&format!("Venue {:?} not resolved", venue_id));

    fixture.change_venue(venue_id);

    fixture_repo
        .save(&fixture, tx_ctx)
        .await
        .map_err(|e| e.to_string())?;

    domain_event_publisher
        .publish_domain_event(DomainEvent::FixtureVenueChanged {
            fixture_id: fixture.id().clone(),
            venue_id,
        })
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

pub async fn cancel_fixture<TxCtx>(
    fixture_id: FixtureId,
    fixture_repo: &impl FixtureRepository<TxCtx = TxCtx, Error = String>,
    domain_event_publisher: &Box<dyn DomainEventPublisher + Send + Sync>,
    tx_ctx: &mut TxCtx,
) -> Result<Fixture, String> {
    let mut fixture = fixture_repo
        .find_by_id(fixture_id, tx_ctx)
        .await?
        .expect("Fixture not found");

    if fixture.is_cancelled() {
        return Err("Fixture is already cancelled".to_string());
    }

    fixture.cancel();

    fixture_repo
        .save(&fixture, tx_ctx)
        .await
        .map_err(|e| e.to_string())?;

    domain_event_publisher
        .publish_domain_event(DomainEvent::FixtureCancelled {
            fixture_id: fixture.id().clone(),
        })
        .await
        .map_err(|e| e.to_string())?;

    Ok(fixture)
}

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use microservices_shared::{
        domain_events::{DomainEventPublisher, MockDomainEventPublisher},
        domain_ids::{FixtureId, TeamId, VenueId},
        resolvers::traits::{MockTeamResolver, MockVenueResolver},
    };
    use mockall::predicate::eq;
    use restinterface::{FixtureDTO, FixtureStatusDTO, TeamDTO, VenueDTO};
    use uuid::Uuid;

    use crate::{
        application::fixture_services::cancel_fixture,
        domain::{
            aggregates::fixture::{Fixture, FixtureStatus},
            repositories::fixture_repo::MockFixtureRepository,
        },
    };

    use super::create_fixture;

    #[tokio::test]
    async fn test_create_fixture() {
        let now = Utc::now();
        let mut fixture_repo = MockFixtureRepository::new();
        let mut venue_resolver = MockVenueResolver::new();
        let mut team_resolver = MockTeamResolver::new();
        let domain_event_publisher: Box<dyn DomainEventPublisher + Send + Sync> =
            Box::new(MockDomainEventPublisher::new());

        let venue_id = VenueId::from(Uuid::new_v4());
        let team_home_id = TeamId::from(Uuid::new_v4());
        let team_away_id = TeamId::from(Uuid::new_v4());

        let venue_dto = VenueDTO {
            id: venue_id.into(),
            name: "Venue A".to_string(),
            street: "Location A".to_string(),
            zip: "12345".to_string(),
            city: "City A".to_string(),
            telephone: None,
            email: None,
        };

        let team_home_dto = TeamDTO {
            id: team_home_id.into(),
            name: "Team A".to_string(),
            club: "Club A".to_string(),
        };

        let team_away_dto = TeamDTO {
            id: team_away_id.into(),
            name: "Team B".to_string(),
            club: "Club B".to_string(),
        };

        venue_resolver
            .expect_resolve()
            .with(eq(venue_id))
            .return_const(Ok(venue_dto.clone()));

        team_resolver
            .expect_resolve()
            .with(eq(team_home_id))
            .return_const(Ok(team_home_dto.clone()));

        team_resolver
            .expect_resolve()
            .with(eq(team_away_id))
            .return_const(Ok(team_away_dto.clone()));

        fixture_repo
            .expect_find_by_day_and_venue()
            .return_const(Ok(vec![]));

        fixture_repo
            .expect_find_by_day_and_team()
            .return_const(Ok(vec![]));

        fixture_repo.expect_save().return_const(Ok(()));

        let fixture_created = create_fixture(
            now,
            venue_id,
            team_home_id,
            team_away_id,
            &fixture_repo,
            &venue_resolver,
            &team_resolver,
            &domain_event_publisher,
            &mut (),
        )
        .await
        .unwrap();

        let fixture_expected = FixtureDTO {
            id: fixture_created.id.into(),
            date: now,
            status: FixtureStatusDTO::Scheduled,
            venue: venue_dto.clone(),
            team_home: team_home_dto.clone(),
            team_away: team_away_dto.clone(),
            first_referee: None,
            second_referee: None,
        };

        assert_eq!(fixture_created, fixture_expected);
    }

    #[tokio::test]
    async fn test_given_scheduled_fixture_when_cancel_then_cancelled() {
        let now = Utc::now();
        let mut fixture_repo = MockFixtureRepository::new();
        let domain_event_publisher: Box<dyn DomainEventPublisher + Send + Sync> =
            Box::new(MockDomainEventPublisher::new());

        let fixture_id = FixtureId::from(Uuid::new_v4());
        let venue_id = VenueId::from(Uuid::new_v4());
        let team_home_id = TeamId::from(Uuid::new_v4());
        let team_away_id = TeamId::from(Uuid::new_v4());

        let fixture = Fixture::from_id(
            fixture_id,
            now,
            FixtureStatus::Scheduled,
            venue_id,
            team_home_id,
            team_away_id,
            None,
            None,
        );

        fixture_repo
            .expect_find_by_id()
            .with(eq(fixture_id), eq(&()))
            .return_const(Ok(Some(fixture.clone())));
        fixture_repo.expect_save().return_const(Ok(()));

        let fixture_cancelled =
            cancel_fixture(fixture_id, &fixture_repo, &domain_event_publisher, &mut ())
                .await
                .unwrap();

        assert!(fixture_cancelled.is_cancelled());
    }

    #[tokio::test]
    #[should_panic]
    async fn test_given_cancelled_fixture_when_cancel_then_panic() {
        let now = Utc::now();
        let mut fixture_repo = MockFixtureRepository::new();
        let domain_event_publisher: Box<dyn DomainEventPublisher + Send + Sync> =
            Box::new(MockDomainEventPublisher::new());

        let fixture_id = FixtureId::from(Uuid::new_v4());
        let venue_id = VenueId::from(Uuid::new_v4());
        let team_home_id = TeamId::from(Uuid::new_v4());
        let team_away_id = TeamId::from(Uuid::new_v4());

        let fixture = Fixture::from_id(
            fixture_id,
            now,
            FixtureStatus::Cancelled,
            venue_id,
            team_home_id,
            team_away_id,
            None,
            None,
        );

        fixture_repo
            .expect_find_by_id()
            .with(eq(fixture_id), eq(&()))
            .return_const(Ok(Some(fixture.clone())));
        fixture_repo.expect_save().return_const(Ok(()));

        cancel_fixture(fixture_id, &fixture_repo, &domain_event_publisher, &mut ())
            .await
            .unwrap();
    }
}
