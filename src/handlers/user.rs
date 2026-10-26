use crate::models::error;
use crate::models::user as user_model;
use crate::services::user;
use crate::state::SessionState;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

/// Получить список всех пользователей
#[utoipa::path(
    get,
    path = "/users",
    tag = "Users",
    responses(
        (status = 200, description = "Список пользователей", body = [user_model::User],
            examples(
                ("Несколько пользователей" = (
                    summary = "Обычный ответ",
                    value = json!([
                        {"id": 1, "name": "Ivan Petrov", "email": "ivan@example.com"},
                        {"id": 2, "name": "Anna Smirnova", "email": "anna@example.com"}
                    ])
                )),
                ("Пусто" = (summary = "Пользователей нет", value = json!([])))
            )
        ),
        (status = 500, description = "Ошибка базы данных", body = error::ErrorResponse,
            example = json!({"error": "internal server error"}))
    )
)]
pub async fn users_list(
    State(state): State<SessionState>,
) -> Result<Json<Vec<user_model::User>>, error::ApiError> {
    user::users_list(state).await
}

/// Создать пользователя
#[utoipa::path(
    post,
    path = "/users",
    tag = "Users",
    request_body(
        content = user_model::UserPayLoad,
        description = "Имя и email нового пользователя",
        content_type = "application/json",
        examples(
            ("Иван" = (summary = "Пример пользователя",
                value = json!({"name": "Ivan Petrov", "email": "ivan@example.com"}))),
            ("Анна" = (summary = "Другой пример",
                value = json!({"name": "Anna Smirnova", "email": "anna@example.com"})))
        )
    ),
    responses(
        (status = 201, description = "Пользователь создан", body = user_model::User,
            example = json!({"id": 1, "name": "Ivan Petrov", "email": "ivan@example.com"})),
        (status = 409, description = "Email уже занят", body = error::ErrorResponse,
            example = json!({"error": "user with this email already exists"})),
        (status = 422, description = "Невалидное тело запроса (нет поля или неверный тип)",
            body = String, content_type = "text/plain",
            example = "Failed to deserialize the JSON body into the target type: missing field `email`"),
        (status = 500, description = "Ошибка базы данных", body = error::ErrorResponse,
            example = json!({"error": "internal server error"}))
    )
)]
pub async fn create_user(
    State(state): State<SessionState>,
    Json(payload): Json<user_model::UserPayLoad>,
) -> Result<(StatusCode, Json<user_model::User>), error::ApiError> {
    user::create_user(state, payload).await
}

/// Удалить пользователя по id
#[utoipa::path(
    delete,
    path = "/users/{id}",
    tag = "Users",
    security(
            ("bearer_auth" = [])
        ),
    params(
        ("id" = i32, Path, description = "Идентификатор пользователя", example = 1)
    ),
    responses(
        (status = 204, description = "Пользователь удалён"),
        (status = 404, description = "Пользователь не найден", body = error::ErrorResponse,
            example = json!({"error": "user not found"})),
        (status = 500, description = "Ошибка базы данных", body = error::ErrorResponse,
            example = json!({"error": "internal server error"}))
    )
)]
pub async fn delete_user(
    State(state): State<SessionState>,
    Path(id): Path<i32>,
) -> Result<StatusCode, error::ApiError> {
    user::delete_user(state, id).await
}
