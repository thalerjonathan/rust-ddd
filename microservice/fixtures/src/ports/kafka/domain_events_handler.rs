use std::sync::Arc;

use crate::{
    adapters::db::fixture_repo_pg::FixtureRepositoryPg,
    domain::repositories::fixture_repo::FixtureRepository,
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
use sqlx::{
    types::chrono::{DateTime, Utc},
    PgPool,
};

pub struct DomainEventCallbacksImpl {
    redis_conn: redis::Connection,
    connection_pool: PgPool,
    tracer: Arc<BoxedTracer>,
    delegate: DomainEventCallbacksLoggerImpl,
}

impl DomainEventCallbacksImpl {
    pub fn new(
        redis_conn: redis::Connection,
        connection_pool: PgPool,
        tracer: Arc<BoxedTracer>,
    ) -> Self {
        Self {
            redis_conn,
            connection_pool,
            delegate: DomainEventCallbacksLoggerImpl::new(tracer.clone()),
            tracer,
        }
    }
}

#[async_trait]
impl DomainEventCallbacks for DomainEventCallbacksImpl {
    async fn on_referee_created(&mut self, referee_id: RefereeId) -> Result<(), String> {
        self.delegate.on_referee_created(referee_id).await
    }

    async fn on_referee_club_changed(
        &mut self,
        referee_id: RefereeId,
        club_name: String,
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

    async fn on_team_created(&mut self, team_id: TeamId) -> Result<(), String> {
        self.delegate.on_team_created(team_id).await
    }

    async fn on_venue_created(&mut self, venue_id: VenueId) -> Result<(), String> {
        self.delegate.on_venue_created(venue_id).await
    }

    async fn on_fixture_created(&mut self, fixture_id: FixtureId) -> Result<(), String> {
        self.delegate.on_fixture_created(fixture_id).await
    }

    async fn on_fixture_date_changed(
        &mut self,
        fixture_id: FixtureId,
        date: DateTime<Utc>,
    ) -> Result<(), String> {
        self.delegate
            .on_fixture_date_changed(fixture_id, date)
            .await?;

        invalidate_fixture_cache_entry(&mut self.redis_conn, fixture_id)
    }

    async fn on_fixture_venue_changed(
        &mut self,
        fixture_id: FixtureId,
        venue_id: VenueId,
    ) -> Result<(), String> {
        self.delegate
            .on_fixture_venue_changed(fixture_id, venue_id)
            .await?;

        invalidate_fixture_cache_entry(&mut self.redis_conn, fixture_id)
    }

    async fn on_fixture_cancelled(&mut self, fixture_id: FixtureId) -> Result<(), String> {
        self.delegate.on_fixture_cancelled(fixture_id).await?;

        invalidate_fixture_cache_entry(&mut self.redis_conn, fixture_id)
    }

    async fn on_availability_declared(
        &mut self,
        fixture_id: FixtureId,
        referee_id: RefereeId,
    ) -> Result<(), String> {
        self.delegate
            .on_availability_declared(fixture_id, referee_id)
            .await
    }

    async fn on_availability_withdrawn(
        &mut self,
        fixture_id: FixtureId,
        referee_id: RefereeId,
    ) -> Result<(), String> {
        self.delegate
            .on_availability_withdrawn(fixture_id, referee_id)
            .await
    }

    async fn on_first_referee_assignment_removed(
        &mut self,
        fixture_id: FixtureId,
        referee_id: RefereeId,
    ) -> Result<(), String> {
        info!(
            "Received Domain Event: First referee assignment removed: {:?} -> {:?}",
            fixture_id, referee_id
        );

        let mut span = self.tracer.start("on_first_referee_assignment_removed");
        span.set_attribute(KeyValue::new("fixture_id", fixture_id.to_string()));
        span.set_attribute(KeyValue::new("referee_id", referee_id.to_string()));

        let mut tx = self
            .connection_pool
            .begin()
            .await
            .map_err(|e| e.to_string())?;

        let fixture_repo = FixtureRepositoryPg::new();
        let mut fixture = fixture_repo
            .find_by_id(fixture_id, &mut tx)
            .await
            .map_err(|e| e.to_string())?
            .expect(format!("Fixture not found: {:?}", fixture_id).as_str());

        fixture.unassign_first_referee();

        fixture_repo
            .save(&fixture, &mut tx)
            .await
            .map_err(|e| e.to_string())?;

        tx.commit().await.map_err(|e| e.to_string())?;

        invalidate_fixture_cache_entry(&mut self.redis_conn, fixture_id)
    }

    async fn on_second_referee_assignment_removed(
        &mut self,
        fixture_id: FixtureId,
        referee_id: RefereeId,
    ) -> Result<(), String> {
        info!(
            "Received Domain Event: Second referee assignment removed: {:?} -> {:?}",
            fixture_id, referee_id
        );

        let mut span = self.tracer.start("on_second_referee_assignment_removed");
        span.set_attribute(KeyValue::new("fixture_id", fixture_id.to_string()));
        span.set_attribute(KeyValue::new("referee_id", referee_id.to_string()));

        let mut tx = self
            .connection_pool
            .begin()
            .await
            .map_err(|e| e.to_string())?;

        let fixture_repo = FixtureRepositoryPg::new();
        let mut fixture = fixture_repo
            .find_by_id(fixture_id, &mut tx)
            .await
            .map_err(|e| e.to_string())?
            .expect(format!("Fixture not found: {:?}", fixture_id).as_str());

        fixture.unassign_second_referee();

        fixture_repo
            .save(&fixture, &mut tx)
            .await
            .map_err(|e| e.to_string())?;

        tx.commit().await.map_err(|e| e.to_string())?;

        invalidate_fixture_cache_entry(&mut self.redis_conn, fixture_id)
    }

    async fn on_first_referee_assigned(
        &mut self,
        fixture_id: FixtureId,
        referee_id: RefereeId,
    ) -> Result<(), String> {
        info!(
            "Received Domain Event: First referee assigned: {:?} -> {:?}",
            fixture_id, referee_id
        );

        let mut span = self.tracer.start("on_first_referee_assigned");
        span.set_attribute(KeyValue::new("fixture_id", fixture_id.to_string()));
        span.set_attribute(KeyValue::new("referee_id", referee_id.to_string()));

        let mut tx = self
            .connection_pool
            .begin()
            .await
            .map_err(|e| e.to_string())?;

        let fixture_repo = FixtureRepositoryPg::new();
        let mut fixture = fixture_repo
            .find_by_id(fixture_id, &mut tx)
            .await
            .map_err(|e| e.to_string())?
            .expect(format!("Fixture not found: {:?}", fixture_id).as_str());

        // NOTE: ideally we would check if the referee is available for the fixture

        fixture.assign_first_referee(referee_id);

        fixture_repo
            .save(&fixture, &mut tx)
            .await
            .map_err(|e| e.to_string())?;

        tx.commit().await.map_err(|e| e.to_string())?;

        invalidate_fixture_cache_entry(&mut self.redis_conn, fixture_id)
    }

    async fn on_second_referee_assigned(
        &mut self,
        fixture_id: FixtureId,
        referee_id: RefereeId,
    ) -> Result<(), String> {
        info!(
            "Received Domain Event: Second referee assigned: {:?} -> {:?}",
            fixture_id, referee_id
        );

        let mut span = self.tracer.start("on_second_referee_assigned");
        span.set_attribute(KeyValue::new("fixture_id", fixture_id.to_string()));
        span.set_attribute(KeyValue::new("referee_id", referee_id.to_string()));

        let mut tx = self
            .connection_pool
            .begin()
            .await
            .map_err(|e| e.to_string())?;

        let fixture_repo = FixtureRepositoryPg::new();
        let mut fixture = fixture_repo
            .find_by_id(fixture_id, &mut tx)
            .await
            .map_err(|e| e.to_string())?
            .expect(format!("Fixture not found: {:?}", fixture_id).as_str());

        // NOTE: ideally we would check if the referee is available for the fixture

        fixture.assign_second_referee(referee_id);

        fixture_repo
            .save(&fixture, &mut tx)
            .await
            .map_err(|e| e.to_string())?;

        tx.commit().await.map_err(|e| e.to_string())?;

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
