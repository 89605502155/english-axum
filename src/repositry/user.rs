use axum::{http::StatusCode, Json};
use sqlx::{PgPool, postgres::PgQueryResult};
use crate::models::user::User;
use crate::repositry::error::db_error;
use crate::models::error::ApiError;


pub async fn users_list(pool: PgPool) -> Result<Json<Vec<User>>, ApiError> {
    sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(&pool)
        .await
        .map(Json)
        .map_err(db_error)
}


pub async fn create_user(
    pool: PgPool,
    name: String, email: String,
) -> Result<(StatusCode, Json<User>), ApiError> {
    sqlx::query_as::<_, User>("INSERT INTO users (name, email) VALUES ($1, $2) RETURNING *")
        .bind(name)
        .bind(email)
        .fetch_one(&pool)
        .await
        .map(|u| (StatusCode::CREATED, Json(u)))
        .map_err(db_error)
}


pub async fn delete_user(
    pool: PgPool,
    id: i32,
) -> Result<PgQueryResult, ApiError> {
    sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await
        .map_err(db_error)
}