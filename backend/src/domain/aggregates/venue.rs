use std::str::FromStr;

use uuid::Uuid;

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub struct VenueId(pub Uuid);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Venue {
    id: VenueId,
    name: String,
    street: String,
    zip: String,
    city: String,
    telephone: Option<String>,
    email: Option<String>,
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

impl Venue {
    pub fn new(
        name: &str,
        street: &str,
        zip: &str,
        city: &str,
        telephone: Option<String>,
        email: Option<String>,
    ) -> Self {
        Self {
            id: VenueId(Uuid::new_v4()),
            name: name.to_string(),
            street: street.to_string(),
            zip: zip.to_string(),
            city: city.to_string(),
            telephone,
            email,
        }
    }

    pub fn from_id(
        id: VenueId,
        name: String,
        street: String,
        zip: String,
        city: String,
        telephone: Option<String>,
        email: Option<String>,
    ) -> Self {
        Self {
            id,
            name,
            street,
            zip,
            city,
            telephone,
            email,
        }
    }

    pub fn id(&self) -> &VenueId {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn street(&self) -> &str {
        &self.street
    }

    pub fn zip(&self) -> &str {
        &self.zip
    }

    pub fn city(&self) -> &str {
        &self.city
    }

    pub fn telephone(&self) -> Option<String> {
        self.telephone.clone()
    }

    pub fn email(&self) -> Option<String> {
        self.email.clone()
    }
}
