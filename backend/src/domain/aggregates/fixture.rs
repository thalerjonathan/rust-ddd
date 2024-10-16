use std::str::FromStr;

use chrono::{DateTime, Utc};
use uuid::Uuid;

use super::{referee::Referee, team::Team, venue::Venue};

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FixtureStatus {
    Scheduled,
    Cancelled,
}

#[derive(Debug, Clone)]
pub struct Fixture {
    id: FixtureId,
    date: DateTime<Utc>,
    venue: Venue,
    team_home: Team,
    team_away: Team,
    status: FixtureStatus,
    first_referee: Option<Referee>,
    second_referee: Option<Referee>,
}

impl Fixture {
    pub fn new(
        date: DateTime<Utc>,
        venue: Venue,
        team_home: Team,
        team_away: Team,
        first_referee: Option<Referee>,
        second_referee: Option<Referee>,
    ) -> Self {
        Self {
            id: FixtureId(Uuid::new_v4()),
            date,
            venue,
            team_home,
            team_away,
            status: FixtureStatus::Scheduled,
            first_referee,
            second_referee,
        }
    }

    pub fn from_id(
        id: FixtureId,
        date: DateTime<Utc>,
        status: FixtureStatus,
        venue: Venue,
        team_home: Team,
        team_away: Team,
        first_referee: Option<Referee>,
        second_referee: Option<Referee>,
    ) -> Self {
        Self {
            id,
            date,
            venue,
            team_home,
            team_away,
            status,
            first_referee,
            second_referee,
        }
    }

    pub fn id(&self) -> FixtureId {
        self.id
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

    pub fn status(&self) -> &FixtureStatus {
        &self.status
    }

    pub fn first_referee(&self) -> Option<&Referee> {
        self.first_referee.as_ref()
    }

    pub fn second_referee(&self) -> Option<&Referee> {
        self.second_referee.as_ref()
    }

    pub fn is_scheduled(&self) -> bool {
        self.status == FixtureStatus::Scheduled
    }

    pub fn is_cancelled(&self) -> bool {
        self.status == FixtureStatus::Cancelled
    }

    pub fn cancel(&mut self) {
        if self.status != FixtureStatus::Scheduled {
            // NOTE: this is not how we would like to handle this in a real application
            panic!("Fixture is not scheduled");
        }

        self.status = FixtureStatus::Cancelled;
    }

    pub fn change_venue(&mut self, venue: Venue) {
        self.venue = venue;
    }

    pub fn change_date(&mut self, date: DateTime<Utc>) {
        self.date = date;
    }
}
