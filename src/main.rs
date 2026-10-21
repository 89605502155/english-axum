mod handlers;
mod models;
mod routes;
mod state;

use models::Task;
use state::AppState;



#[tokio::main]
async fn main() {
    let state = AppState::new();
    let app = routes::app(state);
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


async fn print_title(task: &mut Task){
    task.id=3;
    println!("{} {} {}", task.id, task.title, task.done)
}
