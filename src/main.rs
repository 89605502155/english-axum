mod handlers;
mod models;
mod repositry;
mod services;
mod routes;
mod state;

use axum;
use sqlx::postgres::PgPoolOptions;
use std::env;
use crate::state::SessionState;



#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let db_url = env::var("DB_URL").expect("DB_URL must be set");
    let pool = PgPoolOptions::new()
        .connect(&db_url)
        .await
        .expect("failed to connect to database");
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("failed to run migrations");

    let state = SessionState::new(pool);
    let app = routes::app(state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3003").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

