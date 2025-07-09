use std::sync::Arc;

use axum::{extract::State, Json};
use serde_json::{json, Value};

use crate::{app_state::AppState, features::todos::{data::todo_repository::TodoRepository}};

pub async fn get_todos_list(State(state): State<Arc<AppState>>) -> Json<Value> {
    let repo = TodoRepository::new(State(state));
    return Json(json!(repo.get_todos()));
}