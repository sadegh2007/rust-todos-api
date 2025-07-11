use chrono::prelude::*;
use diesel::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Selectable)]
#[diesel(table_name = crate::schema::todos)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Todo {
    pub id: uuid::Uuid,
    pub title: String,
    pub content: String,
    pub completed: Option<bool>,
    pub created_at: Option<DateTime<chrono::Utc>>,
    pub updated_at: Option<DateTime<chrono::Utc>>,
}

impl Todo {
    pub fn new(title: String, content: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            title,
            content,
            completed: Some(false),
            created_at: Some(Utc::now()),
            updated_at: None,
        }
    }

    pub fn mark_completed(&mut self) {
        self.completed = Some(true);
        self.updated_at = Some(Utc::now());
    }
}