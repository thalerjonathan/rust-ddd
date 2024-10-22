use std::fmt::Display;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use log::error;

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
