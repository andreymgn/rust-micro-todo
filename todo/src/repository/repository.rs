use crate::repository::error::Error;
use crate::repository::hashmap::HashMapRepository;
use crate::repository::model::{Todo, Todos};
use async_trait::async_trait;

#[async_trait]
pub trait Repository {
    async fn list(&self) -> Result<Todos, Error>;
    async fn get(&self, id: &str) -> Result<Todo, Error>;
    async fn create(&self, title: String, body: String) -> Result<Todo, Error>;
    async fn update(
        &self,
        id: &str,
        title: String,
        body: String,
        is_completed: bool,
    ) -> Result<Todo, Error>;
    async fn delete(&self, id: &str) -> Result<(), Error>;
    async fn complete(&self, id: &str) -> Result<Todo, Error>;
}

#[derive(Debug, Deserialize, Clone)]
pub struct PostgresSettings {
    pub connection_string: String,
}

#[derive(Debug, Deserialize, Clone)]
pub enum StorageSettings {
    Postgres(PostgresSettings),
    HashMap,
}

pub fn get_repository(
    params: StorageSettings,
    logger: slog::Logger,
) -> Result<Box<dyn Repository + Send + Sync>, Box<dyn std::error::Error>> {
    match params {
        StorageSettings::Postgres(_) => unimplemented!(),
        StorageSettings::HashMap => Ok(Box::new(HashMapRepository::new(logger))),
    }
}
