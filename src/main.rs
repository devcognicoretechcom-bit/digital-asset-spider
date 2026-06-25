use axum::{routing::{get, post}, Json, Router};
use serde::{Deserialize, Serialize};
use std::{fs::{OpenOptions, read_to_string}, io::Write, net::SocketAddr};

#[derive(Serialize, Deserialize, Clone)]
struct Client { email: String, plan: String }

#[tokio::main]
async fn main() {
    let port = std::env::var("PORT").unwrap_or_else(|_| "10000".to_string());
    let addr: SocketAddr = format!("0.0.0.0:{}", port).parse().unwrap();

    let app = Router::new()
        .route("/", get(|| async { axum::response::Html(include_str!("index.html")) }))
        .route("/api/register", post(register_client));

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn register_client(Json(client): Json<Client>) -> String {
    let data = format!("{},{}\n", client.email, client.plan);
    let mut file = OpenOptions::new().append(true).create(true).open("clients.csv").unwrap();
    file.write_all(data.as_bytes()).unwrap();
    format!("Success: Registered {} for {} plan.", client.email, client.plan)
}
