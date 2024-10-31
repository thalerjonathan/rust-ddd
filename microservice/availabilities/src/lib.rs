use microservices_shared::domain_events::DomainEventPublisher;
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
}
