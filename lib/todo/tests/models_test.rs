use todo::models::{Priority, Status, TodoItem};

#[cfg(test)]
mod models_test {

    use todo::models::TodoItemBuilder;

    use super::*;

    #[test]
    fn test_todo_items() {
        let mut todo_item1 = TodoItem::new("hello", Priority::Low, None);
        let mut todo_item2 = TodoItemBuilder::default()
            .content("world")
            .priority(Priority::High)
            .status(Status::Done)
            .build()
            .expect("Build Failure");

        assert_eq!(todo_item1.content, "hello".to_string());
        assert_eq!(todo_item2.content, "world".to_string());

        todo_item1.content = todo_item1.content.to_uppercase();
        todo_item2.content = todo_item2.content.replace("world", "world!");

        assert_eq!(todo_item1.content, "HELLO");
        assert_eq!(todo_item2.content, "world!")
    }
}
