//! Practice 14: Web Development with Axum
//! Demonstrates an HTTP REST API with routing, state extraction, and JSON handling.

use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct AppState {
    pub items: Arc<Mutex<Vec<String>>>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct HealthResponse {
    pub status: &'static str,
    pub item_count: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AddItemRequest {
    pub name: String,
}

pub fn app_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health_handler))
        .route("/items", get(get_items_handler).post(add_item_handler))
        .with_state(state)
}

async fn health_handler(State(state): State<AppState>) -> Json<HealthResponse> {
    let count = state.items.lock().unwrap().len();
    Json(HealthResponse {
        status: "ok",
        item_count: count,
    })
}

async fn get_items_handler(State(state): State<AppState>) -> Json<Vec<String>> {
    let items = state.items.lock().unwrap().clone();
    Json(items)
}

async fn add_item_handler(
    State(state): State<AppState>,
    Json(payload): Json<AddItemRequest>,
) -> (StatusCode, Json<String>) {
    state.items.lock().unwrap().push(payload.name.clone());
    (StatusCode::CREATED, Json(format!("Item '{}' added", payload.name)))
}

#[tokio::main]
async fn main() {
    println!("=== 14: Web Development with Axum ===");

    let state = AppState {
        items: Arc::new(Mutex::new(vec!["InitialItem".to_string()])),
    };

    let _app = app_router(state);
    println!("Router initialized with endpoints: /health, /items");
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::Request;
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_health_endpoint() {
        let state = AppState {
            items: Arc::new(Mutex::new(vec!["item1".to_string(), "item2".to_string()])),
        };
        let app = app_router(state);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
