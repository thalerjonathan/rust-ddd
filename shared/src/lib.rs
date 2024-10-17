use chrono::{DateTime, Utc};
use reqwest::Url;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub struct RefereeIdDTO(pub Uuid);

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub struct FixtureIdDTO(pub Uuid);

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub struct VenueIdDTO(pub Uuid);

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub struct TeamIdDTO(pub Uuid);

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct RefereeDTO {
    pub id: RefereeIdDTO,
    pub name: String,
    pub club: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct RefereeCreationDTO {
    pub name: String,
    pub club: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct VenueDTO {
    pub id: VenueIdDTO,
    pub name: String,
    pub street: String,
    pub zip: String,
    pub city: String,
    pub telephone: Option<String>,
    pub email: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct VenueCreationDTO {
    pub name: String,
    pub street: String,
    pub zip: String,
    pub city: String,
    pub telephone: Option<String>,
    pub email: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct TeamDTO {
    pub id: TeamIdDTO,
    pub name: String,
    pub club: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct TeamCreationDTO {
    pub name: String,
    pub club: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum FixtureStatusDTO {
    Scheduled,
    Cancelled,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct FixtureDTO {
    pub id: FixtureIdDTO,
    pub team_home: TeamDTO,
    pub team_away: TeamDTO,
    pub venue: VenueDTO,
    pub date: DateTime<Utc>,
    pub status: FixtureStatusDTO,
    pub first_referee: Option<RefereeDTO>,
    pub second_referee: Option<RefereeDTO>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct FixtureCreationDTO {
    pub team_home_id: TeamIdDTO,
    pub team_away_id: TeamIdDTO,
    pub venue_id: VenueIdDTO,
    pub date: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssignmentStatusDTO {
    Committed,
    Staged,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssignmentRefereeRoleDTO {
    First,
    Second,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct AssignmentDTO {
    pub status: AssignmentStatusDTO,
    pub referee_role: AssignmentRefereeRoleDTO,
    pub fixture_id: FixtureIdDTO,
    pub referee_id: RefereeIdDTO,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct AssignmentStagingDTO {
    pub fixture_id: FixtureIdDTO,
    pub referee_id: RefereeIdDTO,
    pub referee_role: AssignmentRefereeRoleDTO,
}

impl From<String> for RefereeIdDTO {
    fn from(value: String) -> Self {
        RefereeIdDTO(Uuid::parse_str(&value).unwrap())
    }
}

impl From<String> for FixtureIdDTO {
    fn from(value: String) -> Self {
        FixtureIdDTO(Uuid::parse_str(&value).unwrap())
    }
}

impl From<String> for VenueIdDTO {
    fn from(value: String) -> Self {
        VenueIdDTO(Uuid::parse_str(&value).unwrap())
    }
}

impl From<String> for TeamIdDTO {
    fn from(value: String) -> Self {
        TeamIdDTO(Uuid::parse_str(&value).unwrap())
    }
}

impl From<Uuid> for RefereeIdDTO {
    fn from(value: Uuid) -> Self {
        RefereeIdDTO(value)
    }
}

impl From<Uuid> for FixtureIdDTO {
    fn from(value: Uuid) -> Self {
        FixtureIdDTO(value)
    }
}

impl From<Uuid> for VenueIdDTO {
    fn from(value: Uuid) -> Self {
        VenueIdDTO(value)
    }
}

impl From<Uuid> for TeamIdDTO {
    fn from(value: Uuid) -> Self {
        TeamIdDTO(value)
    }
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

pub async fn fetch_referee(referee_id: RefereeIdDTO) -> RefereeDTO {
    let url = Url::parse(&format!("http://localhost:3001/referee/{}", referee_id.0));
    let response = reqwest::Client::new().get(url.unwrap()).send().await;
    response.unwrap().json().await.unwrap()
}

pub async fn change_referee_club(
    referee_id: RefereeIdDTO,
    club: &str,
) -> Result<String, reqwest::Error> {
    let url = Url::parse(&format!(
        "http://localhost:3001/referee/{}/club",
        referee_id.0
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

pub async fn fetch_venue(venue_id: VenueIdDTO) -> Result<VenueDTO, reqwest::Error> {
    let url = Url::parse(&format!("http://localhost:3001/venue/{}", venue_id.0));
    let response = reqwest::Client::new().get(url.unwrap()).send().await?;
    response.json().await
}

pub async fn fetch_teams() -> Vec<TeamDTO> {
    let url = Url::parse("http://localhost:3001/teams");
    let response = reqwest::Client::new().get(url.unwrap()).send().await;
    response.unwrap().json().await.unwrap()
}

pub async fn fetch_team(team_id: TeamIdDTO) -> Result<TeamDTO, reqwest::Error> {
    let url = Url::parse(&format!("http://localhost:3001/team/{}", team_id.0));
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

pub async fn fetch_fixture(fixture_id: FixtureIdDTO) -> Result<FixtureDTO, reqwest::Error> {
    let url = Url::parse(&format!("http://localhost:3001/fixture/{}", fixture_id.0));
    let response = reqwest::Client::new().get(url.unwrap()).send().await?;
    response.json().await
}

pub async fn change_fixture_date(
    fixture_id: FixtureIdDTO,
    date: DateTime<Utc>,
) -> Result<FixtureDTO, reqwest::Error> {
    let url = Url::parse(&format!(
        "http://localhost:3001/fixture/{}/date",
        fixture_id.0
    ));
    let response = reqwest::Client::new()
        .post(url.unwrap())
        .json(&date)
        .send()
        .await?;
    response.json().await
}

pub async fn change_fixture_venue(
    fixture_id: FixtureIdDTO,
    venue_id: VenueIdDTO,
) -> Result<FixtureDTO, reqwest::Error> {
    let url = Url::parse(&format!(
        "http://localhost:3001/fixture/{}/venue",
        fixture_id.0
    ));
    let response = reqwest::Client::new()
        .post(url.unwrap())
        .json(&venue_id)
        .send()
        .await?;
    response.json().await
}

pub async fn cancel_fixture(fixture_id: FixtureIdDTO) -> Result<FixtureDTO, reqwest::Error> {
    let url = Url::parse(&format!(
        "http://localhost:3001/fixture/{}/cancel",
        fixture_id.0
    ));
    let response = reqwest::Client::new().post(url.unwrap()).send().await?;
    response.json().await
}

pub async fn fetch_availabilities_for_referee(
    referee_id: RefereeIdDTO,
) -> Result<Vec<FixtureIdDTO>, reqwest::Error> {
    let url = Url::parse(&format!(
        "http://localhost:3001/availabilities/referee/{}",
        referee_id.0
    ));
    let response = reqwest::Client::new().get(url.unwrap()).send().await?;
    response.json().await
}

pub async fn declare_availability(
    fixture_id: FixtureIdDTO,
    referee_id: RefereeIdDTO,
) -> Result<(), reqwest::Error> {
    let url = Url::parse(&format!(
        "http://localhost:3001/availabilities/declare/fixture/{}/referee/{}",
        fixture_id.0, referee_id.0
    ));
    let response = reqwest::Client::new().post(url.unwrap()).send().await?;
    response.json().await
}

pub async fn withdraw_availability(
    fixture_id: FixtureIdDTO,
    referee_id: RefereeIdDTO,
) -> Result<(), reqwest::Error> {
    let url = Url::parse(&format!(
        "http://localhost:3001/availabilities/withdraw/fixture/{}/referee/{}",
        fixture_id.0, referee_id.0
    ));
    let response = reqwest::Client::new().post(url.unwrap()).send().await?;
    response.json().await
}

pub async fn fetch_assignments() -> Vec<AssignmentDTO> {
    let url = Url::parse("http://localhost:3001/assignments");
    let response = reqwest::Client::new().get(url.unwrap()).send().await;
    response.unwrap().json().await.unwrap()
}

pub async fn stage_assignment(
    assignment_staging: &AssignmentStagingDTO,
) -> Result<AssignmentDTO, reqwest::Error> {
    let url = Url::parse("http://localhost:3001/assignments").unwrap();
    let response = reqwest::Client::new()
        .put(url)
        .json(&assignment_staging)
        .send()
        .await?;
    response.json().await
}

pub async fn delete_staged_assignment(
    assignment: &AssignmentDTO,
) -> Result<(), reqwest::Error> {
    let url = Url::parse(&format!(
        "http://localhost:3001/assignments/{}/{}",
        assignment.fixture_id.0, assignment.referee_id.0
    )).unwrap();
    let response = reqwest::Client::new()
        .delete(url)
        .send()
        .await?;
    response.json().await
}
pub async fn validate_assignments() -> Result<String, reqwest::Error> {
    let url = Url::parse("http://localhost:3001/assignments/validate");
    let response = reqwest::Client::new().post(url.unwrap()).send().await?;
    response.text().await
}

pub async fn commit_assignments() -> Result<String, reqwest::Error> {
    let url = Url::parse("http://localhost:3001/assignments/commit");
    let response = reqwest::Client::new().post(url.unwrap()).send().await?;
    response.text().await
}
