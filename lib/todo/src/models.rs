use chrono::{Local, NaiveDate, NaiveDateTime};

#[derive(Debug, Clone)] // 터미널에 출력할 수 있게 해주는 마법의 문장
pub enum Status {
    Todo,
    InProgress,
    Done,
}

#[derive(Debug, Clone)]
pub enum Priority {
    Critical,
    High,
    Medium,
    Low,
    Optional,
}

#[derive(Debug, Clone)]
pub struct TodoItem {
    pub content: String,
    pub priority: Priority,
    pub deadline: Option<NaiveDate>,
    pub created_at: NaiveDateTime,
    pub status: Status,
}

impl Default for TodoItem {
    fn default() -> Self {
        Self {
            content: String::from("Default"),
            priority: Priority::Optional,
            deadline: Some(Local::now().date_naive()),
            created_at: Local::now().naive_local(),
            status: Status::Todo,
        }
    }
}

impl TodoItem {
    pub fn new(content: &str, priority: Priority, deadline: Option<NaiveDate>) -> Self {
        Self {
            content: content.to_string(),
            priority: priority,
            deadline: deadline,
            created_at: Local::now().naive_local(),
            status: Status::Todo,
        }
    }

    pub fn update_status(&mut self, status: Status) {
        self.status = status;
    }

    pub fn update_priority(&mut self, priority: Priority) {
        self.priority = priority;
    }

    pub fn update_deadline(&mut self, deadline: Option<NaiveDate>) {
        self.deadline = deadline;
    }

    pub fn update_content(&mut self, content: &str) {
        self.content = content.to_string();
    }
}
