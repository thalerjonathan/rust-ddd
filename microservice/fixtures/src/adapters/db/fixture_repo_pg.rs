use chrono::{DateTime, Datelike, TimeZone, Utc};
use log::debug;
use microservices_shared::domain_ids::{FixtureId, RefereeId, TeamId, VenueId};
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

use crate::domain::{
    aggregates::fixture::{Fixture, FixtureStatus},
    repositories::fixture_repo::FixtureRepository,
};

pub struct FixtureRepositoryPg();

#[derive(sqlx::Type, Debug)]
#[sqlx(type_name = "rustddd.fixture_status", rename_all = "lowercase")]
enum FixtureStatusDb {
    Scheduled,
    Cancelled,
}

#[derive(sqlx::FromRow, Debug)]
struct FixtureDb {
    pub id: Uuid,
    pub date: DateTime<Utc>,
    pub status: FixtureStatusDb,
    pub venue_id: Uuid,
    pub team_home_id: Uuid,
    pub team_away_id: Uuid,
    pub first_referee_id: Option<Uuid>,
    pub second_referee_id: Option<Uuid>,
}

impl FixtureRepositoryPg {
    pub fn new() -> Self {
        Self {}
    }
}

impl From<FixtureStatusDb> for FixtureStatus {
    fn from(status: FixtureStatusDb) -> Self {
        match status {
            FixtureStatusDb::Scheduled => FixtureStatus::Scheduled,
            FixtureStatusDb::Cancelled => FixtureStatus::Cancelled,
        }
    }
}

impl From<FixtureStatus> for FixtureStatusDb {
    fn from(status: FixtureStatus) -> Self {
        match status {
            FixtureStatus::Scheduled => FixtureStatusDb::Scheduled,
            FixtureStatus::Cancelled => FixtureStatusDb::Cancelled,
        }
    }
}

impl From<FixtureDb> for Fixture {
    fn from(fixture: FixtureDb) -> Self {
        Fixture::from_id(
            FixtureId::from(fixture.id),
            fixture.date,
            fixture.status.into(),
            VenueId::from(fixture.venue_id),
            TeamId::from(fixture.team_home_id),
            TeamId::from(fixture.team_away_id),
            fixture.first_referee_id.map(|id| RefereeId::from(id)),
            fixture.second_referee_id.map(|id| RefereeId::from(id)),
        )
    }
}

impl FixtureRepository for FixtureRepositoryPg {
    type Error = String;
    type TxCtx = Transaction<'static, Postgres>;

    async fn find_by_id(
        &self,
        fixture_id: FixtureId,
        tx_ctx: &mut Self::TxCtx,
    ) -> Result<Option<Fixture>, Self::Error> {
        // NOTE: need to force nullable for referees, see https://docs.rs/sqlx/0.4.2/sqlx/macro.query.html#force-nullable
        let fixture: Option<FixtureDb> = sqlx::query_as(
            r#"SELECT f.fixture_id as id, f.date, f.status as "status: FixtureStatusDb", f.venue_id, f.team_home_id, f.team_away_id, f.first_referee_id as "first_referee_id?", f.second_referee_id as "second_referee_id?"
            FROM rustddd.fixtures f
            WHERE f.fixture_id = $1"#,
        )
        .bind( fixture_id.0)
        .fetch_optional(&mut **tx_ctx)
        .await
        .map_err(|e| e.to_string())?;

        debug!("FixtureDb: {:?}", fixture);

        Ok(fixture.map(|f| f.into()))
    }

    async fn get_all(&self, tx_ctx: &mut Self::TxCtx) -> Result<Vec<Fixture>, Self::Error> {
        let fixtures: Vec<FixtureDb> = sqlx::query_as(
            r#"SELECT f.fixture_id as id, f.date, f.status as "status: FixtureStatusDb", f.venue_id, f.team_home_id, f.team_away_id, f.first_referee_id as "first_referee_id?", f.second_referee_id as "second_referee_id?"
            FROM rustddd.fixtures f
            ORDER BY f.date ASC"#
        )
        .fetch_all(&mut **tx_ctx)
        .await
        .map_err(|e| e.to_string())?;

        Ok(fixtures.into_iter().map(Fixture::from).collect())
    }

