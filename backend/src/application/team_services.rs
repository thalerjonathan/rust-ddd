use log::debug;

use crate::domain::{aggregates::team::Team, repositories::team_repo::TeamRepository};

pub async fn create_team(
    name: &str,
    club: &str,
    repo: &impl TeamRepository<Error = String>,
) -> Result<Team, String> {
    let team = Team::new(name, club);

    repo.save(&team).await.map_err(|e| e.to_string())?;

    debug!("Team created: {:?}", team);

    Ok(team)
}
