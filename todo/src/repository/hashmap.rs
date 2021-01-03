use crate::repository::error::Error;
use crate::repository::model::{Todo, Todos};
use crate::repository::repository::Repository;
use async_trait::async_trait;
use chrono::Utc;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct HashMapRepository {
    logger: slog::Logger,
    db: Arc<RwLock<HashMap<String, Todo>>>,
    id_generator: libxid::Generator,
}

impl HashMapRepository {
    pub fn new(logger: slog::Logger) -> HashMapRepository {
        HashMapRepository {
            logger,
            db: Arc::new(RwLock::new(HashMap::new())),
            id_generator: libxid::new_generator(),
        }
    }
}

#[async_trait]
impl Repository for HashMapRepository {
    async fn list(&self) -> Result<Todos, Error> {
        let lock = self.db.clone();
        let db = lock.read().await;
        let mut todos = Vec::with_capacity(db.len());
        for v in db.values() {
            todos.push(v.clone());
        }

        Ok(todos)
    }

    async fn get(&self, id: &str) -> Result<Todo, Error> {
        let lock = self.db.clone();
        let db = lock.read().await;
        match db.get(id) {
            Some(todo) => Ok(todo.clone()),
            None => {
                error!(self.logger, "todo not found"; "id" => id);
                Err(Error::NotFound)
            }
        }
    }

    async fn create(&self, title: String, body: String) -> Result<Todo, Error> {
        let lock = self.db.clone();
        let mut db = lock.write().await;
        let id = self.id_generator.new_id()?.encode();
        let now = Utc::now();
        let todo = Todo {
            id: id.clone(),
            title,
            body,
            is_completed: false,
            created_at: now,
            updated_at: now,
        };
        db.insert(id, todo.clone());
        Ok(todo)
    }

    async fn update(
        &self,
        id: &str,
        title: String,
        body: String,
        is_completed: bool,
    ) -> Result<Todo, Error> {
        let lock = self.db.clone();
        let mut db = lock.write().await;
        match db.get_mut(id) {
            Some(mut todo) => {
                let now = Utc::now();
                todo.title = title;
                todo.body = body;
                todo.is_completed = is_completed;
                todo.updated_at = now;

                Ok(todo.clone())
            }
            None => {
                error!(self.logger, "todo not found"; "id" => id);
                Err(Error::NotFound)
            }
        }
    }

    async fn delete(&self, id: &str) -> Result<(), Error> {
        let lock = self.db.clone();
        let mut db = lock.write().await;
        match db.remove(id) {
            Some(_) => Ok(()),
            None => {
                error!(self.logger, "todo not found"; "id" => id);
                Err(Error::NotFound)
            }
        }
    }

    async fn complete(&self, id: &str) -> Result<Todo, Error> {
        let lock = self.db.clone();
        let mut db = lock.write().await;
        match db.get_mut(id) {
            Some(mut todo) => {
                if todo.is_completed {
                    return Err(Error::AlreadyCompleted);
                }

                let now = Utc::now();
                todo.is_completed = true;
                todo.updated_at = now;
                Ok(todo.clone())
            }
            None => {
                error!(self.logger, "todo not found"; "id" => id);
                Err(Error::NotFound)
            }
        }
    }
}
