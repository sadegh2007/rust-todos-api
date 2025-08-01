use axum::{extract::{Path}, Json};
use axum_valid::Valid;
use serde_json::{json, Value};

use crate::{features::todos::{data::todo_repository::TodoRepository, dtos::upsert_todo_request::UpsertTodoRequest}};

pub async fn update_todo(
    Path(id): Path<uuid::Uuid>,
    body: Valid<Json<UpsertTodoRequest>>,
) -> Result<Json<Value>, (axum::http::StatusCode, Json<Value>)> {
    let repo = TodoRepository::new();
    let updated_todo = repo.update(id, &body);

    if let Some(todo) = updated_todo {
        return Ok(Json(json!(todo)));
    }

    return Err((axum::http::StatusCode::NOT_FOUND, Json(json!({"error": "Todo not found"}))));
}