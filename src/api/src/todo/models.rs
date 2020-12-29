use chrono::{DateTime, TimeZone, Utc};
use serde_derive::{Deserialize, Serialize};

use crate::todo::service::todo_service as pb;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CreateTodo {
    pub title: String,
    pub body: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UpdateTodo {
    pub title: String,
    pub body: String,
    pub is_completed: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Todo {
    pub id: String,
    pub title: String,
    pub body: String,
    pub is_completed: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<pb::Todo> for Todo {
    fn from(todo: pb::Todo) -> Self {
        Todo {
            id: todo.id,
            title: todo.title,
            body: todo.body,
            is_completed: todo.is_completed,
            created_at: match todo.created_at {
                Some(v) => chrono::Utc.timestamp(v.seconds, v.nanos as u32),
                None => chrono::Utc.timestamp(0, 0),
            },
            updated_at: match todo.updated_at {
                Some(v) => chrono::Utc.timestamp(v.seconds, v.nanos as u32),
                None => chrono::Utc.timestamp(0, 0),
            },
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Todos {
    pub todos: Vec<Todo>,
}

impl From<pb::Todos> for Todos {
    fn from(todos: pb::Todos) -> Self {
        let v = todos.todos.iter().map(|todo| Todo::from(todo.clone())).collect();
        Todos {
            todos: v,
        }
    }
}
