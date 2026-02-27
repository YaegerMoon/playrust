use axum::{
    extract::ConnectInfo,
    routing::get,
    Json, Router,
};
use chrono::Local;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Serialize, Deserialize)]
struct Info {
    time: String,
    ip: String,
}

#[tokio::main]
async fn main() {
    // Build our application with a single route
    let app = Router::new().route("/", get(handler));

    // Run it with hyper on localhost:3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on http://{}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}

async fn handler(ConnectInfo(addr): ConnectInfo<SocketAddr>) -> Json<Info> {
    let now = Local::now();
    let info = Info {
        time: now.to_rfc3339(),
        ip: addr.ip().to_string(),
    };
    Json(info)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::to_bytes,
        http::{Request, StatusCode},
    };
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_handler() {
        let app = Router::new().route("/", get(handler));

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/")
                    .extension(ConnectInfo(SocketAddr::from(([127, 0, 0, 1], 1234))))
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let info: Info = serde_json::from_slice(&body).unwrap();

        assert_eq!(info.ip, "127.0.0.1");
        assert!(!info.time.is_empty());
    }
}
