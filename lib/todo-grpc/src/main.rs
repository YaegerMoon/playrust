use tonic::{Request, Response, Status, transport::Server};

pub mod todo_proto {
    tonic::include_proto!("todo");
}

use todo_proto::todo_service_server::{TodoService, TodoServiceServer};
use todo_proto::{TodoRequest, TodoResponse};

#[derive(Default)]
pub struct MyTodoService {}

#[tonic::async_trait]
impl TodoService for MyTodoService {
    async fn get_todo(
        &self,
        request: Request<TodoRequest>,
    ) -> Result<Response<TodoResponse>, Status> {
        let id = request.into_inner().id;

        let response = TodoResponse {
            id,
            content: "Learn gRPC".to_string(),
            priority: "High".to_string(),
            status: "Todo".to_string(),
            created_at: "2026-03-27T00:00:00Z".to_string(),
        };

        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let todo_service = MyTodoService::default();

    println!("Todo gRPC Server listening on {}", addr);

    Server::builder()
        .add_service(TodoServiceServer::new(todo_service))
        .serve(addr)
        .await?;

    Ok(())
}
