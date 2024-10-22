use sqlx::PgPool;

pub mod adapters;
pub mod application;
pub mod domain;
pub mod ports;

pub struct AppState {
    pub connection_pool: PgPool,
}
