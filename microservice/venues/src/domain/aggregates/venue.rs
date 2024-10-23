use microservices_shared::domain_ids::VenueId;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Venue {
    id: VenueId,
    name: String,
    street: String,
    zip: String,
    city: String,
    telephone: Option<String>,
    email: Option<String>,
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

    pub fn id(&self) -> VenueId {
        self.id
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
