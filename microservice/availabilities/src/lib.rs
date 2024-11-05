use std::sync::Arc;

use microservices_shared::domain_events::DomainEventPublisher;
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
    pub domain_event_publisher: Box<dyn DomainEventPublisher + Send + Sync>,
    pub tracer: Arc<BoxedTracer>,
}
