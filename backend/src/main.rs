use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post, put, delete},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::net::SocketAddr;
use tower_http::cors::{CorsLayer, Any};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct User {
    id: u32,
    name: String,
    email: String,
}

#[derive(Debug, Deserialize)]
struct CreateUserRequest {
    name: String,
    email: String,
}

#[derive(Debug, Deserialize)]
struct UpdateUserRequest {
    name: String,
    email: String,
}

#[derive(Clone)]
struct AppState {
    users: Arc<RwLock<HashMap<u32, User>>>,
    next_id: Arc<RwLock<u32>>,
}

async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> impl IntoResponse {
    let id = {
        let mut next_id = state.next_id.write().unwrap();
        let id = *next_id;
        *next_id += 1;
        id
    };

    let user = User {
        id,
        name: payload.name,
        email: payload.email,
    };

    {
        let mut users = state.users.write().unwrap();
        users.insert(id, user.clone());
    }

    (StatusCode::CREATED, Json(user))
}

async fn get_users(State(state): State<AppState>) -> impl IntoResponse {
    let users = state.users.read().unwrap();
    let users_list: Vec<User> = users.values().cloned().collect();
    Json(users_list)
}

async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<u32>,
) -> impl IntoResponse {
    let users = state.users.read().unwrap();
    match users.get(&id) {
        Some(user) => (StatusCode::OK, Json(user.clone())).into_response(),
        None => (StatusCode::NOT_FOUND, Json(serde_json::json!({"error": "User not found"}))).into_response(),
    }
}

async fn update_user(
    State(state): State<AppState>,
    Path(id): Path<u32>,
    Json(payload): Json<UpdateUserRequest>,
) -> impl IntoResponse {
    let mut users = state.users.write().unwrap();
    
    if let Some(user) = users.get_mut(&id) {
        user.name = payload.name;
        user.email = payload.email;
        (StatusCode::OK, Json(user.clone())).into_response()
    } else {
        (StatusCode::NOT_FOUND, Json(serde_json::json!({"error": "User not found"}))).into_response()
    }
}

async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<u32>,
) -> impl IntoResponse {
    let mut users = state.users.write().unwrap();
    
    if users.remove(&id).is_some() {
        StatusCode::NO_CONTENT.into_response()
    } else {
        (StatusCode::NOT_FOUND, Json(serde_json::json!({"error": "User not found"}))).into_response()
    }
}

async fn health_check() -> impl IntoResponse {
    Json(serde_json::json!({"status": "ok"}))
}

#[tokio::main]
async fn main() {
    let state = AppState {
        users: Arc::new(RwLock::new(HashMap::new())),
        next_id: Arc::new(RwLock::new(1)),
    };

    {
        let mut users = state.users.write().unwrap();
        let mut next_id = state.next_id.write().unwrap();
        
        let user1 = User {
            id: *next_id,
            name: "Alice Johnson".to_string(),
            email: "alice@example.com".to_string(),
        };
        users.insert(*next_id, user1);
        *next_id += 1;
        
        let user2 = User {
            id: *next_id,
            name: "Bob Smith".to_string(),
            email: "bob@example.com".to_string(),
        };
        users.insert(*next_id, user2);
        *next_id += 1;
    }

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/users", get(get_users).post(create_user))
        .route("/users/:id", get(get_user).put(update_user).delete(delete_user))
        .layer(CorsLayer::permissive())  // 开发环境使用宽松的 CORS
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Server running on http://{}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
