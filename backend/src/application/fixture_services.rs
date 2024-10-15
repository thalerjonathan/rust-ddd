use chrono::{DateTime, Utc};

use crate::domain::{
    aggregates::{
        fixture::{Fixture, FixtureId},
        team::TeamId,
        venue::VenueId,
    },
    repositories::{
        fixture_repo::FixtureRepository, team_repo::TeamRepository, venue_repo::VenueRepository,
    },
};

pub async fn create_fixture<TxCtx>(
    date: DateTime<Utc>,
    venue_id: VenueId,
    team_home_id: TeamId,
    team_away_id: TeamId,
    fixture_repo: &impl FixtureRepository<TxCtx = TxCtx, Error = String>,
    venue_repo: &impl VenueRepository<TxCtx = TxCtx, Error = String>,
    team_repo: &impl TeamRepository<TxCtx = TxCtx, Error = String>,
    tx_ctx: &mut TxCtx,
) -> Result<Fixture, String> {
    let venue = venue_repo
        .find_by_id(&venue_id, tx_ctx)
        .await?
        .expect("Venue not found");
    let team_home = team_repo
        .find_by_id(&team_home_id, tx_ctx)
        .await?
        .expect("Team home not found");
    let team_away = team_repo
        .find_by_id(&team_away_id, tx_ctx)
        .await?
        .expect("Team away not found");

    if team_home.id() == team_away.id() {
        return Err("Team home and team away cannot be the same".to_string());
    }

    // we simplify the constraint to no other fixture at the same venue on the same day
    let fixtures = fixture_repo
        .find_by_day_and_venue(&date, &venue_id, tx_ctx)
        .await?;
    if !fixtures.is_empty() {
        return Err("There is already a fixture at the same venue on the same day".to_string());
    }

    // we simplify the constraint to no other fixture at the same day for the same team
    let fixtures = fixture_repo
        .find_by_day_and_team(&date, &team_home_id, tx_ctx)
        .await?;
    if !fixtures.is_empty() {
        return Err("There is already a fixture at the same day for the home team".to_string());
    }
    let fixtures = fixture_repo
        .find_by_day_and_team(&date, &team_away_id, tx_ctx)
        .await?;
    if !fixtures.is_empty() {
        return Err("There is already a fixture at the same day for the away team".to_string());
    }

    let fixture = Fixture::new(date, venue, team_home, team_away);

    fixture_repo
        .save(&fixture, tx_ctx)
        .await
        .map_err(|e| e.to_string())?;
    Ok(fixture)
}

pub async fn update_fixture_date<TxCtx>(
    fixture_id: FixtureId,
    date: DateTime<Utc>,
    fixture_repo: &impl FixtureRepository<TxCtx = TxCtx, Error = String>,
    tx_ctx: &mut TxCtx,
) -> Result<Fixture, String> {
    let mut fixture = fixture_repo
        .find_by_id(&fixture_id, tx_ctx)
        .await?
        .expect("Fixture not found");

    fixture.change_date(date);

    fixture_repo
        .save(&fixture, tx_ctx)
        .await
        .map_err(|e| e.to_string())?;

    Ok(fixture)
}

pub async fn update_fixture_venue<TxCtx>(
    fixture_id: FixtureId,
    venue_id: VenueId,
    fixture_repo: &impl FixtureRepository<TxCtx = TxCtx, Error = String>,
    venue_repo: &impl VenueRepository<TxCtx = TxCtx, Error = String>,
    tx_ctx: &mut TxCtx,
) -> Result<Fixture, String> {
    let mut fixture = fixture_repo
        .find_by_id(&fixture_id, tx_ctx)
        .await?
        .expect("Fixture not found");

    let venue = venue_repo
        .find_by_id(&venue_id, tx_ctx)
        .await?
        .expect("Venue not found");

    fixture.change_venue(venue);

    fixture_repo
        .save(&fixture, tx_ctx)
        .await
        .map_err(|e| e.to_string())?;

    Ok(fixture)
}

