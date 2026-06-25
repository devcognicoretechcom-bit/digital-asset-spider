use axum::{routing::{get, post}, Json, Router, extract::Form};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::env;

#[derive(Serialize)]
struct ApiResponse { message: String, status: String }

#[derive(Deserialize)]
struct AccessRequest { email: String }

#[tokio::main]
async fn main() {
    let port = env::var("PORT").unwrap_or_else(|_| "10000".to_string());
    let addr: SocketAddr = format!("0.0.0.0:{}", port).parse().unwrap();

    let app = Router::new()
        .route("/", get(|| async { axum::response::Html(include_str!("index.html")) }))
        .route("/api/v1/request-access", post(handle_request));

    println!("--- API Server listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handle_request(Json(payload): Json<AccessRequest>) -> Json<ApiResponse> {
    println!("--- New Enterprise Lead: {}", payload.email);
    Json(ApiResponse {
        message: "Request received. Enterprise team notified.".to_string(),
        status: "success".to_string(),
    })
}
