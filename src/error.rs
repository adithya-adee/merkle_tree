use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use std::fmt;

/// Application error types
#[derive(Debug)]
pub enum AppError {
    NotFound(String),
    TreeBuildError(String),
    InvalidInput(String),
    Internal(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::NotFound(s) => write!(f, "Not found: {}", s),
            AppError::TreeBuildError(s) => write!(f, "Tree build error: {}", s),
            AppError::InvalidInput(s) => write!(f, "Invalid input: {}", s),
            AppError::Internal(s) => write!(f, "Internal error: {}", s),
        }
    }
}

impl std::error::Error for AppError {}

/// Error response structure
#[derive(Serialize)]
struct ErrorResponse {
    error: String,
    message: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_type) = match &self {
            AppError::NotFound(_) => (StatusCode::NOT_FOUND, "NOT_FOUND"),
            AppError::InvalidInput(_) => (StatusCode::BAD_REQUEST, "INVALID_INPUT"),
            AppError::TreeBuildError(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "TREE_BUILD_ERROR")
            }
            AppError::Internal(_) => (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR"),
        };

        let body = Json(ErrorResponse {
            error: error_type.to_string(),
            message: self.to_string(),
        });

        (status, body).into_response()
    }
}