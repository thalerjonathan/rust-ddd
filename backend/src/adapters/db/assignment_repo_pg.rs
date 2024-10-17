use sqlx::{Postgres, Transaction};
use uuid::Uuid;

use crate::domain::{aggregates::{assignment::{Assignment, AssignmentRefereeRole, AssignmentStatus}, fixture::FixtureId, referee::RefereeId}, repositories::assignment_repo::AssignmentRepository};

#[derive(sqlx::Type, Debug)]
#[sqlx(type_name = "rustddd.assignment_referee_role", rename_all = "lowercase")]
enum AssignmentRefereeRoleDb {
    First,
    Second,
}

#[derive(sqlx::Type, Debug)]
#[sqlx(type_name = "rustddd.assignment_status", rename_all = "lowercase")]
enum AssignmentStatusDb {
    Committed,
    Staged,
}

#[derive(sqlx::FromRow, Debug)]
struct AssignmentDb {
    pub status: AssignmentStatusDb,
    pub fixture_id: Uuid,
    pub referee_id: Uuid,
    pub referee_role: AssignmentRefereeRoleDb,
}

pub struct AssignmentRepositoryPg ();


impl AssignmentRepositoryPg {
    pub fn new() -> Self {
        Self { }
    }
}

impl From<AssignmentStatusDb> for AssignmentStatus {
    fn from(status: AssignmentStatusDb) -> Self {
        match status {
            AssignmentStatusDb::Committed => AssignmentStatus::Committed,
            AssignmentStatusDb::Staged => AssignmentStatus::Staged,
        }
    }
}

impl From<AssignmentStatus> for AssignmentStatusDb {
    fn from(status: AssignmentStatus) -> Self {
        match status {
            AssignmentStatus::Committed => AssignmentStatusDb::Committed,
            AssignmentStatus::Staged => AssignmentStatusDb::Staged,
        }
    }
}

impl From<AssignmentRefereeRoleDb> for AssignmentRefereeRole {
    fn from(role: AssignmentRefereeRoleDb) -> Self {
        match role {
            AssignmentRefereeRoleDb::First => AssignmentRefereeRole::First,
            AssignmentRefereeRoleDb::Second => AssignmentRefereeRole::Second,
        }
    }
}

impl From<AssignmentRefereeRole> for AssignmentRefereeRoleDb {
    fn from(role: AssignmentRefereeRole) -> Self {
        match role {
            AssignmentRefereeRole::First => AssignmentRefereeRoleDb::First,
            AssignmentRefereeRole::Second => AssignmentRefereeRoleDb::Second,
        }
    }
}

impl From<AssignmentDb> for Assignment {
    fn from(assignment: AssignmentDb) -> Self {
        Assignment::new(
            assignment.fixture_id.into(),
            assignment.referee_id.into(),
            assignment.referee_role.into(),
            assignment.status.into()
        )
    }
}

impl AssignmentRepository for AssignmentRepositoryPg {
    type Error = String;
    type TxCtx = Transaction<'static, Postgres>;

    async fn get_all(&self, tx_ctx: &mut Self::TxCtx) -> Result<Vec<Assignment>, Self::Error> {
        let assignments = sqlx::query_as!(
            AssignmentDb,
            "SELECT status as \"status: AssignmentStatusDb\", fixture_id, referee_id, referee_role as \"referee_role: AssignmentRefereeRoleDb\" 
            FROM rustddd.assignments"
        )
        .fetch_all(&mut **tx_ctx)
        .await
        .map_err(|e| e.to_string())?;   

        Ok(assignments.into_iter().map(|a| a.into()).collect())
    }

    async fn find_all_staged(&self, tx_ctx: &mut Self::TxCtx) -> Result<Vec<Assignment>, Self::Error> {
        let assignments = sqlx::query_as!(
            AssignmentDb,
            "SELECT status as \"status: AssignmentStatusDb\", fixture_id, referee_id, referee_role as \"referee_role: AssignmentRefereeRoleDb\" 
            FROM rustddd.assignments WHERE status = 'staged'"
        )
        .fetch_all(&mut **tx_ctx)
        .await
        .map_err(|e| e.to_string())?;

        Ok(assignments.into_iter().map(|a| a.into()).collect())
    }

    async fn find_by_fixture_and_referee(&self, fixture_id: FixtureId, referee_id: RefereeId, tx_ctx: &mut Self::TxCtx) -> Result<Option<Assignment>, Self::Error> {
        let assignment = sqlx::query_as!(
            AssignmentDb,
            "SELECT status as \"status: AssignmentStatusDb\", fixture_id, referee_id, referee_role as \"referee_role: AssignmentRefereeRoleDb\" 
            FROM rustddd.assignments WHERE fixture_id = $1 AND referee_id = $2",
            fixture_id.0,
            referee_id.0
        )
        .fetch_optional(&mut **tx_ctx)
        .await
        .map_err(|e| e.to_string())?;

        Ok(assignment.map(Assignment::from))
    }

    async fn delete(&self, assignment: &Assignment, tx_ctx: &mut Self::TxCtx) -> Result<(), Self::Error> {
        sqlx::query!(
            "DELETE FROM rustddd.assignments WHERE fixture_id = $1 AND referee_id = $2",
            assignment.fixture_id().0,
            assignment.referee_id().0
        )
        .execute(&mut **tx_ctx)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }   

    async fn save(&self, assignment: &Assignment, tx_ctx: &mut Self::TxCtx) -> Result<(), Self::Error> {
        let referee_role: AssignmentRefereeRoleDb = assignment.referee_role().into();
        let status: AssignmentStatusDb = assignment.status().into();
        sqlx::query!(
            "INSERT INTO rustddd.assignments (status, fixture_id, referee_id, referee_role) 
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (fixture_id, referee_id) 
            DO UPDATE SET referee_role = $4, status = $1",
            status as AssignmentStatusDb,
            assignment.fixture_id().0,
            assignment.referee_id().0,
            referee_role as AssignmentRefereeRoleDb
            )
        .execute(&mut **tx_ctx)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }
}