    async fn find_by_day_and_venue(
        &self,
        date: &DateTime<Utc>,
        venue_id: VenueId,
        tx_ctx: &mut Self::TxCtx,
    ) -> Result<Vec<Fixture>, Self::Error> {
        let day_start = Utc
            .with_ymd_and_hms(date.year(), date.month(), date.day(), 0, 0, 0)
            .unwrap();
        let day_end = Utc
            .with_ymd_and_hms(date.year(), date.month(), date.day(), 23, 59, 59)
            .unwrap();

        debug!("day_start: {}", day_start);
        debug!("day_end: {}", day_end);

        let fixtures: Vec<FixtureDb> = sqlx::query_as(
            r#"SELECT f.fixture_id as id, f.date, f.status as "status: FixtureStatusDb", f.venue_id, f.team_home_id, f.team_away_id, f.first_referee_id as "first_referee_id?", f.second_referee_id as "second_referee_id?"
            FROM rustddd.fixtures f
            WHERE f.date BETWEEN $1 AND $2 AND f.venue_id = $3
            ORDER BY f.date ASC"#
        )
        .bind(day_start)
        .bind(day_end)
        .bind(venue_id.0)
        .fetch_all(&mut **tx_ctx)
        .await
        .map_err(|e| e.to_string())?;

        Ok(fixtures.into_iter().map(Fixture::from).collect())
    }

    async fn find_by_day_and_team(
        &self,
        date: &DateTime<Utc>,
        team_id: TeamId,
        tx_ctx: &mut Self::TxCtx,
    ) -> Result<Vec<Fixture>, Self::Error> {
        let day_start = Utc
            .with_ymd_and_hms(date.year(), date.month(), date.day(), 0, 0, 0)
            .unwrap();
        let day_end = Utc
            .with_ymd_and_hms(date.year(), date.month(), date.day(), 23, 59, 59)
            .unwrap();

        debug!("day_start: {}", day_start);
        debug!("day_end: {}", day_end);

        let fixtures: Vec<FixtureDb> = sqlx::query_as(
            r#"SELECT f.fixture_id as id, f.date, f.status as "status: FixtureStatusDb", f.venue_id, f.team_home_id, f.team_away_id, f.first_referee_id as "first_referee_id?", f.second_referee_id as "second_referee_id?"
            FROM rustddd.fixtures f
            WHERE f.date BETWEEN $1 AND $2 AND (f.team_home_id = $3 OR f.team_away_id = $3)
            ORDER BY f.date ASC"#
        )
        .bind(day_start)
        .bind(day_end)
        .bind(team_id.0)
        .fetch_all(&mut **tx_ctx)
        .await
        .map_err(|e| e.to_string())?;

        Ok(fixtures.into_iter().map(Fixture::from).collect())
    }

    async fn save(&self, fixture: &Fixture, tx_ctx: &mut Self::TxCtx) -> Result<(), Self::Error> {
        let status: FixtureStatusDb = FixtureStatusDb::from(fixture.status().clone());
        let first_referee_id = fixture.first_referee_id().map(|r| r.0);
        let second_referee_id = fixture.second_referee_id().map(|r| r.0);
        // NOTE: we do an upsert that only updates the stuff that is allowed to change: cancelled, date, venue_id, first_referee_id, second_referee_id
        sqlx::query(
            "INSERT INTO rustddd.fixtures (fixture_id, date, venue_id, team_home_id, team_away_id, status, first_referee_id, second_referee_id) 
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            ON CONFLICT (fixture_id) 
            DO UPDATE SET date = $2, venue_id = $3, status = $6, first_referee_id = $7, second_referee_id = $8"
        )
        .bind(fixture.id().0)
        .bind(fixture.date())
        .bind(fixture.venue_id().0)
        .bind(fixture.team_home_id().0)
        .bind(fixture.team_away_id().0)
        .bind(status as FixtureStatusDb)
        .bind(first_referee_id)
        .bind(second_referee_id)
        .execute(&mut **tx_ctx)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }
}
