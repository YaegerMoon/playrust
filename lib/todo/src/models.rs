use chrono::{Local, NaiveDate, NaiveDateTime, Utc};
use derive_builder::Builder;

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

#[derive(Builder, Debug)]
#[builder(setter(into))] // 1. &str을 넣으면 자동으로 String으로 변환해줌
pub struct TodoItem {
    pub content: String,

    #[builder(default = "Priority::Medium")] // 2. 값이 없으면 Medium으로 설정
    pub priority: Priority,

    #[builder(default)] // 3. Option 필드는 기본적으로 None이 됨
    pub deadline: Option<NaiveDate>,

    #[builder(default = "Utc::now().naive_utc()")] // 4. 생성 시점 자동 기록
    pub created_at: NaiveDateTime,

    #[builder(default = "Status::Todo")] // 5. 기본 상태는 Todo
    pub status: Status,
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
}
