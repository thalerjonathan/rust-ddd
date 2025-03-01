use std::sync::Arc;

use opentelemetry::global::BoxedTracer;
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
