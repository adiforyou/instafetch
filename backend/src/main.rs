use axum::{routing::get,Router, Json};
use std::net::SocketAddr;
use serde_json::json;
use tokio;
use axum::http::StatusCode;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/",get(root_handler));
    let addr= SocketAddr::from(([127,0,0,1],3000));
    println!("Server running on http://{}",addr);
    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(),app)
        .await
        .unwrap();
}
async fn root_handler() -> Json<serde_json::Value>{
    Json(json!({"message": "Backend Working"}))
}