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

pub async fn create_fixture<DbTx>(
    date: DateTime<Utc>,
    venue_id: VenueId,
    team_home_id: TeamId,
    team_away_id: TeamId,
    tx: &mut DbTx,
    fixture_repo: &impl FixtureRepository<Error = String>,
    venue_repo: &mut impl VenueRepository<Tx = DbTx, Error = String>,
    team_repo: &mut impl TeamRepository<Error = String>,
) -> Result<Fixture, String> {
    // TODO: write full test coverage because its a complex use case - use mocks with mockall

    let venue = venue_repo
        .find_by_id(&venue_id, tx)
        .await?
        .expect("Venue not found");
    let team_home = team_repo
        .find_by_id(&team_home_id)
        .await?
        .expect("Team home not found");
    let team_away = team_repo
        .find_by_id(&team_away_id)
        .await?
        .expect("Team away not found");

    if team_home.id() == team_away.id() {
        return Err("Team home and team away cannot be the same".to_string());
    }

    // we simplify the constraint to no other fixture at the same venue on the same day
    let fixtures = fixture_repo.find_by_day_and_venue(&date, &venue_id).await?;
    if !fixtures.is_empty() {
        return Err("There is already a fixture at the same venue on the same day".to_string());
    }

    // we simplify the constraint to no other fixture at the same day for the same team
    let fixtures = fixture_repo
        .find_by_day_and_team(&date, &team_home_id)
        .await?;
    if !fixtures.is_empty() {
        return Err("There is already a fixture at the same day for the home team".to_string());
    }
    let fixtures = fixture_repo
        .find_by_day_and_team(&date, &team_away_id)
        .await?;
    if !fixtures.is_empty() {
        return Err("There is already a fixture at the same day for the away team".to_string());
    }

    let fixture = Fixture::new(date, venue, team_home, team_away);

    fixture_repo
        .save(&fixture)
        .await
        .map_err(|e| e.to_string())?;
    Ok(fixture)
}

pub async fn update_fixture_date(
    fixture_id: FixtureId,
    date: DateTime<Utc>,
    fixture_repo: &impl FixtureRepository<Error = String>,
) -> Result<Fixture, String> {
    let mut fixture = fixture_repo
        .find_by_id(&fixture_id)
        .await?
        .expect("Fixture not found");

    fixture.change_date(date);

    fixture_repo
        .save(&fixture)
        .await
        .map_err(|e| e.to_string())?;

    Ok(fixture)
}

pub async fn update_fixture_venue<Tx>(
    fixture_id: FixtureId,
    venue_id: VenueId,
    tx: &mut Tx,
    fixture_repo: &impl FixtureRepository<Error = String>,
    venue_repo: &mut impl VenueRepository<Tx = Tx, Error = String>,
) -> Result<Fixture, String> {
    let mut fixture = fixture_repo
        .find_by_id(&fixture_id)
        .await?
        .expect("Fixture not found");

    let venue = venue_repo
        .find_by_id(&venue_id, tx)
        .await?
        .expect("Venue not found");

    fixture.change_venue(venue);

    fixture_repo
        .save(&fixture)
        .await
        .map_err(|e| e.to_string())?;

    Ok(fixture)
}

pub async fn cancel_fixture(
    fixture_id: FixtureId,
    fixture_repo: &impl FixtureRepository<Error = String>,
) -> Result<Fixture, String> {
    let mut fixture = fixture_repo
        .find_by_id(&fixture_id)
        .await?
        .expect("Fixture not found");

    if fixture.is_cancelled() {
        return Err("Fixture is already cancelled".to_string());
    }

    fixture.cancel();

    fixture_repo
        .save(&fixture)
        .await
        .map_err(|e| e.to_string())?;

    Ok(fixture)
}
