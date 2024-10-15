use crate::domain::{aggregates::{fixture::FixtureId, referee::RefereeId}, repositories::{availability_repo::AvailabilityRepository, fixture_repo::FixtureRepository, referee_repo::RefereeRepository}};

pub async fn declare_availability<TxCtx>(
    fixture_id: FixtureId,
    referee_id: RefereeId,
    fixture_repo: &impl FixtureRepository<TxCtx = TxCtx, Error = String>,
    referee_repo: &impl RefereeRepository<TxCtx = TxCtx, Error = String>,
    availability_repo: &impl AvailabilityRepository<TxCtx = TxCtx, Error = String>,
    tx_ctx: &mut TxCtx,
) -> Result<(), String> {

    let fixture = fixture_repo.find_by_id(fixture_id, tx_ctx).await?.expect("Fixture not found when declaring availability");
    let referee = referee_repo.find_by_id(referee_id, tx_ctx).await?.expect("Referee not found when declaring availability");

    if availability_repo.is_available(&fixture, &referee, tx_ctx).await? {
        return Err("Referee is already available for this fixture - cannot declare availability".to_string());
    }

    availability_repo.declare_availability(&fixture, &referee, tx_ctx).await?;

    Ok(())
}

pub async fn withdraw_availability<TxCtx>(
    fixture_id: FixtureId,
    referee_id: RefereeId,
    fixture_repo: &impl FixtureRepository<TxCtx = TxCtx, Error = String>,
    referee_repo: &impl RefereeRepository<TxCtx = TxCtx, Error = String>,
    availability_repo: &impl AvailabilityRepository<TxCtx = TxCtx, Error = String>,
    tx_ctx: &mut TxCtx,
) -> Result<(), String> {
    let fixture = fixture_repo.find_by_id(fixture_id, tx_ctx).await?.expect("Fixture not found when withdrawing availability");
    let referee = referee_repo.find_by_id(referee_id, tx_ctx).await?.expect("Referee not found when withdrawing availability");

    if ! availability_repo.is_available(&fixture, &referee, tx_ctx).await? {
        return Err("Referee is not available for this fixture - cannot withdraw availability".to_string());
    }

    availability_repo.withdraw_availability(&fixture, &referee, tx_ctx).await?;

    Ok(())
}

pub async fn get_availabilities_for_referee<TxCtx>(
    referee_id: RefereeId,
    referee_repo: &impl RefereeRepository<TxCtx = TxCtx, Error = String>,
    availability_repo: &impl AvailabilityRepository<TxCtx = TxCtx, Error = String>,
    tx_ctx: &mut TxCtx,
) -> Result<Vec<FixtureId>, String> {
    let referee = referee_repo.find_by_id(referee_id, tx_ctx).await?.expect("Referee not found");

    let availabilities = availability_repo.get_availabilities_for_referee(&referee, tx_ctx).await?;

    Ok(availabilities)
}
