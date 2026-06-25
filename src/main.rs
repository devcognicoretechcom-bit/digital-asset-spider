use axum::{routing::get, response::Html, Router};
use std::net::SocketAddr;
use std::env;

#[tokio::main]
async fn main() {
    let port = env::var("PORT").unwrap_or_else(|_| "10000".to_string());
    let addr_str = format!("0.0.0.0:{}", port);
    let addr: SocketAddr = addr_str.parse().expect("Invalid address");

    // Serve the professional landing page
    let app = Router::new().route("/", get(|| async {
        Html(include_str!("index.html"))
    }));

    println!("--- API Server listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
