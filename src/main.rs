use axum::{routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use log::{info};
use tower_http::cors::{Any, CorsLayer};

#[derive(Deserialize)]
struct ScanRequest {
    url: String,
}

#[derive(Serialize)]
struct ScanResponse {
    url: String,
    status: String,
}

#[tokio::main]
async fn main() {
    env_logger::init();
    
    // Configure CORS to allow your browser to talk to the API
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/scan", post(scan_handler))
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("--- API Server listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn scan_handler(Json(payload): Json<ScanRequest>) -> Json<ScanResponse> {
    info!("Received scan request for: {}", payload.url);
    let status = check_asset(&payload.url).await;
    Json(ScanResponse {
        url: payload.url,
        status,
    })
}

async fn check_asset(url: &str) -> String {
    match reqwest::get(url).await {
        Ok(res) => res.status().to_string(),
        Err(_) => "Error/Down".to_string(),
    }
}
