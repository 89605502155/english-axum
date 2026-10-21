use axum::{routing::{get, post}, Router, extract::{Path, Query}, http::StatusCode, Json};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Clone)]
struct Task {
    id: u32,
    title: String,
    done: bool,
}

fn seed_tasks()-> Vec<Task> {
    vec![
        Task{id: 1, title: "ewf".to_string(), done: false},
        Task{id: 2, title: "esdfwf".to_string(), done: false},
        Task{id: 3, title: "ewytytf".to_string(), done: true}
    ]
}

#[derive(Deserialize)]
struct CreateTask {
    title: String,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/g/{id}", get(path_f))
        .route("/h",get(qwerty).post(create_t))
        .route("/l", get(list_t));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3004").await.unwrap();

    let mut task = Task {
        id: 1,
        title: "sd".to_string(),
        done: false,
    };
    print_title(&mut task).await;
    println!("{}", task.id);
    let may_be_title: Option<String>;
    if 2==2{
        may_be_title=None
    } else {
        may_be_title = Some("title".to_string());
    }
    println!("{:#?} d", may_be_title);
    match may_be_title {
        Some(t) => println!("{:#?} ddfds", t),
        None => println!("N dfs")
    }
    axum::serve(listener, app).await.unwrap();
}

async fn create_t(Json(payload): Json<CreateTask>) -> (StatusCode, Json<Task>){
    let t: Task = Task {
        id: 11,
        title: payload.title,
        done: false
    };

    (StatusCode::CREATED, Json(t))

}

async fn root() -> &'static str {
    "sdfsdfs"
}
async fn list_t()->Json<Vec<Task>>{
    Json(seed_tasks())
}

async fn path_f(Path(id): Path<u32>) -> Result<Json<Task>, StatusCode> {
    let t = seed_tasks();
    t.into_iter().find(|t| t.id == id).map(Json).ok_or(StatusCode::NOT_FOUND)
}
async fn qwerty(Query(params): Query<HashMap<String, String>>) -> String {
    let filter = params.get("done").map_or("ever".to_string(),|v| format!("done={}",v));
    format!("listing don {filter}")
}

async fn print_title(task: &mut Task){
    task.id=3;
    println!("{} {} {}", task.id, task.title, task.done)
}
