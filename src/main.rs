use axum::{routing::{get, post}, Json, Router};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::env;
use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;

#[derive(Serialize, ToSchema)]
struct ApiResponse { 
    message: String, 
    status: String 
}

#[derive(Deserialize, ToSchema)]
struct AccessRequest { 
    email: String 
}

#[derive(OpenApi)]
#[openapi(
    paths(handle_request),
    components(schemas(ApiResponse, AccessRequest))
)]
struct ApiDoc;

#[tokio::main]
async fn main() {
    let port = env::var("PORT").unwrap_or_else(|_| "10000".to_string());
    let addr: SocketAddr = format!("0.0.0.0:{}", port).parse().unwrap();

    let app = Router::new()
        .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/", get(|| async { axum::response::Html(include_str!("index.html")) }))
        .route("/api/v1/request-access", post(handle_request));

    println!("--- API Server listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[utoipa::path(
    post,
    path = "/api/v1/request-access",
    request_body = AccessRequest,
    responses((status = 200, body = ApiResponse))
)]
async fn handle_request(Json(payload): Json<AccessRequest>) -> Json<ApiResponse> {
    println!("--- SaaS Subscription Lifecycle Triggered for: {}", payload.email);
    Json(ApiResponse {
        message: "SaaS Workspace initialized successfully.".to_string(),
        status: "success".to_string(),
    })
}
