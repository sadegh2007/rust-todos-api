use axum::{Json};
use axum_valid::Valid;
use serde_json::{json, Value};

use crate::{features::todos::{data::todo_repository::TodoRepository, dtos::{upsert_todo_request::UpsertTodoRequest}}};

pub async fn create_todo(
    body: Valid<Json<UpsertTodoRequest>>
) -> Result<Json<Value>, (axum::http::StatusCode, Json<Value>)> {
    let repo = TodoRepository::new();
    let todo = repo.create(&body);

    if let Some(todo) = todo {
        return Ok(Json(json!(todo)));
    }

    return Err((axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Failed to create todo"}))));
}