use axum::extract::FromRef;
use sqlx::PgPool;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct SessionState {
    pub is_start: Arc<Mutex<bool>>,
    pub db: PgPool,
    pub jwt_config: JwtConfig,
}

impl SessionState {
    pub fn new(pool: PgPool, secret: String, hourse: i64) -> Self {
        Self {
            is_start: Arc::new(Mutex::new(false)),
            db: pool,
            jwt_config: JwtConfig {
                secret: secret,
                ttl_hours: hourse,
            },
        }
    }
}

#[derive(Clone)]
pub struct JwtConfig {
    pub secret: String,
    pub ttl_hours: i64,
}

impl FromRef<SessionState> for JwtConfig {
    fn from_ref(state: &SessionState) -> Self {
        state.jwt_config.clone()
    }
}
