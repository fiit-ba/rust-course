use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{delete, get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;
use tower_http::cors::CorsLayer;
use uuid::Uuid;

// Application state to store pastes in memory
type AppState = Arc<RwLock<HashMap<Uuid, String>>>;

// Request/Response types
#[derive(Serialize, Deserialize)]
struct CreatePasteRequest {
    content: String,
}

#[derive(Serialize, Deserialize)]
struct CreatePasteResponse {
    id: Uuid,
    message: String,
}

#[derive(Serialize, Deserialize)]
struct GetPasteResponse {
    id: Uuid,
    content: String,
}

#[derive(Serialize, Deserialize)]
struct DeletePasteResponse {
    message: String,
}

#[derive(Serialize, Deserialize)]
struct ErrorResponse {
    error: String,
}

#[derive(Debug)]
enum AppError {
    NotFound,
    InvalidInput(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::NotFound => (StatusCode::NOT_FOUND, "Paste not found".to_string()),
            AppError::InvalidInput(msg) => (StatusCode::BAD_REQUEST, msg),
        };

        let body = Json(ErrorResponse {
            error: error_message.to_string(),
        });

        (status, body).into_response()
    }
}

// Handler to create a new paste
async fn create_paste(
    State(state): State<AppState>,
    Json(payload): Json<CreatePasteRequest>,
) -> Result<impl IntoResponse, AppError> {
    if payload.content.trim().is_empty() {
        return Err(AppError::InvalidInput(
            "Content cannot be empty".to_string(),
        ));
    }

    let id = Uuid::new_v4();

    {
        let mut pastes = state.write().await;
        pastes.insert(id, payload.content);
    }

    let response = CreatePasteResponse {
        id,
        message: "Paste created successfully".to_string(),
    };

    Ok((StatusCode::CREATED, Json(response)))
}

async fn get_paste(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let pastes = state.read().await;

    match pastes.get(&id) {
        Some(content) => {
            let response = GetPasteResponse {
                id,
                content: content.clone(),
            };
            Ok(Json(response))
        }
        None => Err(AppError::NotFound),
    }
}

async fn delete_paste(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let mut pastes = state.write().await;

    match pastes.remove(&id) {
        Some(_) => {
            let response = DeletePasteResponse {
                message: "Paste deleted successfully".to_string(),
            };
            Ok(Json(response))
        }
        None => Err(AppError::NotFound),
    }
}

async fn list_pastes(State(state): State<AppState>) -> impl IntoResponse {
    let pastes = state.read().await;
    let paste_ids: Vec<Uuid> = pastes.keys().cloned().collect();
    Json(paste_ids)
}

async fn health_check() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "healthy",
        "service": "pastebin"
    }))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let state: AppState = Arc::new(RwLock::new(HashMap::new()));

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/pastes", post(create_paste))
        .route("/pastes", get(list_pastes))
        .route("/pastes/:id", get(get_paste))
        .route("/pastes/:id", delete(delete_paste))
        .layer(CorsLayer::permissive())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    println!("🚀 Pastebin server running on http://127.0.0.1:3000");
    println!("📋 Endpoints:");
    println!("  POST   /pastes     - Create a new paste");
    println!("  GET    /pastes/:id - Get a paste by ID");
    println!("  DELETE /pastes/:id - Delete a paste by ID");
    println!("  GET    /pastes     - List all paste IDs");
    println!("  GET    /health     - Health check");

    axum::serve(listener, app).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Method, Request, StatusCode},
    };
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_create_and_get_paste() {
        let state: AppState = Arc::new(RwLock::new(HashMap::new()));
        let app = Router::new()
            .route("/pastes", post(create_paste))
            .route("/pastes/:id", get(get_paste))
            .with_state(state);

        let create_request = Request::builder()
            .method(Method::POST)
            .uri("/pastes")
            .header("content-type", "application/json")
            .body(Body::from(r#"{"content": "Hello, World!"}"#))
            .unwrap();

        let response = app.clone().oneshot(create_request).await.unwrap();
        assert_eq!(response.status(), StatusCode::CREATED);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let create_response: CreatePasteResponse = serde_json::from_slice(&body).unwrap();

        let get_request = Request::builder()
            .method(Method::GET)
            .uri(&format!("/pastes/{}", create_response.id))
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(get_request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let get_response: GetPasteResponse = serde_json::from_slice(&body).unwrap();
        assert_eq!(get_response.content, "Hello, World!");
    }

    #[tokio::test]
    async fn test_get_nonexistent_paste() {
        let state: AppState = Arc::new(RwLock::new(HashMap::new()));
        let app = Router::new()
            .route("/pastes/:id", get(get_paste))
            .with_state(state);

        let request = Request::builder()
            .method(Method::GET)
            .uri(&format!("/pastes/{}", Uuid::new_v4()))
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
