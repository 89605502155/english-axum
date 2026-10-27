use crate::models::auth as dto;
use crate::models::error::{ApiError, api_error};
use crate::repositry::user as repositry;
use crate::state::SessionState;
use axum::{Json, http::StatusCode};
use jsonwebtoken::{self, EncodingKey, Header, encode};

pub async fn login(
    state: SessionState,
    email: String,
) -> Result<(StatusCode, Json<dto::LogInResponse>), ApiError> {
    let id = repositry::get_by_email(&state.db, &email).await?;
    match id {
        Some(id) => generate_token(&email, id),
        None => Err(api_error(StatusCode::NOT_FOUND, "hjhjhj")),
    }
}

fn generate_token(
    email: &str,
    id: i32,
) -> Result<(StatusCode, Json<dto::LogInResponse>), ApiError> {
    let claims = dto::Claims {
        sub: id.to_string(),
        exp: (chrono::Utc::now() + chrono::Duration::hours(1)).timestamp() as usize,
        email: email.to_string(),
    };

    let token = match encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret("serct".as_ref()),
    ) {
        Ok(tok) => tok,
        Err(e) => {
            eprintln!("Error generate token {e}");
            return Err(api_error(StatusCode::INTERNAL_SERVER_ERROR, "500 ERROR"));
        }
    };
    Ok((StatusCode::OK, Json(dto::LogInResponse { token })))
}
