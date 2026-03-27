use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "todo-cli", about = "간단한 할 일 관리 툴", version = "1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 할 일 추가
    Add { task: String },
    /// 전체 목록 보기
    List,
    /// 할 일 삭제 (인덱스 번호)
    Remove { index: usize },
    /// 할 일 완료 상태 변경 (인덱스 번호)
    Update { index: usize, done: bool },
}
