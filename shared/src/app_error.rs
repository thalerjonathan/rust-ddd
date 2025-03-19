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
    status_code: Option<StatusCode>,
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
            self.status_code
                .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
            format!("Failed processing request due to error: {}", self),
        )
            .into_response()
    }
}

impl AppError {
    pub fn from_error(error: &str) -> Self {
        Self {
            error: error.to_string(),
            status_code: Option::None,
        }
    }

    pub fn from_error_unauthorized(error: &str) -> Self {
        AppError::from_error_with_status(error, StatusCode::UNAUTHORIZED)
    }

    pub fn from_error_with_status(error: &str, status: StatusCode) -> Self {
        Self {
            error: error.to_string(),
            status_code: Some(status),
        }
    }
}
