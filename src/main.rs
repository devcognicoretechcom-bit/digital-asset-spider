use axum::{routing::{get, post}, response::{Html, IntoResponse}, Router, extract::Form};
use serde::Deserialize;
use std::net::SocketAddr;
use std::env;

#[derive(Deserialize)]
struct AccessRequest { email: String }

#[tokio::main]
async fn main() {
    let port = env::var("PORT").unwrap_or_else(|_| "10000".to_string());
    let addr: SocketAddr = format!("0.0.0.0:{}", port).parse().unwrap();

    let app = Router::new()
        .route("/", get(|| async { Html(include_str!("index.html")) }))
        .route("/request-access", post(handle_request));

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handle_request(Form(input): Form<AccessRequest>) -> impl IntoResponse {
    println!("--- New Lead Received: {}", input.email);
    "Thank you. Our enterprise team will contact you shortly."
}
