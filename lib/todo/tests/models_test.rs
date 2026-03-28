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

    #[test]
    fn test_todo_item_id() {
        use uuid::Uuid;
        let todo = TodoItemBuilder::default()
            .content("Check ID")
            .build()
            .expect("Build Failure");

        // The id should be a Uuid
        let id: Uuid = todo.id;
        assert!(!id.is_nil());
    }

    #[test]
    fn test_todo_item_serialization() {
        let todo = TodoItemBuilder::default()
            .content("Serialize me")
            .priority(Priority::Medium)
            .status(Status::InProgress)
            .build()
            .expect("Build Failure");

        // Serialize to JSON
        let json = serde_json::to_string(&todo).expect("Serialization failed");
        println!("Serialized: {}", json);

        // Deserialize back to TodoItem
        let deserialized: TodoItem = serde_json::from_str(&json).expect("Deserialization failed");

        assert_eq!(todo.content, deserialized.content);
        assert_eq!(todo.status, deserialized.status);
    }
}
