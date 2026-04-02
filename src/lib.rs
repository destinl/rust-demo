use axum::{extract::{Path, State}, Json, Router, routing::{delete, get, post, put}};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::{Arc, Mutex}};
use tokio::sync::Mutex;
use axum::response::IntoResponse;

pub type AppState = Arc<Mutex<Store>>;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Item {
    pub id: u64,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateItem {
    pub name: String,
}

#[derive(Default)]
pub struct Store {
    items: HashMap<u64, Item>,
    next_id: u64,
}

impl Store {
    fn next_id(&mut self) -> u64 {
        self.next_id += 1;
        self.next_id
    }
}

pub fn app() -> Router {
    let state: AppState = Arc::new(Mutex::new(Store::default()));

    Router::new()
        .route("/items", get(list_items).post(create_item))
        .route(
            "/items/:id",
            get(get_item).put(update_item).delete(delete_item),
        )
        .with_state(state)
}

async fn list_items(State(state): State<AppState>) -> Json<Vec<Item>> {
    let store = state.lock().unwrap();
    let items = store.items.values().cloned().collect();
    Json(items)
}

async fn create_item(
    State(state): State<AppState>,
    Json(payload): Json<CreateItem>,
) -> Json<Item> {
    let mut store = state.lock().unwrap();
    let id = store.next_id();
    let item = Item { id, name: payload.name };
    store.items.insert(id, item.clone());
    Json(item)
}


async fn get_item(Path(id): Path<u64>, State(state): State<AppState>) -> impl IntoResponse {
    let store = state.lock().unwrap();
    match store.items.get(&id) {
        Some(item) => Json(item).into_response(),
        None => (StatusCode::NOT_FOUND, "Item not found").into_response(),
    }
}

async fn update_item(
    Path(id): Path<u64>,
    State(state): State<AppState>,
    Json(payload): Json<CreateItem>,
) -> Option<Json<Item>> {
    let mut store = state.lock().unwrap();
    if let Some(item) = store.items.get_mut(&id) {
        item.name = payload.name.clone();
        return Some(Json(item.clone()));
    }
    None
}

async fn delete_item(Path(id): Path<u64>, State(state): State<AppState>) -> axum::http::StatusCode {
    let mut store = state.lock().unwrap();
    if store.items.remove(&id).is_some() {
        axum::http::StatusCode::NO_CONTENT
    } else {
        axum::http::StatusCode::NOT_FOUND
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;
    use axum_test::TestServer;

    #[tokio::test]
    async fn test_crud_flow() {
        let app = app();
        let server = TestServer::new(app).unwrap();

        // 初始列表
        let response = server.get("/items").await;
        assert_eq!(response.status_code(), StatusCode::OK);
        assert_eq!(response.text(), "[]");

        // 创建项目
        let response = server
            .post("/items")
            .json(&serde_json::json!({"name": "task1"}))
            .await;
        assert_eq!(response.status_code(), StatusCode::OK);
        let item: Item = response.json();
        assert_eq!(item.id, 1);
        assert_eq!(item.name, "task1");

        // 获取项目
        let response = server.get("/items/1").await;
        assert_eq!(response.status_code(), StatusCode::OK);

        // 更新项目
        let response = server
            .put("/items/1")
            .json(&serde_json::json!({"name": "updated"}))
            .await;
        assert_eq!(response.status_code(), StatusCode::OK);
        let item: Item = response.json();
        assert_eq!(item.name, "updated");

        // 删除项目
        let response = server.delete("/items/1").await;
        assert_eq!(response.status_code(), StatusCode::NO_CONTENT);
    }
}
