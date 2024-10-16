use crate::domain::aggregates::fixture::Fixture;
use crate::domain::aggregates::referee::Referee;
use crate::domain::aggregates::team::Team;
use crate::domain::aggregates::venue::Venue;

// NOTE: This file contains utility functions to assert the values of entities and aggregates.
// The reason we use these functions is because if we use the assert_eq! macro, the tests will
// require us to implement the Eq trait for all the entities and aggregates, which is
// semantically wrong for Entities and Aggregates in DDD because they are equal only
// when they have the same identity.
// Is generally only used in tests.

#[allow(dead_code)]
pub fn assert_venues_values_eq(venue1: &Venue, venue2: &Venue) {
    assert_eq!(venue1.id(), venue2.id());
    assert_eq!(venue1.name(), venue2.name());
    assert_eq!(venue1.street(), venue2.street());
    assert_eq!(venue1.zip(), venue2.zip());
    assert_eq!(venue1.city(), venue2.city());
    assert_eq!(venue1.telephone(), venue2.telephone());
    assert_eq!(venue1.email(), venue2.email());
}

#[allow(dead_code)]
pub fn assert_teams_values_eq(team1: &Team, team2: &Team) {
    assert_eq!(team1.id(), team2.id());
    assert_eq!(team1.name(), team2.name());
    assert_eq!(team1.club(), team2.club());
}

#[allow(dead_code)]
pub fn assert_referees_values_eq(referee1: &Referee, referee2: &Referee) {
    assert_eq!(referee1.id(), referee2.id());
    assert_eq!(referee1.name(), referee2.name());
    assert_eq!(referee1.club(), referee2.club());
}

#[allow(dead_code)]
pub fn assert_fixtures_values_eq(fixture1: &Fixture, fixture2: &Fixture) {
    assert_eq!(fixture1.id(), fixture2.id());
    assert_eq!(fixture1.date(), fixture2.date());
    assert_eq!(fixture1.status(), fixture2.status());

    assert_venues_values_eq(&fixture1.venue(), &fixture2.venue());
    assert_teams_values_eq(&fixture1.team_home(), &fixture2.team_home());
    assert_teams_values_eq(&fixture1.team_away(), &fixture2.team_away());

    assert_options_eq_with_comparator(
        &fixture1.first_referee(),
        &fixture2.first_referee(),
        assert_referees_values_eq,
    );
    assert_options_eq_with_comparator(
        &fixture1.second_referee(),
        &fixture2.second_referee(),
        assert_referees_values_eq,
    );
}

pub fn assert_options_eq_with_comparator<T>(
    option1: &Option<&T>,
    option2: &Option<&T>,
    comparator: fn(&T, &T),
) {
    match (option1, option2) {
        (Some(v1), Some(v2)) => comparator(v1, v2),
        (None, None) => (),
        _ => panic!("One of the options is None"),
    }
}
