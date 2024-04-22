use std::fmt::Error;
use chrono::prelude::*;
use std::sync::{Arc, Mutex};

use crate::models::todo::Todo;

pub struct Database {
    pub todos: Arc<Mutex<Vec<Todo>>>,
}

impl Database {
    pub fn new() -> Self {
        let todos = Arc::new(Mutex::new(vec![]));
        Database { todos }
    }

    pub fn get_todos(&self) -> Vec<Todo> {
        let todos = self.todos.lock().unwrap();
        todos.clone()
    }

    pub fn get_todo(&self, id: &str) -> Option<Todo> {
        let todos = self.todos.lock().unwrap();
        todos.iter().find(|todo| todo.id == Some(id.to_string())).cloned()
    }

    pub fn create_todo(&self, todo: Todo) -> Result<Todo, Error> {
        let mut todos = self.todos.lock().unwrap();
        let id = uuid::Uuid::new_v4().to_string();
        let created_at = Utc::now();
        let updated_at = Utc::now();
        let todo = Todo {
            id: Some(id),
            created_at: Some(created_at),
            updated_at: Some(updated_at),
            ..todo
        };
        todos.push(todo.clone());
        Ok(todo)
    }

    pub fn update_todo(&self, id: &str, todo: Todo) -> Option<Todo> {
        let mut todos = self.todos.lock().unwrap();
        let index = todos.iter().position(|todo| todo.id == Some(id.to_string()))?;
        let created_at = todos[index].created_at.unwrap();
        let updated_at = Utc::now();
        let todo = Todo {
            id: Some(id.to_string()),
            created_at: Some(created_at),
            updated_at: Some(updated_at),
            ..todo
        };
        todos[index] = todo.clone();
        Some(todo)
    }

    pub fn delete_todo(&self, id: &str) -> Option<Todo> {
        let mut todos = self.todos.lock().unwrap();
        let index = todos.iter().position(| todo | todo.id == Some(id.to_string()))?;
        Some(todos.remove(index))
    }
}
