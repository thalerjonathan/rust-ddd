use microservices_shared::domain_ids::RefereeId;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Referee {
    id: RefereeId,
    name: String,
    club: String,
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
