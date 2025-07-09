use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Validate, Deserialize)]
pub struct UpsertTodoRequest {
    #[validate(length(min = 1, max = 255))]
    pub title: String,
    #[validate(length(min = 3, max = 500))]
    pub content: String
}