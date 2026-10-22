use serde::Serialize;
use utoipa::ToSchema;
use axum::{ http::StatusCode, Json};

/// Тело ответа при ошибке
#[derive(Serialize, ToSchema)]
#[schema(example = json!({"error": "user not found"}))]
pub struct ErrorResponse {
    /// Описание ошибки
    #[schema(example = "user not found")]
    pub error: String,
}

pub type ApiError = (StatusCode, Json<ErrorResponse>);

pub fn api_error(status: StatusCode, msg: &str) -> ApiError {
    (
        status,
        Json(ErrorResponse {
            error: msg.to_string(),
        }),
    )
}