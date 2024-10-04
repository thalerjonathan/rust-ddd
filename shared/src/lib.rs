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

pub async fn fetch_referees() -> Vec<RefereeDTO> {
    let url = Url::parse("http://localhost:3001/referees");
    let response = reqwest::Client::new().get(url.unwrap()).send().await;
    response.unwrap().json().await.unwrap()
}

pub async fn create_referee(
    ref_creation: RefereeCreationDTO,
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