pub async fn cancel_fixture<TxCtx>(
    fixture_id: FixtureId,
    fixture_repo: &impl FixtureRepository<TxCtx = TxCtx, Error = String>,
    tx_ctx: &mut TxCtx,
) -> Result<Fixture, String> {
    let mut fixture = fixture_repo
        .find_by_id(&fixture_id, tx_ctx)
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

    Ok(fixture)
}

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use mockall::predicate::eq;
    use uuid::Uuid;

    use crate::{
        application::fixture_services::cancel_fixture,
        domain::{
            aggregates::{
                fixture::{Fixture, FixtureId, FixtureStatus},
                team::{Team, TeamId},
                venue::{Venue, VenueId},
            },
            repositories::{
                fixture_repo::MockFixtureRepository, team_repo::MockTeamRepository,
                venue_repo::MockVenueRepository,
            },
        },
    };

    use super::create_fixture;

    #[tokio::test]
    async fn test_create_fixture() {
        let now = Utc::now();
        let mut fixture_repo = MockFixtureRepository::new();
        let mut venue_repo = MockVenueRepository::new();
        let mut team_repo = MockTeamRepository::new();

        let venue_id = VenueId::from(Uuid::new_v4());
        let team_home_id = TeamId::from(Uuid::new_v4());
        let team_away_id = TeamId::from(Uuid::new_v4());
        let venue = Venue::from_id(
            venue_id,
            "Venue A".to_string(),
            "Location A".to_string(),
            "12345".to_string(),
            "City A".to_string(),
            None,
            None,
        );
        let team_home = Team::from_id(team_home_id, "Team A".to_string(), "Club A".to_string());
        let team_away = Team::from_id(team_away_id, "Team B".to_string(), "Club B".to_string());

        venue_repo
            .expect_find_by_id()
            .with(eq(venue_id), eq(&()))
            .return_const(Ok(Some(venue.clone())));

        team_repo
            .expect_find_by_id()
            .with(eq(team_home_id), eq(&()))
            .return_const(Ok(Some(team_home.clone())));

        team_repo
            .expect_find_by_id()
            .with(eq(team_away_id), eq(&()))
            .return_const(Ok(Some(team_away.clone())));

        fixture_repo
            .expect_find_by_day_and_venue()
            .return_const(Ok(vec![]));

        fixture_repo
            .expect_find_by_day_and_team()
            .return_const(Ok(vec![]));

        fixture_repo.expect_save().return_const(Ok(()));

        let fixture_created = create_fixture(
            now,
            venue.id(),
            team_home.id(),
            team_away.id(),
            &fixture_repo,
            &venue_repo,
            &team_repo,
            &mut (),
        )
        .await
        .unwrap();

        let fixture_expected = Fixture::from_id(
            fixture_created.id(),
            now,
            FixtureStatus::Scheduled,
            venue,
            team_home,
            team_away,
        );

        assert_eq!(fixture_created, fixture_expected);
    }

    #[tokio::test]
    async fn test_given_scheduled_fixture_when_cancel_then_cancelled() {
        let now = Utc::now();
        let mut fixture_repo = MockFixtureRepository::new();

        let fixture_id = FixtureId::from(Uuid::new_v4());
        let venue_id = VenueId::from(Uuid::new_v4());
        let team_home_id = TeamId::from(Uuid::new_v4());
        let team_away_id = TeamId::from(Uuid::new_v4());
        let venue = Venue::from_id(
            venue_id,
            "Venue A".to_string(),
            "Location A".to_string(),
            "12345".to_string(),
            "City A".to_string(),
            None,
            None,
        );
        let team_home = Team::from_id(team_home_id, "Team A".to_string(), "Club A".to_string());
        let team_away = Team::from_id(team_away_id, "Team B".to_string(), "Club B".to_string());

        let fixture = Fixture::from_id(
            fixture_id,
            now,
            FixtureStatus::Scheduled,
            venue,
            team_home,
            team_away,
        );

        fixture_repo
            .expect_find_by_id()
            .with(eq(fixture_id), eq(&()))
            .return_const(Ok(Some(fixture.clone())));
        fixture_repo.expect_save().return_const(Ok(()));

        let fixture_cancelled = cancel_fixture(fixture_id, &fixture_repo, &mut ())
            .await
            .unwrap();

        assert!(fixture_cancelled.is_cancelled());
    }

    #[tokio::test]
    #[should_panic]
    async fn test_given_cancelled_fixture_when_cancel_then_panic() {
        let now = Utc::now();
        let mut fixture_repo = MockFixtureRepository::new();

        let fixture_id = FixtureId::from(Uuid::new_v4());
        let venue_id = VenueId::from(Uuid::new_v4());
        let team_home_id = TeamId::from(Uuid::new_v4());
        let team_away_id = TeamId::from(Uuid::new_v4());
        let venue = Venue::from_id(
            venue_id,
            "Venue A".to_string(),
            "Location A".to_string(),
            "12345".to_string(),
            "City A".to_string(),
            None,
            None,
        );
        let team_home = Team::from_id(team_home_id, "Team A".to_string(), "Club A".to_string());
        let team_away = Team::from_id(team_away_id, "Team B".to_string(), "Club B".to_string());

        let fixture = Fixture::from_id(
            fixture_id,
            now,
            FixtureStatus::Cancelled,
            venue,
            team_home,
            team_away,
        );

        fixture_repo
            .expect_find_by_id()
            .with(eq(fixture_id), eq(&()))
            .return_const(Ok(Some(fixture.clone())));
        fixture_repo.expect_save().return_const(Ok(()));

        cancel_fixture(fixture_id, &fixture_repo, &mut ())
            .await
            .unwrap();
    }
}
