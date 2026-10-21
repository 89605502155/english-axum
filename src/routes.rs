use axum::{routing::get, Router};
use crate::handlers::tasks;
use crate::state::AppState;

pub fn app(state: AppState) -> Router {
    Router::new()
        .route("/", get(tasks::root))
        .route("/g/{id}", get(tasks::path_f))
        .route("/h",get(tasks::qwerty).post(tasks::create_t))
        .route("/l", get(tasks::list_t))
        .with_state(state)
}