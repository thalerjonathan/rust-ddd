use chrono::{DateTime, Utc};
use reqwest::Url;
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VenueDTO {
    pub id: Uuid,
    pub name: String,
    pub street: String,
    pub zip: String,
    pub city: String,
    pub telephone: Option<String>,
    pub email: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VenueCreationDTO {
    pub name: String,
    pub street: String,
    pub zip: String,
    pub city: String,
    pub telephone: Option<String>,
    pub email: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TeamDTO {
    pub id: Uuid,
    pub name: String,
    pub club: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TeamCreationDTO {
    pub name: String,
    pub club: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum FixtureStatusDTO {
    Scheduled,
    Cancelled,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FixtureDTO {
    pub id: Uuid,
    pub team_home: TeamDTO,
    pub team_away: TeamDTO,
    pub venue: VenueDTO,
    pub date: DateTime<Utc>,
    pub status: FixtureStatusDTO,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FixtureCreationDTO {
    pub team_home_id: Uuid,
    pub team_away_id: Uuid,
    pub venue_id: Uuid,
    pub date: DateTime<Utc>,
}

pub async fn fetch_referees() -> Vec<RefereeDTO> {
    let url = Url::parse("http://localhost:3001/referees");
    let response = reqwest::Client::new().get(url.unwrap()).send().await;
    response.unwrap().json().await.unwrap()
}

pub async fn create_referee(
    ref_creation: &RefereeCreationDTO,
) -> Result<RefereeDTO, reqwest::Error> {
    let url = Url::parse("http://localhost:3001/referee").unwrap();
    let response = reqwest::Client::new()
        .post(url)
        .json(&ref_creation)
        .send()
        .await?;
    response.json().await
}

pub async fn fetch_referee(id: &str) -> RefereeDTO {
    let url = Url::parse(&format!("http://localhost:3001/referee/{}", id));
    let response = reqwest::Client::new().get(url.unwrap()).send().await;
    response.unwrap().json().await.unwrap()
}

pub async fn change_referee_club(referee_id: &str, club: &str) -> Result<String, reqwest::Error> {
    let url = Url::parse(&format!(
        "http://localhost:3001/referee/{}/club",
        referee_id
    ))
    .unwrap();
    let response = reqwest::Client::new().post(url).json(&club).send().await?;
    response.json().await
}

pub async fn fetch_venues() -> Vec<VenueDTO> {
    let url = Url::parse("http://localhost:3001/venues");
    let response = reqwest::Client::new().get(url.unwrap()).send().await;
    response.unwrap().json().await.unwrap()
}

pub async fn create_venue(venue_creation: &VenueCreationDTO) -> Result<VenueDTO, reqwest::Error> {
    let url = Url::parse("http://localhost:3001/venue").unwrap();
    let response = reqwest::Client::new()
        .post(url)
        .json(&venue_creation)
        .send()
        .await?;
    response.json().await
}

pub async fn fetch_venue(id: &str) -> Result<VenueDTO, reqwest::Error> {
    let url = Url::parse(&format!("http://localhost:3001/venue/{}", id));
    let response = reqwest::Client::new().get(url.unwrap()).send().await?;
    response.json().await
}

pub async fn fetch_teams() -> Vec<TeamDTO> {
    let url = Url::parse("http://localhost:3001/teams");
    let response = reqwest::Client::new().get(url.unwrap()).send().await;
    response.unwrap().json().await.unwrap()
}

pub async fn fetch_team(id: &str) -> Result<TeamDTO, reqwest::Error> {
    let url = Url::parse(&format!("http://localhost:3001/team/{}", id));
    let response = reqwest::Client::new().get(url.unwrap()).send().await?;
    response.json().await
}

pub async fn create_team(team_creation: &TeamCreationDTO) -> Result<TeamDTO, reqwest::Error> {
    let url = Url::parse("http://localhost:3001/team").unwrap();
    let response = reqwest::Client::new()
        .post(url)
        .json(&team_creation)
        .send()
        .await?;
    response.json().await
}

pub async fn fetch_fixtures() -> Vec<FixtureDTO> {
    let url = Url::parse("http://localhost:3001/fixtures");
    let response = reqwest::Client::new().get(url.unwrap()).send().await;
    response.unwrap().json().await.unwrap()
}

pub async fn create_fixture(
    fixture_creation: &FixtureCreationDTO,
) -> Result<FixtureDTO, reqwest::Error> {
    let url = Url::parse("http://localhost:3001/fixture").unwrap();
    let response = reqwest::Client::new()
        .post(url)
        .json(&fixture_creation)
        .send()
        .await?;
    response.json().await
}

pub async fn fetch_fixture(id: &str) -> Result<FixtureDTO, reqwest::Error> {
    let url = Url::parse(&format!("http://localhost:3001/fixture/{}", id));
    let response = reqwest::Client::new().get(url.unwrap()).send().await?;
    response.json().await
}

pub async fn change_fixture_date(
    fixture_id: &str,
    date: DateTime<Utc>,
) -> Result<FixtureDTO, reqwest::Error> {
    let url = Url::parse(&format!(
        "http://localhost:3001/fixture/{}/date",
        fixture_id
    ));
    let response = reqwest::Client::new()
        .post(url.unwrap())
        .json(&date)
        .send()
        .await?;
    response.json().await
}

pub async fn change_fixture_venue(
    fixture_id: &str,
    venue_id: &str,
) -> Result<FixtureDTO, reqwest::Error> {
    let url = Url::parse(&format!(
        "http://localhost:3001/fixture/{}/venue",
        fixture_id
    ));
    let response = reqwest::Client::new()
        .post(url.unwrap())
        .json(&venue_id)
        .send()
        .await?;
    response.json().await
}

pub async fn cancel_fixture(fixture_id: &str) -> Result<FixtureDTO, reqwest::Error> {
    let url = Url::parse(&format!(
        "http://localhost:3001/fixture/{}/cancel",
        fixture_id
    ));
    let response = reqwest::Client::new().post(url.unwrap()).send().await?;
    response.json().await
}
