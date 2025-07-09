use serde::Deserialize;

#[derive(Deserialize)]
pub struct UpsertTodoRequest {
    pub title: String,
    pub content: String
}