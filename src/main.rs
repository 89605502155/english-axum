struct Task {
    id: u32,
    title: String,
    done: bool,
}

fn main() {
    let mut task = Task {
        id: 1,
        title: "sd".to_string(),
        done: false,
    };
    print_title(&task);
}

fn print_title(task: &Task){
    println!("{} {} {}", task.id, task.title, task.done)
}
