use std::sync::Arc;

use axum::extract::State;

use crate::{app_state::AppState, features::todos::{domain::todo::Todo, dtos::{todo_dto::TodoDto, upsert_todo_request::UpsertTodoRequest}}};

pub struct TodoRepository {
    state: State<Arc<AppState>>,
}

impl TodoRepository {
    pub fn new(State(state): State<Arc<AppState>>) -> Self {
        TodoRepository {
            state: State(state)
        }
    }

    pub fn get_todos(&self) -> Vec<TodoDto> {
        let todos = self.state.todos.lock().unwrap().iter()
        .map(|todo| TodoDto::from_todo(todo))
        .collect::<Vec<TodoDto>>();

        return todos;
    }
    
    pub fn create(&self, request: &UpsertTodoRequest) -> TodoDto {
        let todo = Todo::new(request.title.to_string(), request.content.to_string());

        let mut todos = self.state.todos.lock().unwrap();
        todos.push(todo.clone());

        return TodoDto::from_todo(&todo);
    }

    pub fn get(&self, id: uuid::Uuid) -> Option<TodoDto> {
        let todos = self.state.todos.lock().unwrap();

        return todos.iter()
            .find(|todo| todo.id == id)
            .map(|todo| TodoDto::from_todo(todo));
    }

    pub fn update(&self, id: uuid::Uuid, request: &UpsertTodoRequest) -> Option<TodoDto> {
        let mut todos = self.state.todos.lock().unwrap();

        if let Some(existing_todo) = todos.iter_mut().find(|todo| todo.id == id) {
            existing_todo.title = request.title.to_string();
            existing_todo.content = request.content.to_string();
            existing_todo.updated_at = Some(chrono::Utc::now());
            
            return Some(TodoDto::from_todo(existing_todo));
        }

        return None;
    }

    pub fn delete(&self, id: uuid::Uuid) -> bool {
        let mut todos = self.state.todos.lock().unwrap();
        let initial_len = todos.len();

        todos.retain(|todo| todo.id != id);

        return todos.len() < initial_len;
        
    }
}