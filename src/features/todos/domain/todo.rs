use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Todo {
    pub id: uuid::Uuid,
    pub title: String,
    pub content: String,
    pub completed: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl Todo {
    pub fn new(title: String, content: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            title,
            content,
            completed: false,
            created_at: chrono::Utc::now(),
            updated_at: None,
        }
    }

    pub fn mark_completed(&mut self) {
        self.completed = true;
        self.updated_at = Some(chrono::Utc::now());
    }

}