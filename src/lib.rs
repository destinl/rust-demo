use axum::{extract::{Path, State}, Json, Router, routing::{delete, get, post, put}};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::{Arc, Mutex}};

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

async fn get_item(Path(id): Path<u64>, State(state): State<AppState>) -> Option<Json<Item>> {
    let store = state.lock().unwrap();
    store.items.get(&id).cloned().map(Json)
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
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use hyper::body::to_bytes;

    #[tokio::test]
    async fn test_crud_flow() {
        let app = app();

        // initial list
        let response = app
            .clone()
            .oneshot(Request::builder().uri("/items").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let body = to_bytes(response.into_body()).await.unwrap();
        assert_eq!(body, "[]");

        // create item
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/items")
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"name":"task1"}"#))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let item: Item = serde_json::from_slice(&to_bytes(response.into_body()).await.unwrap()).unwrap();
        assert_eq!(item.id, 1);
        assert_eq!(item.name, "task1");

        // get item
        let response = app
            .clone()
            .oneshot(Request::builder().uri("/items/1").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        // update item
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("PUT")
                    .uri("/items/1")
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"name":"updated"}"#))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let item: Item = serde_json::from_slice(&to_bytes(response.into_body()).await.unwrap()).unwrap();
        assert_eq!(item.name, "updated");

        // delete item
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("DELETE")
                    .uri("/items/1")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::NO_CONTENT);
    }
}
