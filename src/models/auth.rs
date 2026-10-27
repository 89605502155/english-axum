use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Данные для создания пользователя
#[derive(Deserialize, ToSchema)]
#[schema(example = json!({"email": "ivan@example.com"}))]
pub struct LogIn {
    /// Email, должен быть уникальным
    #[schema(example = "ivan@example.com", format = "email")]
    pub email: String,
}

/// Данные для создания пользователя
#[derive(Serialize, ToSchema)]
#[schema(example = json!({"token": "djishjvjk.diosfsd.fdguifduhi"}))]
pub struct LogInResponse {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub id: i32,
    pub email: String,
    pub exp: usize,
}
