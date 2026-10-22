use crate::repositry::user;
use crate::models::user as model_user;
use axum::{http::StatusCode, Json};
use crate::models::error::{api_error, ApiError};
use crate::state::SessionState;

pub async fn users_list(state: SessionState) -> Result<Json<Vec<model_user::User>>, ApiError> {
    user::users_list(state.db).await
}


pub async fn create_user(
    state: SessionState,
    payload: model_user::UserPayLoad,
) -> Result<(StatusCode, Json<model_user::User>), ApiError> {
    user::create_user(state.db, payload.name, payload.email).await
}


pub async fn delete_user(
    state: SessionState,
    id: i32,
) -> Result<StatusCode, ApiError> {
    let result = user::delete_user(state.db, id).await?;
    if result.rows_affected() == 0 {
        Err(api_error(StatusCode::NOT_FOUND, "user not found"))
    } else {
        Ok(StatusCode::NO_CONTENT)
    }
}