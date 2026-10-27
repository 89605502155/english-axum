use crate::models::auth as dto;
use crate::models::error;
use crate::models::error::ApiError;
use crate::services::auth;
use crate::state::SessionState;
use axum::{Json, extract::State, http::StatusCode};

/// Получить jwt
#[utoipa::path(
    post,
    path = "/auth",
    tag = "Auth",
    request_body(
        content = dto::LogIn,
        description = "email пользователя",
        content_type = "application/json",
        examples(
            ("Иван" = (summary = "Пример пользователя",
                value = json!({"email": "ivan@example.com"})))
        )
    ),
    responses(
        (status = 200, description = "Авторизация", body = dto::LogInResponse,
            examples(
                ("Ok" = (
                    summary = "Обычный ответ",
                    value = json!({
                        "token": "sdfsdfsd.asdfdsaf.asdfasdf"
                    })
                ))
            )
        ),
        (status = 500, description = "Ошибка базы данных", body = error::ErrorResponse,
            example = json!({"error": "internal server error"}))
    )
)]
pub async fn loging_fn(
    State(state): State<SessionState>,
    Json(dto): Json<dto::LogIn>,
) -> Result<(StatusCode, Json<dto::LogInResponse>), ApiError> {
    auth::login(state, dto.email).await
}
