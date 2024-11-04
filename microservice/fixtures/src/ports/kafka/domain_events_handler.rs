use axum::async_trait;
use log::info;
use microservices_shared::{
    domain_events::DomainEventCallbacks,
    domain_ids::{FixtureId, RefereeId, TeamId, VenueId},
};
use redis::Commands;
use sqlx::{
    types::chrono::{DateTime, Utc},
    PgPool,
};

use crate::{
    adapters::db::fixture_repo_pg::FixtureRepositoryPg,
    domain::repositories::fixture_repo::FixtureRepository,
};

pub struct DomainEventCallbacksImpl {
    redis_conn: redis::Connection,
    connection_pool: PgPool,
}

impl DomainEventCallbacksImpl {
    pub fn new(redis_conn: redis::Connection, connection_pool: PgPool) -> Self {
        Self {
            redis_conn,
            connection_pool,
        }
    }
}

#[async_trait]
impl DomainEventCallbacks for DomainEventCallbacksImpl {
    async fn on_referee_created(&mut self, referee_id: RefereeId) -> Result<(), String> {
        info!("Received Domain Event: Referee created: {:?}", referee_id);
        Ok(())
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

        info!("Invalidating cache entry for referee: {:?}", referee_id);

        // NOTE: invalidate the cache entry for the referee
        let key = format!("referee_{}", referee_id.0.to_string());
        let _result: Result<(), redis::RedisError> = self.redis_conn.del(key);

        _result.map_err(|e| e.to_string())
    }

    async fn on_team_created(&mut self, team_id: TeamId) -> Result<(), String> {
        info!("Received Domain Event: Team created: {:?}", team_id);
        Ok(())
    }

    async fn on_venue_created(&mut self, venue_id: VenueId) -> Result<(), String> {
        info!("Received Domain Event: Venue created: {:?}", venue_id);
        Ok(())
    }

    async fn on_fixture_created(&mut self, fixture_id: FixtureId) -> Result<(), String> {
        info!("Received Domain Event: Fixture created: {:?}", fixture_id);
        Ok(())
    }

    async fn on_fixture_date_changed(
        &mut self,
        fixture_id: FixtureId,
        date: DateTime<Utc>,
    ) -> Result<(), String> {
        info!(
            "Received Domain Event: Fixture date changed: {:?} -> {}",
            fixture_id, date
        );

        invalidate_fixture_cache_entry(&mut self.redis_conn, fixture_id)
    }

    async fn on_fixture_venue_changed(
        &mut self,
        fixture_id: FixtureId,
        venue_id: VenueId,
    ) -> Result<(), String> {
        info!(
            "Received Domain Event: Fixture venue changed: {:?} -> {:?}",
            fixture_id, venue_id
        );
        invalidate_fixture_cache_entry(&mut self.redis_conn, fixture_id)
    }

    async fn on_fixture_cancelled(&mut self, fixture_id: FixtureId) -> Result<(), String> {
        info!("Received Domain Event: Fixture cancelled: {:?}", fixture_id);
        invalidate_fixture_cache_entry(&mut self.redis_conn, fixture_id)
    }

    async fn on_availability_declared(
        &mut self,
        fixture_id: FixtureId,
        referee_id: RefereeId,
    ) -> Result<(), String> {
        info!(
            "Received Domain Event: Availability declared: {:?} -> {:?}",
            fixture_id, referee_id
        );
        Ok(())
    }

    async fn on_availability_withdrawn(
        &mut self,
        fixture_id: FixtureId,
        referee_id: RefereeId,
    ) -> Result<(), String> {
        info!(
            "Received Domain Event: Availability withdrawn: {:?} -> {:?}",
            fixture_id, referee_id
        );
        Ok(())
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
