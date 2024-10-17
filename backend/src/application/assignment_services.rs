use shared::{AssignmentDTO, AssignmentStagingDTO};

use crate::domain::{aggregates::{fixture::FixtureId, referee::RefereeId}, repositories::assignment_repo::AssignmentRepository};

pub async fn delete_staged_assignment<TxCtx>(
    fixture_id: FixtureId,
    referee_id: RefereeId,
    assignment_repo: &impl AssignmentRepository<TxCtx = TxCtx, Error = String>,
    tx_ctx: &mut TxCtx,
) -> Result<(), String> {
    let assignment = assignment_repo.find_by_fixture_and_referee(fixture_id, referee_id, tx_ctx).await?;

    if assignment.is_none() {
        return Err(format!("Assignment with fixture_id {} and referee_id {} not found", fixture_id.0, referee_id.0));
    }

    let assignment = assignment.unwrap();

    if assignment.is_committed() {
        return Err(format!("Assignment with fixture_id {} and referee_id {} not staged", fixture_id.0, referee_id.0));
    }

    assignment_repo.delete(&assignment, tx_ctx).await
}

pub async fn stage_assignment<TxCtx>(
    _assignment_staging: AssignmentStagingDTO,
    _assignment_repo: &impl AssignmentRepository<TxCtx = TxCtx, Error = String>,
    _tx_ctx: &mut TxCtx,
) -> Result<AssignmentDTO, String> {
    // TODO: implement

    // TODO: fetch from DB and see if there exists already one

    Err("Assignment staging not implemented".to_string())
}

pub async fn commit_assignments<TxCtx>(
    _assignment_repo: &impl AssignmentRepository<TxCtx = TxCtx, Error = String>,
    _tx_ctx: &mut TxCtx,
) -> Result<String, String> {
    // TODO: implement

    Ok("Assignments committed".to_string())
}

pub async fn validate_assignments<TxCtx>(
    _assignment_repo: &impl AssignmentRepository<TxCtx = TxCtx, Error = String>,
    _tx_ctx: &mut TxCtx,
) -> Result<String, String> {
    // TODO: implement

    Ok("Assignments validated".to_string())
}
