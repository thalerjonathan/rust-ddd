use std::sync::Arc;

use domain::aggregates::assignment::{Assignment, AssignmentRefereeRole, AssignmentStatus};
use opentelemetry::global::BoxedTracer;
use restinterface::{AssignmentDTO, AssignmentRefereeRoleDTO, AssignmentStatusDTO};
use sqlx::PgPool;

pub mod adapters;
pub mod application;
pub mod config;
pub mod domain;
pub mod ports;

pub struct AppState {
    pub connection_pool: PgPool,
    pub redis_client: redis::Client,
    pub tracer: Arc<BoxedTracer>,
}

// NOTE: put here because REST and Application layers need them

impl From<AssignmentRefereeRole> for AssignmentRefereeRoleDTO {
    fn from(role: AssignmentRefereeRole) -> Self {
        match role {
            AssignmentRefereeRole::First => AssignmentRefereeRoleDTO::First,
            AssignmentRefereeRole::Second => AssignmentRefereeRoleDTO::Second,
        }
    }
}

impl From<AssignmentRefereeRoleDTO> for AssignmentRefereeRole {
    fn from(role: AssignmentRefereeRoleDTO) -> Self {
        match role {
            AssignmentRefereeRoleDTO::First => AssignmentRefereeRole::First,
            AssignmentRefereeRoleDTO::Second => AssignmentRefereeRole::Second,
        }
    }
}

impl From<AssignmentStatus> for AssignmentStatusDTO {
    fn from(status: AssignmentStatus) -> Self {
        match status {
            AssignmentStatus::Committed => AssignmentStatusDTO::Committed,
            AssignmentStatus::Staged => AssignmentStatusDTO::Staged,
        }
    }
}

impl From<Assignment> for AssignmentDTO {
    fn from(assignment: Assignment) -> Self {
        AssignmentDTO {
            status: assignment.status().into(),
            fixture_id: assignment.fixture_id().into(),
            referee_id: assignment.referee_id().into(),
            referee_role: assignment.referee_role().into(),
        }
    }
}
