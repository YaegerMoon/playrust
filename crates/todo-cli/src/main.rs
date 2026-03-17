use chrono::Local;

use todo::models::{Priority, Status, TodoItem};

fn main() {
    let todo = TodoItem {
        content: "Hello world".to_string(),
        priority: Priority::High,
        deadline: None,
        created_at: Local::now().naive_local(),
        status: Status::InProgress,
    };

    println!("{:?}", todo);
}
