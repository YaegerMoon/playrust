pub mod commands;

use todo::models::{Priority, TodoItem};

fn main() {
    let todo = TodoItem::new("Hello world", Priority::High, None);

    println!("{:?}", todo);
}
