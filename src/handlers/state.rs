use sqlx::PgPool;

pub struct AppState {
    pub connection_pool: PgPool,
}