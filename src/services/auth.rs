use crate::repositry::user as repositry;
use axum::{http::StatusCode, Json};
use crate::models::auth as dto;
use crate::state::SessionState;

pub async fn login(
    state: SessionState,
    email: String,
){
    let id = repositry::get_by_email(&state.db, &email).await?;
    match id {
        Some(id) => generate_token(&email, id),
        None => Ok((StatusCode::NOT_FOUND, "User not found".to_string())),
    }
}

fn generate_token(email: &str, id: i32){
    let claims = dto::Claims{
        sub: id.to_string(),
        exp: (chrono::Utc::now()+chrono::Duration::hours(1)).timestamp() as usize,
        email: email.to_string(),
    };

    let token = match encode(&Header::default(), &claims, &EncodingKey::from_secret("serct".as_ref())){
        Ok(tok) => tok,
        Err(e)=>{
            eprintln!("Error generate token {e}");
            return Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    };
    Ok(Json(dto::LogInResponse{token}))
}