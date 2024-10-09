use chrono::{DateTime, Utc};

use crate::domain::{
    aggregates::{fixture::Fixture, team::TeamId, venue::VenueId},
    repositories::{
        fixture_repo::FixtureRepository, team_repo::TeamRepository, venue_repo::VenueRepository,
    },
};

pub async fn create_fixture(
    date: DateTime<Utc>,
    venue_id: VenueId,
    team_home_id: TeamId,
    team_away_id: TeamId,
    fixture_repo: &impl FixtureRepository<Error = String>,
    venue_repo: &impl VenueRepository<Error = String>,
    team_repo: &impl TeamRepository<Error = String>,
) -> Result<Fixture, String> {
    // TODO: check if venue, team_home and team_away exist
    // TODO: check if both teams are different
    // TODO: check if no other *Fixture* is scheduled at the same venue and time
    // TODO: check if the teams cannot have other *Fixtures* scheduled at the same time
    // TODO: check if date is in the future

    // TODO: write full test coverage because its a complex use case

    // TODO: handle errors
    let venue = venue_repo.find_by_id(&venue_id).await?.unwrap();
    let team_home = team_repo.find_by_id(&team_home_id).await?.unwrap();
    let team_away = team_repo.find_by_id(&team_away_id).await?.unwrap();

    let fixture = Fixture::new(date, venue, team_home, team_away);

    fixture_repo
        .save(&fixture)
        .await
        .map_err(|e| e.to_string())?;
    Ok(fixture)
}
