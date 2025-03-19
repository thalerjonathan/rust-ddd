use std::sync::Arc;

use microservices_shared::token::TokenManager;
use opentelemetry::global::BoxedTracer;
use tokio::sync::Mutex;

pub mod config;
pub mod handlers;

pub struct AppState {
    pub redis_conn: Mutex<redis::Connection>,
    pub token_manager: TokenManager,
    pub tracer: Arc<BoxedTracer>,
}
