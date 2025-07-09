use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TodoDto {
    pub id: uuid::Uuid,
    pub title: String,
    pub content: String,
    pub completed: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl TodoDto {
    pub fn new(
        id: uuid::Uuid,
        title: String,
        content: String,
        completed: bool,
        created_at: chrono::DateTime<chrono::Utc>,
        updated_at: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Self {
        TodoDto {
            id,
            title,
            content,
            completed,
            created_at,
            updated_at,
        }
    }
    
    pub fn from_todo(todo: &crate::features::todos::domain::todo::Todo) -> Self {
        TodoDto {
            id: todo.id,
            title: todo.title.clone(),
            content: todo.content.clone(),
            completed: todo.completed,
            created_at: todo.created_at,
            updated_at: todo.updated_at,
        }
    }
}