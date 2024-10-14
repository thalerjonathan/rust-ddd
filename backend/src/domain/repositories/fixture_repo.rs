use chrono::{DateTime, Utc};

use crate::domain::aggregates::{
    fixture::{Fixture, FixtureId},
    team::TeamId,
    venue::VenueId,
};

pub trait FixtureRepository {
    type Error;
    type TxCtx;
    async fn find_by_id(
        &self,
        id: &FixtureId,
        tx_ctx: &mut Self::TxCtx,
    ) -> Result<Option<Fixture>, Self::Error>;
    async fn get_all(&self, tx_ctx: &mut Self::TxCtx) -> Result<Vec<Fixture>, Self::Error>;
    async fn find_by_day_and_venue(
        &self,
        date: &DateTime<Utc>,
        venue_id: &VenueId,
        tx_ctx: &mut Self::TxCtx,
    ) -> Result<Vec<Fixture>, Self::Error>;
    async fn find_by_day_and_team(
        &self,
        date: &DateTime<Utc>,
        team_id: &TeamId,
        tx_ctx: &mut Self::TxCtx,
    ) -> Result<Vec<Fixture>, Self::Error>;
    async fn save(&self, fixture: &Fixture, tx_ctx: &mut Self::TxCtx) -> Result<(), Self::Error>;
}
