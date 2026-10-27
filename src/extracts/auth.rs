use crate::models::auth as dto;
use crate::state::JwtConfig;
use axum::extract::FromRef;
use axum::{
    extract::FromRequestParts,
    http::{StatusCode, header::AUTHORIZATION, request::Parts},
};
use jsonwebtoken::{DecodingKey, Validation, decode};

pub struct AuthUser(pub i32);

impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
    JwtConfig: FromRef<S>,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        println!("{:#?}", parts);
        let token = parts
            .headers
            .get(AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .and_then(|value| value.strip_prefix("Bearer "))
            .ok_or((
                StatusCode::UNAUTHORIZED,
                "Missing or invalid Authorization header",
            ))?;

        let jwt = JwtConfig::from_ref(state);
        println!("token {token}");
        println!("secet {}", jwt.secret);

        let parts: Vec<&str> = token.split('.').collect();

        eprintln!("JWT parts count: {}", parts.len());

        if parts.len() == 3 {
            use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};

            match URL_SAFE_NO_PAD.decode(parts[1]) {
                Ok(payload) => {
                    eprintln!("JWT payload: {}", String::from_utf8_lossy(&payload));
                }
                Err(e) => {
                    eprintln!("JWT payload decode error: {e}");
                }
            }
        };
        let token_data = decode::<dto::Claims>(
            token,
            &DecodingKey::from_secret(jwt.secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|e| {
            println!("JWT decode error: {e:?}");
            eprintln!("JWT decode error: {e:?}");

            (StatusCode::UNAUTHORIZED, "Invalid or expired token")
        })?;

        let user_id = token_data.claims.id;

        Ok(AuthUser(user_id))
    }
}
