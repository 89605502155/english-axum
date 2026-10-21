use axum::{routing::{get, post}, Router, extract::{Path, Query}};
use std::collections::HashMap;

struct Task {
    id: u32,
    title: String,
    done: bool,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/g/{id}", get(path_f))
        .route("/h",get(qwerty));
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

async fn root() -> &'static str {
    "sdfsdfs"
}

async fn path_f(Path(id): Path<u32>) {
    id+34;
}
async fn qwerty(Query(params): Query<HashMap<String, String>>) -> String {
    let filter = params.get("done").map_or("ever".to_string(),|v| format!("done={}",v));
    format!("listing don {filter}")
}

async fn print_title(task: &mut Task){
    task.id=3;
    println!("{} {} {}", task.id, task.title, task.done)
}
