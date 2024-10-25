use domain::aggregates::fixture::FixtureStatus;
use restinterface::FixtureStatusDTO;
use sqlx::PgPool;

pub mod adapters;
pub mod application;
pub mod domain;
pub mod ports;

pub struct AppState {
    pub connection_pool: PgPool,
    pub redis_client: redis::Client,
}

// NOTE: This is a temporary solution to convert the FixtureStatus enum to the FixtureStatusDTO enum.
// This is because the FixtureStatusDTO enum is not defined in the same crate as the FixtureStatus enum.
impl From<FixtureStatus> for FixtureStatusDTO {
    fn from(status: FixtureStatus) -> Self {
        match status {
            FixtureStatus::Scheduled => FixtureStatusDTO::Scheduled,
            FixtureStatus::Cancelled => FixtureStatusDTO::Cancelled,
        }
    }
}
