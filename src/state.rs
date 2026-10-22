use std::sync::{ Arc, Mutex };
use sqlx::PgPool;

#[derive(Clone)]
pub struct SessionState {
    pub is_start: Arc<Mutex<bool>>,
    pub db: PgPool,
}

impl SessionState {
    pub fn new(pool: PgPool) -> Self {
        Self {
            is_start: Arc::new(Mutex::new(false)),
            db: pool,
        }
    }
}