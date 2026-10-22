use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;


/// Данные для создания пользователя
#[derive(Deserialize, ToSchema)]
#[schema(example = json!({"name": "Ivan Petrov", "email": "ivan@example.com"}))]
pub struct UserPayLoad {
    /// Имя пользователя
    #[schema(example = "Ivan Petrov", min_length = 1)]
    pub name: String,
    /// Email, должен быть уникальным
    #[schema(example = "ivan@example.com", format = "email")]
    pub email: String,
}

/// Пользователь
#[derive(Serialize, FromRow, ToSchema)]
#[schema(example = json!({"id": 1, "name": "Ivan Petrov", "email": "ivan@example.com"}))]
pub struct User {
    /// Уникальный идентификатор
    #[schema(example = 1)]
    pub id: i32,
    /// Имя пользователя
    #[schema(example = "Ivan Petrov")]
    pub name: String,
    /// Email пользователя
    #[schema(example = "ivan@example.com", format = "email")]
    pub email: String,
}