use chrono::{DateTime, Datelike, TimeZone, Utc};
use log::debug;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::domain::{
    aggregates::{
        fixture::{Fixture, FixtureId, FixtureStatus},
        team::Team,
        venue::Venue,
    },
    repositories::fixture_repo::FixtureRepository,
};


pub struct FixtureRepositoryPg<'a> {
    pool: &'a Pool<Postgres>
}

impl<'a> FixtureRepositoryPg<'a> {
    pub fn new(pool: &'a Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[derive(sqlx::Type, Debug)]
enum FixtureStatusDb {
    Scheduled,
    Cancelled,
}

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
}

impl From<String> for FixtureStatusDb {
    fn from(status: String) -> Self {
        let status_str: &str = &status;
        match status_str {
            "Scheduled" => FixtureStatusDb::Scheduled,
            "Cancelled" => FixtureStatusDb::Cancelled,
            _ => panic!("Invalid fixture status"),
        }
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

impl From<FixtureDb> for Fixture {
    fn from(fixture: FixtureDb) -> Self {
        Fixture::from_id(
            fixture.id,
            fixture.date,
            fixture.status.into(),
            Venue::from_id(
                fixture.venue_id,
                fixture.venue_name,
                fixture.venue_street,
                fixture.venue_zip,
                fixture.venue_city,
                fixture.venue_telephone,
                fixture.venue_email,
            ),
            Team::from_id(
                fixture.team_home_id,
                fixture.team_home_name,
                fixture.team_home_club,
            ),
            Team::from_id(
                fixture.team_away_id,
                fixture.team_away_name,
                fixture.team_away_club,
            ),
        )
    }
}

impl<'a> FixtureRepository for FixtureRepositoryPg<'a> {
    type Error = String;

    async fn find_by_id(&self, id: &FixtureId) -> Result<Option<Fixture>, Self::Error> {
        let fixture: Option<FixtureDb> = sqlx::query_as!(
            FixtureDb,
            "SELECT f.fixture_id as id, f.date, f.status,
                v.venue_id as venue_id, v.name as venue_name, v.street as venue_street, v.zip as venue_zip, v.city as venue_city, v.telephone as venue_telephone, v.email as venue_email,
                th.team_id as team_home_id, th.name as team_home_name, th.club as team_home_club,
                ta.team_id as team_away_id, ta.name as team_away_name, ta.club as team_away_club
            FROM rustddd.fixtures f
            JOIN rustddd.venues v ON v.venue_id = f.venue_id
            JOIN rustddd.teams th ON th.team_id = f.team_home_id
            JOIN rustddd.teams ta ON ta.team_id = f.team_away_id
            WHERE f.fixture_id = $1",
            id.0,
        )
        .fetch_optional(self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(fixture.map(|f| f.into()))
    }

    async fn get_all(&self) -> Result<Vec<Fixture>, Self::Error> {
        let fixtures: Vec<FixtureDb> = sqlx::query_as!(
            FixtureDb,
            "SELECT f.fixture_id as id, f.date, f.status,
                v.venue_id as venue_id, v.name as venue_name, v.street as venue_street, v.zip as venue_zip, v.city as venue_city, v.telephone as venue_telephone, v.email as venue_email,
                th.team_id as team_home_id, th.name as team_home_name, th.club as team_home_club,
                ta.team_id as team_away_id, ta.name as team_away_name, ta.club as team_away_club
            FROM rustddd.fixtures f
            JOIN rustddd.venues v ON v.venue_id = f.venue_id
            JOIN rustddd.teams th ON th.team_id = f.team_home_id
            JOIN rustddd.teams ta ON ta.team_id = f.team_away_id",
        )
        .fetch_all(self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(fixtures.into_iter().map(Fixture::from).collect())
    }

    async fn save(&self, fixture: &Fixture) -> Result<(), Self::Error> {
        let status = format!("{:?}", fixture.status());
        // NOTE: we do an upsert that only updates the stuff that is allowed to change: cancelled, date, venue_id
        sqlx::query!(
            "INSERT INTO rustddd.fixtures (fixture_id, date, venue_id, team_home_id, team_away_id, status) 
            VALUES ($1, $2, $3, $4, $5, $6)
            ON CONFLICT (fixture_id) 
            DO UPDATE SET date = $2, venue_id = $3, status = $6",
            fixture.id().0,
            fixture.date(),
            fixture.venue().id().0, 
            fixture.team_home().id().0,
            fixture.team_away().id().0,
            status
        )
        .execute(self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn find_by_day_and_venue(
        &self,
        date: &DateTime<Utc>,
        venue_id: &crate::domain::aggregates::venue::VenueId,
    ) -> Result<Vec<Fixture>, Self::Error> {
        let day_start = Utc
            .with_ymd_and_hms(date.year(), date.month(), date.day(), 0, 0, 0)
            .unwrap();
        let day_end = Utc
            .with_ymd_and_hms(date.year(), date.month(), date.day(), 23, 59, 59)
            .unwrap();

        debug!("day_start: {}", day_start);
        debug!("day_end: {}", day_end);

        let fixtures: Vec<FixtureDb> = sqlx::query_as!(
            FixtureDb,
            "SELECT f.fixture_id as id, f.date, f.status,
                v.venue_id as venue_id, v.name as venue_name, v.street as venue_street, v.zip as venue_zip, v.city as venue_city, v.telephone as venue_telephone, v.email as venue_email,
                th.team_id as team_home_id, th.name as team_home_name, th.club as team_home_club,
                ta.team_id as team_away_id, ta.name as team_away_name, ta.club as team_away_club
            FROM rustddd.fixtures f
            JOIN rustddd.venues v ON v.venue_id = f.venue_id
            JOIN rustddd.teams th ON th.team_id = f.team_home_id
            JOIN rustddd.teams ta ON ta.team_id = f.team_away_id
            WHERE f.date BETWEEN $1 AND $2 AND f.venue_id = $3",
            day_start,
            day_end,
            venue_id.0,
        )
        .fetch_all(self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(fixtures.into_iter().map(Fixture::from).collect())
    }

    async fn find_by_day_and_team(
        &self,
        date: &DateTime<Utc>,
        team_id: &crate::domain::aggregates::team::TeamId,
    ) -> Result<Vec<Fixture>, Self::Error> {
        let day_start = Utc
            .with_ymd_and_hms(date.year(), date.month(), date.day(), 0, 0, 0)
            .unwrap();
        let day_end = Utc
            .with_ymd_and_hms(date.year(), date.month(), date.day(), 23, 59, 59)
            .unwrap();

        debug!("day_start: {}", day_start);
        debug!("day_end: {}", day_end);

        let fixtures: Vec<FixtureDb> = sqlx::query_as!(
            FixtureDb,
            "SELECT f.fixture_id as id, f.date, f.status,
                v.venue_id as venue_id, v.name as venue_name, v.street as venue_street, v.zip as venue_zip, v.city as venue_city, v.telephone as venue_telephone, v.email as venue_email,
                th.team_id as team_home_id, th.name as team_home_name, th.club as team_home_club,
                ta.team_id as team_away_id, ta.name as team_away_name, ta.club as team_away_club
            FROM rustddd.fixtures f
            JOIN rustddd.venues v ON v.venue_id = f.venue_id
            JOIN rustddd.teams th ON th.team_id = f.team_home_id
            JOIN rustddd.teams ta ON ta.team_id = f.team_away_id
            WHERE f.date BETWEEN $1 AND $2 AND (f.team_home_id = $3 OR f.team_away_id = $3)",
            day_start,
            day_end,
            team_id.0,
        )
        .fetch_all(self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(fixtures.into_iter().map(Fixture::from).collect())
    }
}
