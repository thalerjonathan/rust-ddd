use std::fmt::Display;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use log::error;
use shared::{FixtureDTO, RefereeDTO, TeamDTO, VenueDTO};

use crate::domain::aggregates::{fixture::Fixture, referee::Referee, team::Team, venue::Venue};
/// Represents an application error, where the application failed to handle a response
/// This is used to map such errors to 500 internal server error HTTP codes
#[derive(Debug)]
pub struct AppError {
    error: String,
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.error.fmt(f)
    }
}

// NOTE: need to implement IntoResponse so that axum knows how to return 500 from an AppError
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        error!("Response error: {}", self);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed processing request due to error: {}", self),
        )
            .into_response()
    }
}

impl AppError {
    pub fn from_error(error: &str) -> Self {
        Self {
            error: error.to_string(),
        }
    }
}

impl From<Fixture> for FixtureDTO {
    fn from(fixture: Fixture) -> Self {
        FixtureDTO {
            id: fixture.id().0,
            date: fixture.date().clone(),
            venue: fixture.venue().clone().into(),
            team_home: fixture.team_home().clone().into(),
            team_away: fixture.team_away().clone().into(),
        }
    }
}

impl From<Referee> for RefereeDTO {
    fn from(referee: Referee) -> Self {
        RefereeDTO {
            id: referee.id().0,
            name: referee.name().to_string(),
            club: referee.club().to_string(),
        }
    }
}

impl From<Team> for TeamDTO {
    fn from(team: Team) -> Self {
        Self {
            id: team.id().0,
            name: team.name().to_string(),
            club: team.club().to_string(),
        }
    }
}
impl From<Venue> for VenueDTO {
    fn from(venue: Venue) -> Self {
        VenueDTO {
            id: venue.id().0,
            name: venue.name().to_string(),
            street: venue.street().to_string(),
            zip: venue.zip().to_string(),
            city: venue.city().to_string(),
            telephone: venue.telephone(),
            email: venue.email(),
        }
    }
}
