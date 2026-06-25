use axum::{routing::{get, post}, Json, Router, extract::State};
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, sync::Arc};
use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;

#[derive(Clone)]
struct AppState {
    // In production, this would be a Database connection
    tier: String,
}

#[derive(Serialize, ToSchema)]
struct ApiResponse { message: String, status: String }

#[derive(Deserialize, ToSchema)]
struct AccessRequest { email: String, plan: String }

#[derive(OpenApi)]
#[openapi(paths(handle_request), components(schemas(ApiResponse, AccessRequest)))]
struct ApiDoc;

#[tokio::main]
async fn main() {
    let port = std::env::var("PORT").unwrap_or_else(|_| "10000".to_string());
    let addr: SocketAddr = format!("0.0.0.0:{}", port).parse().unwrap();
    let shared_state = Arc::new(AppState { tier: "free".to_string() });

    let app = Router::new()
        .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/", get(|| async { axum::response::Html(include_str!("index.html")) }))
        .route("/api/v1/subscribe", post(handle_request))
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[utoipa::path(post, path = "/api/v1/subscribe", request_body = AccessRequest, responses((status = 200, body = ApiResponse)))]
async fn handle_request(State(state): State<Arc<AppState>>, Json(payload): Json<AccessRequest>) -> Json<ApiResponse> {
    let msg = match payload.plan.as_str() {
        "free" => "Free tier: 100 requests/mo limit applied.".to_string(),
        "pro" => "Pro tier: 5,000 requests/mo limit applied.".to_string(),
        "premium" => "Premium tier: Unlimited access & dedicated support.".to_string(),
        _ => "Invalid plan.".to_string(),
    };
    
    Json(ApiResponse { message: msg, status: "success".to_string() })
}
