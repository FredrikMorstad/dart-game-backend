use core::fmt;
use std::num::ParseIntError;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};

pub enum ApiError {
    RecordNotFound,
    BadRequest(String),
    DatabaseError(sea_orm::DbErr),
    ConflictError(String),
    UnknownError(String),
}

#[derive(Serialize, Deserialize)]
struct ErrorMessage {
    message: String,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status_code, message) = match self {
            ApiError::DatabaseError(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("An unexpected error fetching from database {}", err),
            ),
            ApiError::RecordNotFound => (
                StatusCode::NOT_FOUND,
                String::from("Could not find entry in database"),
            ),
            ApiError::ConflictError(err) => (StatusCode::CONFLICT, format!("Conflict: {}", err)),
            ApiError::BadRequest(err) => (StatusCode::BAD_REQUEST, format!("Bad request: {}", err)),
            ApiError::UnknownError(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Unknown error {}", err),
            ),
        };
        (status_code, Json(ErrorMessage { message })).into_response()
    }
}

impl From<sea_orm::DbErr> for ApiError {
    fn from(error: sea_orm::DbErr) -> Self {
        ApiError::DatabaseError(error)
    }
}

pub enum NotationError {
    InvalidPoint,
    InvalidFormat,
    ParseError(ParseIntError),
}

impl fmt::Display for NotationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "An Error Occurred, Please Try Again!") // user-facing output
    }
}

impl From<ParseIntError> for NotationError {
    fn from(err: ParseIntError) -> Self {
        NotationError::ParseError(err)
    }
}

impl fmt::Debug for NotationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!()) // programmer-facing output
    }
}
