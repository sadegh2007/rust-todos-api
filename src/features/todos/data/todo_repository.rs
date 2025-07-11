use diesel::{query_dsl::methods::FilterDsl, ExpressionMethods, RunQueryDsl};

use crate::{establish_connection, features::todos::{domain::todo::{Todo}, dtos::{todo_dto::TodoDto, upsert_todo_request::UpsertTodoRequest}}, schema};

pub struct TodoRepository;

impl TodoRepository {
    pub fn new() -> Self {
        TodoRepository {}
    }

    pub fn get_todos(&self) -> Vec<TodoDto> {
        let conn = &mut establish_connection();

        let result = schema::todos::table
            .load::<Todo>(conn);

        if result.is_err() {
            eprintln!("Error loading todos: {:?}", result.err());
            return Vec::new();
        } 

        // Convert the Vec<Todo> to Vec<TodoDto>
        return result.unwrap().into_iter()
            .map(| todo | TodoDto::from_todo(&todo))
            .collect();
    }
    
    pub fn create(&self, request: &UpsertTodoRequest) -> Option<TodoDto> {
        let conn = &mut establish_connection();

        let todo = Todo::new(request.title.to_string(), request.content.to_string());
        
        let result = diesel::insert_into(schema::todos::table)
            .values(request)
            .get_result::<Todo>(conn);

        if result.is_err() {
            eprintln!("Error inserting todo: {:?}", result.err());
            return None;
        }

        return Some(TodoDto::from_todo(&todo));
    }

    pub fn get(&self, id: uuid::Uuid) -> Option<TodoDto> {
        let conn = &mut establish_connection();

        let result = schema::todos::table
            .filter(schema::todos::id.eq(id))
            .first::<Todo>(conn);

        return match result {
            Ok(todo) => Some(TodoDto::from_todo(&todo)),
            Err(_) => None,
        };
    }

    pub fn update(&self, id: uuid::Uuid, request: &UpsertTodoRequest) -> Option<TodoDto> {
        let conn = &mut establish_connection();

        let result = schema::todos::table
            .filter(schema::todos::id.eq(id))
            .first::<Todo>(conn);

        if result.is_err() {
            return None;
        }

        let update_result = diesel::update(schema::todos::table)
            .filter(schema::todos::id.eq(id))
            .set(request)
            .get_result::<Todo>(conn);

        return match update_result {
            Ok(todo) => Some(TodoDto::from_todo(&todo)),
            Err(_) => None,
        };
    }

    pub fn delete(&self, id: uuid::Uuid) -> bool {
         let conn = &mut establish_connection();

         let result = diesel::delete(schema::todos::table)
            .filter(schema::todos::id.eq(id))
            .execute(conn);

        return match result {
            Ok(rows_deleted) => rows_deleted > 0,
            Err(_) => false,
        };
    }
}