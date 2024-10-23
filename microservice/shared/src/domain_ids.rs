use std::str::FromStr;

use restinterface::{FixtureIdDTO, RefereeIdDTO, TeamIdDTO, VenueIdDTO};
use uuid::Uuid;

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub struct FixtureId(pub Uuid);

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub struct RefereeId(pub Uuid);

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub struct TeamId(pub Uuid);

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub struct VenueId(pub Uuid);

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

impl TryFrom<String> for RefereeId {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Uuid::from_str(&value)
            .map_err(|e| e.to_string())
            .map(RefereeId)
    }
}

impl From<Uuid> for RefereeId {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

impl TryFrom<String> for TeamId {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Uuid::from_str(&value)
            .map_err(|e| e.to_string())
            .map(TeamId)
    }
}

impl From<Uuid> for TeamId {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

impl TryFrom<String> for VenueId {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Uuid::from_str(&value)
            .map_err(|e| e.to_string())
            .map(VenueId)
    }
}

impl From<Uuid> for VenueId {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

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

impl From<FixtureIdDTO> for FixtureId {
    fn from(id: FixtureIdDTO) -> Self {
        Self(id.0)
    }
}

impl From<FixtureId> for FixtureIdDTO {
    fn from(id: FixtureId) -> Self {
        FixtureIdDTO(id.0)
    }
}

impl From<TeamIdDTO> for TeamId {
    fn from(id: TeamIdDTO) -> Self {
        Self(id.0)
    }
}

impl From<TeamId> for TeamIdDTO {
    fn from(id: TeamId) -> Self {
        TeamIdDTO(id.0)
    }
}

impl From<RefereeIdDTO> for RefereeId {
    fn from(value: RefereeIdDTO) -> Self {
        Self(value.0)
    }
}

impl From<RefereeId> for RefereeIdDTO {
    fn from(id: RefereeId) -> Self {
        RefereeIdDTO(id.0)
    }
}
