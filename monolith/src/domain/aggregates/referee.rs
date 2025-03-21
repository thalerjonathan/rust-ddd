use std::str::FromStr;

use uuid::Uuid;

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub struct RefereeId(pub Uuid);

#[derive(Debug, Clone)]
pub struct Referee {
    id: RefereeId,
    name: String,
    club: String,
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

impl Referee {
    pub fn new(name: &str, club: &str) -> Self {
        Self {
            id: RefereeId(Uuid::new_v4()),
            name: name.to_string(),
            club: club.to_string(),
        }
    }

    pub fn from_id(id: Uuid, name: String, club: String) -> Self {
        Self {
            id: RefereeId(id),
            name,
            club,
        }
    }

    pub fn id(&self) -> RefereeId {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn club(&self) -> &str {
        &self.club
    }

    pub fn change_club(&mut self, new_club: &str) {
        self.club = new_club.to_string();
    }
}
