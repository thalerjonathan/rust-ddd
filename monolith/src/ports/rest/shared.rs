use chrono::Utc;
use shared::{
    AssignmentDTO, AssignmentRefereeRoleDTO, AssignmentStatusDTO, FixtureCreationDTO, FixtureDTO,
    FixtureIdDTO, FixtureStatusDTO, RefereeDTO, RefereeIdDTO, TeamCreationDTO, TeamDTO, TeamIdDTO,
    VenueCreationDTO,
};

use crate::domain::aggregates::{
    assignment::{Assignment, AssignmentRefereeRole, AssignmentStatus},
    fixture::{Fixture, FixtureId, FixtureStatus},
    referee::{Referee, RefereeId},
    team::{Team, TeamId},
};

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

#[allow(dead_code)]
pub async fn create_test_fixture() -> (FixtureCreationDTO, FixtureDTO) {
    let team_home = shared::create_team(&TeamCreationDTO {
        name: "Team A".to_string(),
        club: "Club A".to_string(),
    })
    .await
    .unwrap();

    let team_away = shared::create_team(&TeamCreationDTO {
        name: "Team B".to_string(),
        club: "Club B".to_string(),
    })
    .await
    .unwrap();

    let venue = shared::create_venue(&VenueCreationDTO {
        name: "Venue A".to_string(),
        street: "Street A".to_string(),
        zip: "12345".to_string(),
        city: "City A".to_string(),
        telephone: Some("1234567890".to_string()),
        email: Some("email@example.com".to_string()),
    })
    .await
    .unwrap();

    let fixture_creation = FixtureCreationDTO {
        date: Utc::now(),
        venue_id: venue.id,
        team_home_id: team_home.id,
        team_away_id: team_away.id,
    };

    let fixture_dto = shared::create_fixture(&fixture_creation).await;
    assert!(fixture_dto.is_ok(), "Fixture should be created");

    (fixture_creation, fixture_dto.unwrap())
}
