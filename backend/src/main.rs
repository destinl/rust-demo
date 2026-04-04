use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::sync::{Arc, RwLock};
use tokio;
use tower_http::cors::{Any, CorsLayer};

// 定义用户结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
struct User {
    id: u32,
    name: String,
    email: String,
}

// 创建用户请求体
#[derive(Debug, Deserialize)]
struct CreateUserRequest {
    name: String,
    email: String,
}

// 更新用户请求体
#[derive(Debug, Deserialize)]
struct UpdateUserRequest {
    name: String,
    email: String,
}

// 应用状态
#[derive(Clone)]
struct AppState {
    users: Arc<RwLock<HashMap<u32, User>>>,
    next_id: Arc<RwLock<u32>>,
}

// 创建新用户
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

// 获取所有用户
async fn get_users(State(state): State<AppState>) -> impl IntoResponse {
    let users = state.users.read().unwrap();
    let users_list: Vec<User> = users.values().cloned().collect();
    Json(users_list)
}

// 获取单个用户
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

// 更新用户
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

// 删除用户
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

// 健康检查
async fn health_check() -> impl IntoResponse {
    Json(serde_json::json!({"status": "ok"}))
}

#[tokio::main]
async fn main() {
    // 初始化应用状态
    let state = AppState {
        users: Arc::new(RwLock::new(HashMap::new())),
        next_id: Arc::new(RwLock::new(1)),
    };

    // 添加示例数据
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

    // 配置 CORS - 允许前端访问
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // 构建路由
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/users", get(get_users).post(create_user))
        .route("/users/:id", get(get_user).put(update_user).delete(delete_user))
        .with_state(state)
        .layer(cors);  // CORS 层需要在 with_state 之后

    // 从环境变量读取端口（Railway 需要）
    let port = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .expect("PORT must be a number");

    let addr = format!("0.0.0.0:{}", port);
    println!("🚀 Server running on http://{}", addr);
    println!("📊 Health check: http://{}/health", addr);
    println!("👥 Users API: http://{}/users", addr);
    
    // 启动服务器
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}