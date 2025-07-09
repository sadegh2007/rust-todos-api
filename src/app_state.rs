use std::sync::{Arc, Mutex};
use crate::features::todos::domain::todo::Todo;

#[derive(Clone)]
pub struct AppState {
    pub todos: Arc<Mutex<Vec<Todo>>>, // Example field, you can add more as needed
}
