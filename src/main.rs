mod extracts;
mod handlers;
mod models;
mod repositry;
mod routes;
mod services;
mod state;

use crate::state::SessionState;
use axum;
use sqlx::postgres::PgPoolOptions;
use std::env;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let db_url = env::var("DB_URL").expect("DB_URL must be set");
    let jwt_secret = env::var("JWT_SECRET").expect("DB_URL must be set");
    let jwt_hourse_str = env::var("JWT_MAXAGE").expect("DB_URL must be set");
    let jwt_hourse: i64 = jwt_hourse_str.parse().expect("msg");
    let pool = PgPoolOptions::new()
        .connect(&db_url)
        .await
        .expect("failed to connect to database");
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("failed to run migrations");

    let state = SessionState::new(pool, jwt_secret, jwt_hourse);
    let app = routes::app(state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3003").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
