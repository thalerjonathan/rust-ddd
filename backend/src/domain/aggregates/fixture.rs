use std::str::FromStr;

use chrono::{DateTime, Utc};
use uuid::Uuid;

use super::{team::Team, venue::Venue};

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub struct FixtureId(pub Uuid);

impl TryFrom<String> for FixtureId {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Uuid::from_str(&value)
            .map_err(|e| e.to_string())
            .map(FixtureId)
    }
}

impl From<Uuid> for FixtureId {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone)]
pub struct Fixture {
    id: FixtureId,
    date: DateTime<Utc>,
    venue: Venue,
    team_home: Team,
    team_away: Team,
}

impl Fixture {
    pub fn new(date: DateTime<Utc>, venue: Venue, team_home: Team, team_away: Team) -> Self {
        Self {
            id: FixtureId(Uuid::new_v4()),
            date,
            venue,
            team_home,
            team_away,
        }
    }

    pub fn from_id(
        id: Uuid,
        date: DateTime<Utc>,
        venue: Venue,
        team_home: Team,
        team_away: Team,
    ) -> Self {
        Self {
            id: FixtureId(id),
            date,
            venue,
            team_home,
            team_away,
        }
    }

    pub fn id(&self) -> &FixtureId {
        &self.id
    }

    pub fn date(&self) -> &DateTime<Utc> {
        &self.date
    }

    pub fn venue(&self) -> &Venue {
        &self.venue
    }

    pub fn team_home(&self) -> &Team {
        &self.team_home
    }

    pub fn team_away(&self) -> &Team {
        &self.team_away
    }
}
