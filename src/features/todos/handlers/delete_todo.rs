use std::sync::Arc;

use axum::{extract::{Path, State}, Json};
use serde_json::{json, Value};

use crate::{app_state::AppState, features::todos::{data::todo_repository::TodoRepository}};

pub async fn delete_todo(
    Path(id): Path<uuid::Uuid>,
    State(state): State<Arc<AppState>>
) -> Result<Json<Value>, (axum::http::StatusCode, Json<Value>)> {
    let repo = TodoRepository::new(State(state));
    let is_deleted = repo.delete(id);

    if is_deleted {
        return Ok(Json(json!({ "message": "Todo deleted successfully" })));
    }

    return Err((axum::http::StatusCode::NOT_FOUND, Json(json!({"error": "Todo not found"}))));
}