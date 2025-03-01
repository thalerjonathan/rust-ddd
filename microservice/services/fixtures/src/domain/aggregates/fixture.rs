use chrono::{DateTime, Utc};
use microservices_shared::domain_ids::{FixtureId, RefereeId, TeamId, VenueId};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FixtureStatus {
    Scheduled,
    Cancelled,
}

#[derive(Debug, Clone)]
pub struct Fixture {
    id: FixtureId,
    date: DateTime<Utc>,
    status: FixtureStatus,
    venue_id: VenueId,
    team_home_id: TeamId,
    team_away_id: TeamId,
    first_referee_id: Option<RefereeId>,
    second_referee_id: Option<RefereeId>,
}

impl Fixture {
    pub fn new(
        date: DateTime<Utc>,
        venue_id: VenueId,
        team_home_id: TeamId,
        team_away_id: TeamId,
        first_referee_id: Option<RefereeId>,
        second_referee_id: Option<RefereeId>,
    ) -> Self {
        Self {
            id: FixtureId(Uuid::new_v4()),
            date,
            status: FixtureStatus::Scheduled,
            venue_id,
            team_home_id,
            team_away_id,
            first_referee_id,
            second_referee_id,
        }
    }

    pub fn from_id(
        id: FixtureId,
        date: DateTime<Utc>,
        status: FixtureStatus,
        venue_id: VenueId,
        team_home_id: TeamId,
        team_away_id: TeamId,
        first_referee_id: Option<RefereeId>,
        second_referee_id: Option<RefereeId>,
    ) -> Self {
        Self {
            id,
            date,
            status,
            venue_id,
            team_home_id,
            team_away_id,
            first_referee_id,
            second_referee_id,
        }
    }

    pub fn id(&self) -> FixtureId {
        self.id
    }

    pub fn date(&self) -> &DateTime<Utc> {
        &self.date
    }

    pub fn venue_id(&self) -> &VenueId {
        &self.venue_id
    }

    pub fn team_home_id(&self) -> &TeamId {
        &self.team_home_id
    }

    pub fn team_away_id(&self) -> &TeamId {
        &self.team_away_id
    }

    pub fn status(&self) -> &FixtureStatus {
        &self.status
    }

    pub fn first_referee_id(&self) -> Option<&RefereeId> {
        self.first_referee_id.as_ref()
    }

    pub fn second_referee_id(&self) -> Option<&RefereeId> {
        self.second_referee_id.as_ref()
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

    pub fn change_venue(&mut self, venue_id: VenueId) {
        self.venue_id = venue_id;
    }

    pub fn change_date(&mut self, date: DateTime<Utc>) {
        self.date = date;
    }

    pub fn assign_first_referee(&mut self, referee_id: RefereeId) {
        if self.first_referee_id.is_some() {
            // NOTE: this is not how we would like to handle this in a real application
            panic!("First referee already assigned");
        }

        self.first_referee_id = Some(referee_id);
    }

    pub fn assign_second_referee(&mut self, referee_id: RefereeId) {
        if self.second_referee_id.is_some() {
            // NOTE: this is not how we would like to handle this in a real application
            panic!("Second referee already assigned");
        }

        self.second_referee_id = Some(referee_id);
    }

    pub fn unassign_first_referee(&mut self) {
        if self.first_referee_id.is_none() {
            // NOTE: this is not how we would like to handle this in a real application
            panic!("First referee not assigned");
        }

        self.first_referee_id = None;
    }

    pub fn unassign_second_referee(&mut self) {
        if self.second_referee_id.is_none() {
            // NOTE: this is not how we would like to handle this in a real application
            panic!("Second referee not assigned");
        }

        self.second_referee_id = None;
    }
}
