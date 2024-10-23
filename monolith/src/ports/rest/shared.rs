use restinterface::{
    AssignmentDTO, AssignmentRefereeRoleDTO, AssignmentStatusDTO, FixtureDTO, FixtureIdDTO,
    FixtureStatusDTO, RefereeDTO, RefereeIdDTO, TeamDTO, TeamIdDTO, VenueDTO, VenueIdDTO,
};

use crate::domain::aggregates::{
    assignment::{Assignment, AssignmentRefereeRole, AssignmentStatus},
    fixture::{Fixture, FixtureId, FixtureStatus},
    referee::{Referee, RefereeId},
    team::{Team, TeamId},
    venue::{Venue, VenueId},
};

impl From<VenueIdDTO> for VenueId {
    fn from(value: VenueIdDTO) -> Self {
        Self(value.0)
    }
}

impl From<VenueId> for VenueIdDTO {
    fn from(id: VenueId) -> Self {
        VenueIdDTO(id.0)
    }
}

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

impl From<FixtureStatus> for FixtureStatusDTO {
    fn from(status: FixtureStatus) -> Self {
        match status {
            FixtureStatus::Scheduled => FixtureStatusDTO::Scheduled,
            FixtureStatus::Cancelled => FixtureStatusDTO::Cancelled,
        }
    }
}

impl From<Fixture> for FixtureDTO {
    fn from(fixture: Fixture) -> Self {
        FixtureDTO {
            id: fixture.id().into(),
            date: fixture.date().clone(),
            venue: fixture.venue().clone().into(),
            team_home: fixture.team_home().clone().into(),
            team_away: fixture.team_away().clone().into(),
            status: fixture.status().clone().into(),
            first_referee: fixture.first_referee().map(|r| r.clone().into()),
            second_referee: fixture.second_referee().map(|r| r.clone().into()),
        }
    }
}

impl From<Referee> for RefereeDTO {
    fn from(referee: Referee) -> Self {
        RefereeDTO {
            id: referee.id().into(),
            name: referee.name().to_string(),
            club: referee.club().to_string(),
        }
    }
}

impl From<Team> for TeamDTO {
    fn from(team: Team) -> Self {
        Self {
            id: team.id().into(),
            name: team.name().to_string(),
            club: team.club().to_string(),
        }
    }
}

impl From<RefereeIdDTO> for RefereeId {
    fn from(value: RefereeIdDTO) -> Self {
        Self(value.0)
    }
}

impl From<TeamIdDTO> for TeamId {
    fn from(id: TeamIdDTO) -> Self {
        Self(id.0)
    }
}

impl From<FixtureIdDTO> for FixtureId {
    fn from(id: FixtureIdDTO) -> Self {
        Self(id.0)
    }
}

impl From<RefereeId> for RefereeIdDTO {
    fn from(id: RefereeId) -> Self {
        RefereeIdDTO(id.0)
    }
}

impl From<TeamId> for TeamIdDTO {
    fn from(id: TeamId) -> Self {
        TeamIdDTO(id.0)
    }
}

impl From<FixtureId> for FixtureIdDTO {
    fn from(id: FixtureId) -> Self {
        FixtureIdDTO(id.0)
    }
}

impl From<AssignmentRefereeRole> for AssignmentRefereeRoleDTO {
    fn from(role: AssignmentRefereeRole) -> Self {
        match role {
            AssignmentRefereeRole::First => AssignmentRefereeRoleDTO::First,
            AssignmentRefereeRole::Second => AssignmentRefereeRoleDTO::Second,
        }
    }
}

impl From<AssignmentRefereeRoleDTO> for AssignmentRefereeRole {
    fn from(role: AssignmentRefereeRoleDTO) -> Self {
        match role {
            AssignmentRefereeRoleDTO::First => AssignmentRefereeRole::First,
            AssignmentRefereeRoleDTO::Second => AssignmentRefereeRole::Second,
        }
    }
}

impl From<AssignmentStatus> for AssignmentStatusDTO {
    fn from(status: AssignmentStatus) -> Self {
        match status {
            AssignmentStatus::Committed => AssignmentStatusDTO::Committed,
            AssignmentStatus::Staged => AssignmentStatusDTO::Staged,
        }
    }
}

impl From<Assignment> for AssignmentDTO {
    fn from(assignment: Assignment) -> Self {
        AssignmentDTO {
            status: assignment.status().into(),
            fixture_id: assignment.fixture_id().into(),
            referee_id: assignment.referee_id().into(),
            referee_role: assignment.referee_role().into(),
        }
    }
}
