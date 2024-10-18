use shared::{AssignmentDTO, AssignmentStagingDTO};

use crate::domain::{aggregates::{assignment::{Assignment, AssignmentRefereeRole}, fixture::FixtureId, referee::RefereeId}, repositories::{assignment_repo::AssignmentRepository, fixture_repo::FixtureRepository, referee_repo::RefereeRepository}};

pub async fn remove_staged_assignment<TxCtx>(
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

pub async fn remove_committed_assignment<TxCtx>(
    fixture_id: FixtureId,
    referee_id: RefereeId,
    assignment_repo: &impl AssignmentRepository<TxCtx = TxCtx, Error = String>,
    fixture_repo: &impl FixtureRepository<TxCtx = TxCtx, Error = String>,
    tx_ctx: &mut TxCtx,
) -> Result<(), String> {
    let assignment = assignment_repo.find_by_fixture_and_referee(fixture_id, referee_id, tx_ctx).await?;

    if assignment.is_none() {
        return Err(format!("Assignment with fixture_id {} and referee_id {} not found", fixture_id.0, referee_id.0));
    }

    let assignment = assignment.unwrap();

    if assignment.is_staged() {
        return Err(format!("Assignment with fixture_id {} and referee_id {} not committed", fixture_id.0, referee_id.0));
    }

    let mut fixture = fixture_repo.find_by_id(assignment.fixture_id(), tx_ctx).await?.expect(&format!("Fixture {} not found when removing committed assignment", assignment.fixture_id().0));

    match assignment.referee_role() {
        AssignmentRefereeRole::First => {
            if fixture.first_referee().is_none() {
                return Err(format!("First referee not assigned for fixture {}", fixture.id().0));
            }

            if fixture.first_referee().unwrap().id() != assignment.referee_id() {
                return Err(format!("First referee not assigned for fixture {}", fixture.id().0));
            }

            fixture.unassign_first_referee();
        },
        AssignmentRefereeRole::Second => {
            if fixture.second_referee().is_none() {
                return Err(format!("Second referee not assigned for fixture {}", fixture.id().0));
            }

            if fixture.second_referee().unwrap().id() != assignment.referee_id() {
                return Err(format!("Second referee not assigned for fixture {}", fixture.id().0));
            }

            fixture.unassign_second_referee()
        },
    }

    fixture_repo.save(&fixture, tx_ctx).await?;
    assignment_repo.delete(&assignment, tx_ctx).await
}

pub async fn stage_assignment<TxCtx>(
    assignment_staging: &AssignmentStagingDTO,
    assignment_repo: &impl AssignmentRepository<TxCtx = TxCtx, Error = String>,
    fixture_repo: &impl FixtureRepository<TxCtx = TxCtx, Error = String>,
    referee_repo: &impl RefereeRepository<TxCtx = TxCtx, Error = String>,
    tx_ctx: &mut TxCtx,
) -> Result<AssignmentDTO, String> {

    let fixture = fixture_repo.find_by_id(assignment_staging.fixture_id.0.into(), tx_ctx).await?.expect(&format!("Fixture {} not found when staging assignment", assignment_staging.fixture_id.0));
    let referee = referee_repo.find_by_id(assignment_staging.referee_id.0.into(), tx_ctx).await?.expect(&format!("Referee {} not found when staging assignment", assignment_staging.referee_id.0));

    let assignment_lookup = assignment_repo.find_by_fixture_and_referee(fixture.id().0.into(), referee.id().0.into(), tx_ctx).await?;

    let mut assignment = match assignment_lookup {
        // NOTE: if assignment already exists, we simply override it as staged
        Some(a) => Assignment::staged(a.fixture_id(), a.referee_id(), a.referee_role()),
        None => Assignment::staged(assignment_staging.fixture_id.0.into(), assignment_staging.referee_id.0.into(), assignment_staging.referee_role.into()), // otherwise we create a new one
    };

    // always overwrite the referee role
    assignment.change_referee_role(assignment_staging.referee_role.into());
    // save does an upsert, potentially updating the referee role if it has changed
    assignment_repo.save(&assignment, tx_ctx).await?;

    Ok(assignment.into())
}

pub async fn commit_assignments<TxCtx>(
    assignment_repo: &impl AssignmentRepository<TxCtx = TxCtx, Error = String>,
    fixture_repo: &impl FixtureRepository<TxCtx = TxCtx, Error = String>,
    referee_repo: &impl RefereeRepository<TxCtx = TxCtx, Error = String>,
    tx_ctx: &mut TxCtx,
) -> Result<String, String> {
    // NOTE: committing assignments also validates them and rejects if any invalid
    validate_assignments(assignment_repo, tx_ctx).await?;

    // NOTE: committing assignments fetches all staged assignments and assigns the referees to the fixtures according to their roles 
    // and changes the assignment status to committed

    let assignments = assignment_repo.find_all_staged(tx_ctx).await?;

    for mut assignment in assignments {
        let mut fixture = fixture_repo.find_by_id(assignment.fixture_id(), tx_ctx).await?.expect(&format!("Fixture {} not found when committing assignments", assignment.fixture_id().0));
        let referee = referee_repo.find_by_id(assignment.referee_id(), tx_ctx).await?.expect(&format!("Referee {} not found when committing assignments", assignment.referee_id().0));
        let role = assignment.referee_role();

        match role {
            AssignmentRefereeRole::First => fixture.assign_first_referee(referee),
            AssignmentRefereeRole::Second => fixture.assign_second_referee(referee),
        }

        assignment.commit();

        fixture_repo.save(&fixture, tx_ctx).await?;
        assignment_repo.save(&assignment, tx_ctx).await?;
    }

    Ok("Assignments committed".to_string())
}

pub async fn validate_assignments<TxCtx>(
    _assignment_repo: &impl AssignmentRepository<TxCtx = TxCtx, Error = String>,
    _tx_ctx: &mut TxCtx,
) -> Result<String, String> {
    // NOTE: this is not fully implemented, just a placeholder because its
    // complex domain logic, requiring additional domain functions in the 
    // aggregates and the domain service layer.
    // An assignment is valid if:
    //  - the fixture is in a playable state (i.e. not postponed, cancelled, etc.)
    //  - the referee is available for the fixture
    //  - the referee is not already assigned to another fixture at the same time
    //  - the referee is not assigned to a fixture that is "close" to the fixture (i.e. same field, same game-hour), except when its at the same venue

    Ok("Assignments validated".to_string())
}
