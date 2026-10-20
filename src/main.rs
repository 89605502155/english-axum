use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::{delete, get},
};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool, postgres::PgPoolOptions};
use std::env;
use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;

/// Данные для создания пользователя
#[derive(Deserialize, ToSchema)]
#[schema(example = json!({"name": "Ivan Petrov", "email": "ivan@example.com"}))]
struct UserPayLoad {
    /// Имя пользователя
    #[schema(example = "Ivan Petrov", min_length = 1)]
    name: String,
    /// Email, должен быть уникальным
    #[schema(example = "ivan@example.com", format = "email")]
    email: String,
}

/// Пользователь
#[derive(Serialize, FromRow, ToSchema)]
#[schema(example = json!({"id": 1, "name": "Ivan Petrov", "email": "ivan@example.com"}))]
struct User {
    /// Уникальный идентификатор
    #[schema(example = 1)]
    id: i32,
    /// Имя пользователя
    #[schema(example = "Ivan Petrov")]
    name: String,
    /// Email пользователя
    #[schema(example = "ivan@example.com", format = "email")]
    email: String,
}

/// Тело ответа при ошибке
#[derive(Serialize, ToSchema)]
#[schema(example = json!({"error": "user not found"}))]
struct ErrorResponse {
    /// Описание ошибки
    #[schema(example = "user not found")]
    error: String,
}

type ApiError = (StatusCode, Json<ErrorResponse>);

fn api_error(status: StatusCode, msg: &str) -> ApiError {
    (
        status,
        Json(ErrorResponse {
            error: msg.to_string(),
        }),
    )
}

fn db_error(e: sqlx::Error) -> ApiError {
    if let sqlx::Error::Database(db) = &e {
        if db.is_unique_violation() {
            return api_error(StatusCode::CONFLICT, "user with this email already exists");
        }
    }
    api_error(StatusCode::INTERNAL_SERVER_ERROR, "internal server error")
}

#[derive(OpenApi)]
#[openapi(
    info(
        title = "English API",
        version = "0.1.0",
        description = "API для управления пользователями"
    ),
    paths(root, users_list, create_user, delete_user),
    components(schemas(User, UserPayLoad, ErrorResponse)),
    tags(
        (name = "Start", description = "Проверка доступности сервиса"),
        (name = "Users", description = "CRUD операции над пользователями")
    )
)]
struct ApiDoc;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let db_url = env::var("DB_URL").expect("DB_URL must be set");
    let pool = PgPoolOptions::new()
        .connect(&db_url)
        .await
        .expect("failed to connect to database");
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("failed to run migrations");

    let app = Router::new()
        .route("/", get(root))
        .route("/users", get(users_list).post(create_user))
        .route("/users/{id}", delete(delete_user))
        .merge(SwaggerUi::new("/api").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .with_state(pool);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3003").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

/// Проверка, что сервис жив
#[utoipa::path(
    get,
    path = "/",
    tag = "Start",
    responses(
        (status = 200, description = "Сервис работает", body = String, content_type = "text/plain", example = "Hello")
    )
)]
async fn root() -> &'static str {
    "Hello"
}

/// Получить список всех пользователей
#[utoipa::path(
    get,
    path = "/users",
    tag = "Users",
    responses(
        (status = 200, description = "Список пользователей", body = [User],
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
        (status = 500, description = "Ошибка базы данных", body = ErrorResponse,
            example = json!({"error": "internal server error"}))
    )
)]
async fn users_list(State(pool): State<PgPool>) -> Result<Json<Vec<User>>, ApiError> {
    sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(&pool)
        .await
        .map(Json)
        .map_err(db_error)
}

/// Создать пользователя
#[utoipa::path(
    post,
    path = "/users",
    tag = "Users",
    request_body(
        content = UserPayLoad,
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
        (status = 201, description = "Пользователь создан", body = User,
            example = json!({"id": 1, "name": "Ivan Petrov", "email": "ivan@example.com"})),
        (status = 409, description = "Email уже занят", body = ErrorResponse,
            example = json!({"error": "user with this email already exists"})),
        (status = 422, description = "Невалидное тело запроса (нет поля или неверный тип)",
            body = String, content_type = "text/plain",
            example = "Failed to deserialize the JSON body into the target type: missing field `email`"),
        (status = 500, description = "Ошибка базы данных", body = ErrorResponse,
            example = json!({"error": "internal server error"}))
    )
)]
async fn create_user(
    State(pool): State<PgPool>,
    Json(payload): Json<UserPayLoad>,
) -> Result<(StatusCode, Json<User>), ApiError> {
    sqlx::query_as::<_, User>("INSERT INTO users (name, email) VALUES ($1, $2) RETURNING *")
        .bind(payload.name)
        .bind(payload.email)
        .fetch_one(&pool)
        .await
        .map(|u| (StatusCode::CREATED, Json(u)))
        .map_err(db_error)
}

/// Удалить пользователя по id
#[utoipa::path(
    delete,
    path = "/users/{id}",
    tag = "Users",
    params(
        ("id" = i32, Path, description = "Идентификатор пользователя", example = 1)
    ),
    responses(
        (status = 204, description = "Пользователь удалён"),
        (status = 404, description = "Пользователь не найден", body = ErrorResponse,
            example = json!({"error": "user not found"})),
        (status = 500, description = "Ошибка базы данных", body = ErrorResponse,
            example = json!({"error": "internal server error"}))
    )
)]
async fn delete_user(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<StatusCode, ApiError> {
    let result = sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await
        .map_err(db_error)?;
    if result.rows_affected() == 0 {
        Err(api_error(StatusCode::NOT_FOUND, "user not found"))
    } else {
        Ok(StatusCode::NO_CONTENT)
    }
}
