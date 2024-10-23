use microservices_shared::domain_ids::TeamId;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Team {
    id: TeamId,
    name: String,
    club: String,
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
