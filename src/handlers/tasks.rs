use axum::{extract::{Path, Query, State}, Json, http::StatusCode};
use crate::models::{Task, CreateTask};
use std::collections::HashMap;
use crate::state::AppState;

pub async fn create_t(State(state): State<AppState>, Json(payload): Json<CreateTask>) -> (StatusCode, Json<Task>){
    let mut tasks =  state.tasks.lock().unwrap();
    let mut next_n = state.next_id.lock().unwrap();
    let t: Task = Task {
        id: *next_n,
        title: payload.title,
        done: false
    };
    *next_n += 1;
    (&mut tasks).push(t.clone());

    (StatusCode::CREATED, Json(t))

}

pub async fn root(State(state): State<AppState>) -> &'static str {
    "sdfsdfs"
}
pub async fn list_t(State(state): State<AppState>)->Json<Vec<Task>>{
    Json(state.tasks.lock().unwrap().clone())
}

pub async fn path_f(State(state): State<AppState>, Path(id): Path<u32>) -> Result<Json<Task>, StatusCode> {
    let t = state.tasks.lock().unwrap();
    t.iter().find(|t| t.id == id).map(|t| Json(t.clone())).ok_or(StatusCode::NOT_FOUND)
}
pub async fn qwerty(State(state): State<AppState>, Query(params): Query<HashMap<String, String>>) -> String {
    let filter = params.get("done").map_or("ever".to_string(),|v| format!("done={}",v));
    format!("listing don {filter}")
}

pub async fn print_title(State(state): State<AppState>, task: &mut Task){
    task.id=3;
    println!("{} {} {}", task.id, task.title, task.done)
}