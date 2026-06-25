use axum::{routing::get, Router};
use std::net::SocketAddr;
use std::env;

#[tokio::main]
async fn main() {
    // Read the PORT from the environment (defaulting to 10000 for Render)
    let port = env::var("PORT").unwrap_or_else(|_| "10000".to_string());
    
    // Bind to 0.0.0.0 to accept traffic from the public internet
    let addr_str = format!("0.0.0.0:{}", port);
    let addr: SocketAddr = addr_str.parse().expect("Invalid address configuration");

    let app = Router::new().route("/", get(|| async { "API Server is running" }));

    println!("--- API Server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
