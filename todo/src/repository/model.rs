use super::super::server::todo_service as pb;
use chrono::{DateTime, Timelike, Utc};

#[derive(Debug, Clone)]
pub struct Todo {
    pub id: String,
    pub title: String,
    pub body: String,
    pub is_completed: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Into<pb::Todo> for Todo {
    fn into(self) -> pb::Todo {
        let created_at = prost_types::Timestamp {
            seconds: self.created_at.second() as i64,
            nanos: self.created_at.nanosecond() as i32,
        };
        let updated_at = prost_types::Timestamp {
            seconds: self.updated_at.second() as i64,
            nanos: self.updated_at.nanosecond() as i32,
        };
        pb::Todo {
            id: self.id,
            title: self.title,
            body: self.body,
            is_completed: self.is_completed,
            created_at: Some(created_at),
            updated_at: Some(updated_at),
        }
    }
}

pub type Todos = Vec<Todo>;

impl Into<pb::Todos> for Todos {
    fn into(self) -> pb::Todos {
        let converted_todos = self.iter().map(|todo| todo.clone().into()).collect();
        pb::Todos {
            todos: converted_todos,
        }
    }
}
