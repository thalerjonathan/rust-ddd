use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RefereeDTO {
    pub id: Uuid,
    pub name: String,
    pub club: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RefereeCreationDTO {
    pub name: String,
    pub club: String,
}
