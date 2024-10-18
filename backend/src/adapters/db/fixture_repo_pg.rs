use chrono::{DateTime, Datelike, TimeZone, Utc};
use log::debug;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

use crate::domain::{
    aggregates::{
        fixture::{Fixture, FixtureId, FixtureStatus},
        referee::Referee,
        team::{Team, TeamId},
        venue::{Venue, VenueId},
    },
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
    pub venue_name: String,
    pub venue_street: String,
    pub venue_zip: String,
    pub venue_city: String,
    pub venue_telephone: Option<String>,
    pub venue_email: Option<String>,
    pub team_home_id: Uuid,
    pub team_home_name: String,
    pub team_home_club: String,
    pub team_away_id: Uuid,
    pub team_away_name: String,
    pub team_away_club: String,
    pub first_referee_id: Option<Uuid>,
    pub first_referee_name: Option<String>,
    pub first_referee_club: Option<String>,
    pub second_referee_id: Option<Uuid>,
    pub second_referee_name: Option<String>,
    pub second_referee_club: Option<String>,
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
            Venue::from_id(
                VenueId::from(fixture.venue_id),
                fixture.venue_name,
                fixture.venue_street,
                fixture.venue_zip,
                fixture.venue_city,
                fixture.venue_telephone,
                fixture.venue_email,
            ),
            Team::from_id(
                TeamId::from(fixture.team_home_id),
                fixture.team_home_name,
                fixture.team_home_club,
            ),
            Team::from_id(
                TeamId::from(fixture.team_away_id),
                fixture.team_away_name,
                fixture.team_away_club,
            ),
            fixture.first_referee_id.map(|id| {
                Referee::from_id(
                    id.into(),
                    fixture
                        .first_referee_name
                        .expect("first_referee_name is required"),
                    fixture
                        .first_referee_club
                        .expect("first_referee_club is required"),
                )
            }),
            fixture.second_referee_id.map(|id| {
                Referee::from_id(
                    id.into(),
                    fixture
                        .second_referee_name
                        .expect("second_referee_name is required"),
                    fixture
                        .second_referee_club
                        .expect("second_referee_club is required"),
                )
            }),
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
        let fixture: Option<FixtureDb> = sqlx::query_as!(
            FixtureDb,
            "SELECT f.fixture_id as id, f.date, f.status as \"status: FixtureStatusDb\",
                v.venue_id as venue_id, v.name as venue_name, v.street as venue_street, v.zip as venue_zip, v.city as venue_city, v.telephone as venue_telephone, v.email as venue_email,
                th.team_id as team_home_id, th.name as team_home_name, th.club as team_home_club,
                ta.team_id as team_away_id, ta.name as team_away_name, ta.club as team_away_club,
                r1.referee_id as \"first_referee_id?\", r1.name as \"first_referee_name?\", r1.club as \"first_referee_club?\",
                r2.referee_id as \"second_referee_id?\", r2.name as \"second_referee_name?\", r2.club as \"second_referee_club?\"
            FROM rustddd.fixtures f
            JOIN rustddd.venues v ON v.venue_id = f.venue_id
            JOIN rustddd.teams th ON th.team_id = f.team_home_id
            JOIN rustddd.teams ta ON ta.team_id = f.team_away_id
            LEFT JOIN rustddd.referees r1 ON r1.referee_id = f.first_referee_id
            LEFT JOIN rustddd.referees r2 ON r2.referee_id = f.second_referee_id
            WHERE f.fixture_id = $1", fixture_id.0
        )
        .fetch_optional(&mut **tx_ctx)
        .await
        .map_err(|e| e.to_string())?;

        debug!("FixtureDb: {:?}", fixture);

        Ok(fixture.map(|f| f.into()))
    }

    async fn get_all(&self, tx_ctx: &mut Self::TxCtx) -> Result<Vec<Fixture>, Self::Error> {
        let fixtures: Vec<FixtureDb> = sqlx::query_as!(
            FixtureDb,
            "SELECT f.fixture_id as id, f.date, f.status as \"status: FixtureStatusDb\",
                v.venue_id as venue_id, v.name as venue_name, v.street as venue_street, v.zip as venue_zip, v.city as venue_city, v.telephone as venue_telephone, v.email as venue_email,
                th.team_id as team_home_id, th.name as team_home_name, th.club as team_home_club,
                ta.team_id as team_away_id, ta.name as team_away_name, ta.club as team_away_club,
                r1.referee_id as \"first_referee_id?\", r1.name as \"first_referee_name?\", r1.club as \"first_referee_club?\",
                r2.referee_id as \"second_referee_id?\", r2.name as \"second_referee_name?\", r2.club as \"second_referee_club?\"
            FROM rustddd.fixtures f
            JOIN rustddd.venues v ON v.venue_id = f.venue_id
            JOIN rustddd.teams th ON th.team_id = f.team_home_id
            JOIN rustddd.teams ta ON ta.team_id = f.team_away_id
            LEFT JOIN rustddd.referees r1 ON r1.referee_id = f.first_referee_id
            LEFT JOIN rustddd.referees r2 ON r2.referee_id = f.second_referee_id
            ORDER BY f.date ASC",
        )
        .fetch_all(&mut **tx_ctx)
        .await
        .map_err(|e| e.to_string())?;

        Ok(fixtures.into_iter().map(Fixture::from).collect())
    }

    async fn find_by_day_and_venue(
        &self,
        date: &DateTime<Utc>,
        venue_id: crate::domain::aggregates::venue::VenueId,
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
            "SELECT f.fixture_id as id, f.date, f.status as \"status: FixtureStatusDb\",
                v.venue_id as venue_id, v.name as venue_name, v.street as venue_street, v.zip as venue_zip, v.city as venue_city, v.telephone as venue_telephone, v.email as venue_email,
                th.team_id as team_home_id, th.name as team_home_name, th.club as team_home_club,
                ta.team_id as team_away_id, ta.name as team_away_name, ta.club as team_away_club,
                r1.referee_id as first_referee_id, r1.name as first_referee_name, r1.club as first_referee_club,
                r2.referee_id as second_referee_id, r2.name as second_referee_name, r2.club as second_referee_club
            FROM rustddd.fixtures f
            JOIN rustddd.venues v ON v.venue_id = f.venue_id
            JOIN rustddd.teams th ON th.team_id = f.team_home_id
            JOIN rustddd.teams ta ON ta.team_id = f.team_away_id
            LEFT JOIN rustddd.referees r1 ON r1.referee_id = f.first_referee_id
            LEFT JOIN rustddd.referees r2 ON r2.referee_id = f.second_referee_id
            WHERE f.date BETWEEN $1 AND $2 AND f.venue_id = $3
            ORDER BY f.date ASC"
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
            "SELECT f.fixture_id as id, f.date, f.status as \"status: FixtureStatusDb\",
                v.venue_id as venue_id, v.name as venue_name, v.street as venue_street, v.zip as venue_zip, v.city as venue_city, v.telephone as venue_telephone, v.email as venue_email,
                th.team_id as team_home_id, th.name as team_home_name, th.club as team_home_club,
                ta.team_id as team_away_id, ta.name as team_away_name, ta.club as team_away_club,
                r1.referee_id as first_referee_id, r1.name as first_referee_name, r1.club as first_referee_club,
                r2.referee_id as second_referee_id, r2.name as second_referee_name, r2.club as second_referee_club
            FROM rustddd.fixtures f
            JOIN rustddd.venues v ON v.venue_id = f.venue_id
            JOIN rustddd.teams th ON th.team_id = f.team_home_id
            JOIN rustddd.teams ta ON ta.team_id = f.team_away_id
            LEFT JOIN rustddd.referees r1 ON r1.referee_id = f.first_referee_id
            LEFT JOIN rustddd.referees r2 ON r2.referee_id = f.second_referee_id
            WHERE f.date BETWEEN $1 AND $2 AND (f.team_home_id = $3 OR f.team_away_id = $3)
            ORDER BY f.date ASC"
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
        let first_referee_id = fixture.first_referee().map(|r| r.id().0);
        let second_referee_id = fixture.second_referee().map(|r| r.id().0);
        // NOTE: we do an upsert that only updates the stuff that is allowed to change: cancelled, date, venue_id, first_referee_id, second_referee_id
        sqlx::query!(
            "INSERT INTO rustddd.fixtures (fixture_id, date, venue_id, team_home_id, team_away_id, status, first_referee_id, second_referee_id) 
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            ON CONFLICT (fixture_id) 
            DO UPDATE SET date = $2, venue_id = $3, status = $6, first_referee_id = $7, second_referee_id = $8",
            fixture.id().0,
            fixture.date(),
            fixture.venue().id().0,
            fixture.team_home().id().0,
            fixture.team_away().id().0,
            status as FixtureStatusDb,
            first_referee_id,
            second_referee_id
        )
        .execute(&mut **tx_ctx)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }
}
