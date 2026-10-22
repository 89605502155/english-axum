use crate::models::error::{ApiError, api_error};
use sqlx::Error;
use axum::http::StatusCode;

pub fn db_error(e: Error) -> ApiError {
    if let Error::Database(db) = &e {
        if db.is_unique_violation() {
            return api_error(StatusCode::CONFLICT, "user with this email already exists");
        }
    }
    api_error(StatusCode::INTERNAL_SERVER_ERROR, "internal server error")
}