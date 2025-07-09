use std::sync::Arc;

use axum::{extract::State, Json};
use serde_json::{json, Value};

use crate::{app_state::AppState, features::todos::{data::todo_repository::TodoRepository, dtos::{upsert_todo_request::UpsertTodoRequest}}};

pub async fn create_todo(
    State(state): State<Arc<AppState>>,
    Json(body): Json<UpsertTodoRequest>
) -> Json<Value> {
    let repo = TodoRepository::new(State(state));
    let todo = repo.create(&body);
    return Json(json!(todo));
}