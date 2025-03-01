use std::sync::Arc;

use crate::{
    adapters::db::fixture_repo_pg::FixtureRepositoryPg,
    application::fixture_services::{
        assign_first_referee, assign_second_referee, unassign_first_referee,
        unassign_second_referee,
    },
};
use axum::async_trait;
use log::info;
use microservices_shared::{
    domain_events::{DomainEventCallbacks, DomainEventCallbacksLoggerImpl},
    domain_ids::{FixtureId, RefereeId, TeamId, VenueId},
};
use opentelemetry::global::BoxedTracer;
use opentelemetry::{
    trace::{Span, Tracer},
    KeyValue,
};
use redis::Commands;
use sqlx::types::chrono::{DateTime, Utc};

pub struct DomainEventCallbacksImpl {
    redis_conn: redis::Connection,
    tracer: Arc<BoxedTracer>,
    delegate: DomainEventCallbacksLoggerImpl,
}

impl DomainEventCallbacksImpl {
    pub fn new(redis_conn: redis::Connection, tracer: Arc<BoxedTracer>) -> Self {
        Self {
            redis_conn,
            delegate: DomainEventCallbacksLoggerImpl::new(tracer.clone()),
            tracer,
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
        span.set_attribute(KeyValue::new("referee_id", referee_id.to_string()));
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
            .await?;

        invalidate_fixture_cache_entry(&mut self.redis_conn, fixture_id)
    }

    async fn on_fixture_venue_changed(
        &mut self,
        fixture_id: FixtureId,
        venue_id: VenueId,
        _tx_ctx: &mut Self::TxCtx,
    ) -> Result<(), String> {
        self.delegate
            .on_fixture_venue_changed(fixture_id, venue_id, _tx_ctx)
            .await?;

        invalidate_fixture_cache_entry(&mut self.redis_conn, fixture_id)
    }

    async fn on_fixture_cancelled(
        &mut self,
        fixture_id: FixtureId,
        _tx_ctx: &mut Self::TxCtx,
    ) -> Result<(), String> {
        self.delegate
            .on_fixture_cancelled(fixture_id, _tx_ctx)
            .await?;

        invalidate_fixture_cache_entry(&mut self.redis_conn, fixture_id)
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

    async fn on_first_referee_assignment_removed(
        &mut self,
        fixture_id: FixtureId,
        referee_id: RefereeId,
        tx_ctx: &mut Self::TxCtx,
    ) -> Result<(), String> {
        info!(
            "Received Domain Event in Fixtures: First referee assignment removed: {:?} -> {:?}",
            fixture_id, referee_id
        );

        let mut span = self.tracer.start("on_first_referee_assignment_removed");
        span.set_attribute(KeyValue::new("fixture_id", fixture_id.to_string()));
        span.set_attribute(KeyValue::new("referee_id", referee_id.to_string()));

        let fixture_repo = FixtureRepositoryPg::new();

        unassign_first_referee(fixture_id, &fixture_repo, tx_ctx).await?;

        invalidate_fixture_cache_entry(&mut self.redis_conn, fixture_id)
    }

    async fn on_second_referee_assignment_removed(
        &mut self,
        fixture_id: FixtureId,
        referee_id: RefereeId,
        tx_ctx: &mut Self::TxCtx,
    ) -> Result<(), String> {
        info!(
            "Received Domain Event in Fixtures: Second referee assignment removed: {:?} -> {:?}",
            fixture_id, referee_id
        );

        let mut span = self.tracer.start("on_second_referee_assignment_removed");
        span.set_attribute(KeyValue::new("fixture_id", fixture_id.to_string()));
        span.set_attribute(KeyValue::new("referee_id", referee_id.to_string()));

        let fixture_repo = FixtureRepositoryPg::new();

        unassign_second_referee(fixture_id, &fixture_repo, tx_ctx).await?;

        invalidate_fixture_cache_entry(&mut self.redis_conn, fixture_id)
    }

    async fn on_first_referee_assigned(
        &mut self,
        fixture_id: FixtureId,
        referee_id: RefereeId,
        tx_ctx: &mut Self::TxCtx,
    ) -> Result<(), String> {
        info!(
            "Received Domain Event in Fixtures: First referee assigned: {:?} -> {:?}",
            fixture_id, referee_id
        );

        let mut span = self.tracer.start("on_first_referee_assigned");
        span.set_attribute(KeyValue::new("fixture_id", fixture_id.to_string()));
        span.set_attribute(KeyValue::new("referee_id", referee_id.to_string()));

        let fixture_repo = FixtureRepositoryPg::new();

        assign_first_referee(fixture_id, referee_id, &fixture_repo, tx_ctx).await?;

        invalidate_fixture_cache_entry(&mut self.redis_conn, fixture_id)
    }

    async fn on_second_referee_assigned(
        &mut self,
        fixture_id: FixtureId,
        referee_id: RefereeId,
        tx_ctx: &mut Self::TxCtx,
    ) -> Result<(), String> {
        info!(
            "Received Domain Event in Fixtures: Second referee assigned: {:?} -> {:?}",
            fixture_id, referee_id
        );

        let mut span = self.tracer.start("on_second_referee_assigned");
        span.set_attribute(KeyValue::new("fixture_id", fixture_id.to_string()));
        span.set_attribute(KeyValue::new("referee_id", referee_id.to_string()));

        let fixture_repo = FixtureRepositoryPg::new();

        assign_second_referee(fixture_id, referee_id, &fixture_repo, tx_ctx).await?;

        invalidate_fixture_cache_entry(&mut self.redis_conn, fixture_id)
    }
}

fn invalidate_fixture_cache_entry(
    redis_conn: &mut redis::Connection,
    fixture_id: FixtureId,
) -> Result<(), String> {
    info!("Invalidating cache entry for fixture: {:?}", fixture_id);

    let key = format!("fixture_{}", fixture_id.0.to_string());
    let _result: Result<(), redis::RedisError> = redis_conn.del(key);

    _result.map_err(|e| e.to_string())
}
