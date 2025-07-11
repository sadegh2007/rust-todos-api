use axum::{Json};
use serde_json::{json, Value};

use crate::{features::todos::{data::todo_repository::TodoRepository}};

pub async fn get_todos_list() -> Json<Value> {
    let repo = TodoRepository::new();
    return Json(json!(repo.get_todos()));
}