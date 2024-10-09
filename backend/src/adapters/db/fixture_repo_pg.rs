use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::domain::{
    aggregates::{
        fixture::{Fixture, FixtureId},
        team::Team,
        venue::Venue,
    },
    repositories::fixture_repo::FixtureRepository,
};

pub struct FixtureRepositoryPg<'a> {
    pool: &'a Pool<Postgres>,
}

impl<'a> FixtureRepositoryPg<'a> {
    pub fn new(pool: &'a Pool<Postgres>) -> Self {
        Self { pool }
    }
}

struct FixtureDb {
    pub id: Uuid,
    pub date: DateTime<Utc>,
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

impl From<FixtureDb> for Fixture {
    fn from(fixture: FixtureDb) -> Self {
        Fixture::from_id(
            fixture.id,
            fixture.date,
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
            "SELECT f.fixture_id as id, f.date,
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
            "SELECT f.fixture_id as id, f.date,
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
        // NOTE: we do an upsert that only updates the stuff that is allowed to change: cancelled, date, venue_id
        sqlx::query!(
            "INSERT INTO rustddd.fixtures (fixture_id, date, venue_id, team_home_id, team_away_id) 
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (fixture_id) 
            DO UPDATE SET date = $2, venue_id = $3",
            fixture.id().0,
            fixture.date(),
            fixture.venue().id().0,
            fixture.team_home().id().0,
            fixture.team_away().id().0
        )
        .execute(self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }
}
