use std::str::FromStr;

use uuid::Uuid;

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub struct TeamId(pub Uuid);

#[derive(Debug, Clone)]
pub struct Team {
    id: TeamId,
    name: String,
    club: String,
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

impl Team {
    pub fn new(name: &str, club: &str) -> Self {
        Self {
            id: TeamId(Uuid::new_v4()),
            name: name.to_string(),
            club: club.to_string(),
        }
    }

    pub fn from_id(id: TeamId, name: String, club: String) -> Self {
        Self { id, name, club }
    }

    pub fn id(&self) -> TeamId {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn club(&self) -> &str {
        &self.club
    }
}
