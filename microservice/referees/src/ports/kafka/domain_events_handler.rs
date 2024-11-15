use std::sync::Arc;

use axum::async_trait;
use log::info;
use microservices_shared::{
    domain_events::{DomainEventCallbacks, DomainEventCallbacksLoggerImpl},
    domain_ids::{FixtureId, RefereeId, TeamId, VenueId},
};
use opentelemetry::{
    trace::{Span, Tracer},
    KeyValue,
};
use redis::Commands;
use sqlx::types::chrono::{DateTime, Utc};
pub struct DomainEventCallbacksImpl {
    redis_conn: redis::Connection,
    tracer: Arc<opentelemetry::global::BoxedTracer>,
    delegate: DomainEventCallbacksLoggerImpl,
}

impl DomainEventCallbacksImpl {
    pub fn new(
        redis_conn: redis::Connection,
        tracer: Arc<opentelemetry::global::BoxedTracer>,
    ) -> Self {
        Self {
            redis_conn,
            tracer: tracer.clone(),
            delegate: DomainEventCallbacksLoggerImpl::new(tracer),
        }
    }
}

#[async_trait]
impl DomainEventCallbacks for DomainEventCallbacksImpl {
    type TxCtx = sqlx::Transaction<'static, sqlx::Postgres>;
    type Error = String;

    async fn on_referee_created(
        &mut self,
        referee_id: RefereeId,
        _tx_ctx: &mut Self::TxCtx,
    ) -> Result<(), String> {
        self.delegate.on_referee_created(referee_id, _tx_ctx).await
    }

    async fn on_referee_club_changed(
        &mut self,
        referee_id: RefereeId,
        club_name: String,
        _tx_ctx: &mut Self::TxCtx,
    ) -> Result<(), String> {
        info!(
            "Received Domain Event: Referee club changed: {:?} -> {}",
            referee_id, club_name
        );
        let mut span = self.tracer.start("on_referee_club_changed");
        span.set_attribute(KeyValue::new("referee_id", referee_id.0.to_string()));
        span.set_attribute(KeyValue::new("club_name", club_name));

        info!("Invalidating cache entry for referee: {:?}", referee_id);

        // NOTE: invalidate the cache entry for the referee
        let key = format!("referee_{}", referee_id.0.to_string());
        let _result: Result<(), redis::RedisError> = self.redis_conn.del(key);

        _result.map_err(|e| e.to_string())
    }

    async fn on_team_created(
        &mut self,
        team_id: TeamId,
        _tx_ctx: &mut Self::TxCtx,
    ) -> Result<(), String> {
        self.delegate.on_team_created(team_id, _tx_ctx).await
    }

    async fn on_venue_created(
        &mut self,
        venue_id: VenueId,
        _tx_ctx: &mut Self::TxCtx,
    ) -> Result<(), String> {
        self.delegate.on_venue_created(venue_id, _tx_ctx).await
    }

    async fn on_fixture_created(
        &mut self,
        fixture_id: FixtureId,
        _tx_ctx: &mut Self::TxCtx,
    ) -> Result<(), String> {
        self.delegate.on_fixture_created(fixture_id, _tx_ctx).await
    }

    async fn on_fixture_date_changed(
        &mut self,
        fixture_id: FixtureId,
        date: DateTime<Utc>,
        _tx_ctx: &mut Self::TxCtx,
    ) -> Result<(), String> {
        self.delegate
            .on_fixture_date_changed(fixture_id, date, _tx_ctx)
            .await
    }

    async fn on_fixture_venue_changed(
        &mut self,
        fixture_id: FixtureId,
        venue_id: VenueId,
        _tx_ctx: &mut Self::TxCtx,
    ) -> Result<(), String> {
        self.delegate
            .on_fixture_venue_changed(fixture_id, venue_id, _tx_ctx)
            .await
    }

    async fn on_fixture_cancelled(
        &mut self,
        fixture_id: FixtureId,
        _tx_ctx: &mut Self::TxCtx,
    ) -> Result<(), String> {
        self.delegate
            .on_fixture_cancelled(fixture_id, _tx_ctx)
            .await
    }

    async fn on_availability_declared(
        &mut self,
        fixture_id: FixtureId,
        referee_id: RefereeId,
        _tx_ctx: &mut Self::TxCtx,
    ) -> Result<(), String> {
        self.delegate
            .on_availability_declared(fixture_id, referee_id, _tx_ctx)
            .await
    }

    async fn on_availability_withdrawn(
        &mut self,
        fixture_id: FixtureId,
        referee_id: RefereeId,
        _tx_ctx: &mut Self::TxCtx,
    ) -> Result<(), String> {
        self.delegate
            .on_availability_withdrawn(fixture_id, referee_id, _tx_ctx)
            .await
    }

    async fn on_first_referee_assigned(
        &mut self,
        fixture_id: FixtureId,
        referee_id: RefereeId,
        _tx_ctx: &mut Self::TxCtx,
    ) -> Result<(), String> {
        self.delegate
            .on_first_referee_assigned(fixture_id, referee_id, _tx_ctx)
            .await
    }

    async fn on_first_referee_assignment_removed(
        &mut self,
        fixture_id: FixtureId,
        referee_id: RefereeId,
        _tx_ctx: &mut Self::TxCtx,
    ) -> Result<(), String> {
        self.delegate
            .on_first_referee_assignment_removed(fixture_id, referee_id, _tx_ctx)
            .await
    }

    async fn on_second_referee_assigned(
        &mut self,
        fixture_id: FixtureId,
        referee_id: RefereeId,
        _tx_ctx: &mut Self::TxCtx,
    ) -> Result<(), String> {
        self.delegate
            .on_second_referee_assigned(fixture_id, referee_id, _tx_ctx)
            .await
    }

    async fn on_second_referee_assignment_removed(
        &mut self,
        fixture_id: FixtureId,
        referee_id: RefereeId,
        _tx_ctx: &mut Self::TxCtx,
    ) -> Result<(), String> {
        self.delegate
            .on_second_referee_assignment_removed(fixture_id, referee_id, _tx_ctx)
            .await
    }
}
