//! # Application Error Types
//!
//! This crate provides a centralized error handling system for the Financial Tracker API.
//!
//! ## Error Types
//!
//! [`AppError`] is the main error enum used throughout the application:
//!
//! - `NotFound` - Resource not found (HTTP 404)
//! - `BadRequest` - Invalid request data (HTTP 400)
//! - `ValidationError` - Validation failure (HTTP 400)
//! - `Unauthorized` - Authentication required/failed (HTTP 401)
//! - `Forbidden` - Access denied (HTTP 403)
//! - `Conflict` - Resource conflict (HTTP 409)
//! - `InternalError` - Server error (HTTP 500)
//!
//! ## Axum Integration
//!
//! `AppError` implements `IntoResponse` for seamless integration with Axum handlers.
//! Errors are automatically converted to JSON responses with appropriate HTTP status codes.

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use abbp_types::ErrorResponse;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Forbidden: {0}")]
    Forbidden(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Internal error: {0}")]
    InternalError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            Self::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            Self::BadRequest(msg) | Self::ValidationError(msg) => (StatusCode::BAD_REQUEST, msg),
            Self::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg),
            Self::Forbidden(msg) => (StatusCode::FORBIDDEN, msg),
            Self::Conflict(msg) => (StatusCode::CONFLICT, msg),
            Self::InternalError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        let error_response = ErrorResponse::new(message);
        let body = Json(error_response);

        (status, body).into_response()
    }
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        Self::InternalError(err.to_string())
    }
}
