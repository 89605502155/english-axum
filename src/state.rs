use crate::models::Task;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct AppState{
    pub tasks: Arc<Mutex<Vec<Task>>>,
    pub next_id: Arc<Mutex<u32>>
}

impl AppState {
    pub fn new() -> Self {
        let seed = vec![
            Task{id: 1, title: "ewf".to_string(), done: false},
            Task{id: 2, title: "esdfwf".to_string(), done: false},
            Task{id: 3, title: "ewytytf".to_string(), done: true}
        ];

        Self {
            tasks: Arc::new(Mutex::new(seed)),
            next_id: Arc::new(Mutex::new(4)),
        }
    }
}