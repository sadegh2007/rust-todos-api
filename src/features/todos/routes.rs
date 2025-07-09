use std::sync::Arc;

use axum::routing::{delete, get, patch, post};
use axum::Router;

use crate::app_state::AppState;
use crate::features::todos::handlers::{
    get_todos_list::get_todos_list,
    create_todo::create_todo,
    get_todo::get_todo,
    update_todo::update_todo,
    delete_todo::delete_todo,
};

pub fn create_todos_routes() -> Router<Arc<AppState>> {
    return Router::new()
        .route("/todos", get(get_todos_list))
        .route("/todos", post(create_todo))
        .route("/todos/{id}", get(get_todo))
        .route("/todos/{id}", patch(update_todo))
        .route("/todos/{id}", delete(delete_todo));
}