use crate::repository::error::Error;
use crate::repository::hashmap::HashMapRepository;
use crate::repository::model::{Todo, Todos};
use crate::repository::postgres::PostgresRepository;
use async_trait::async_trait;
use config::{Config, Environment};

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
    pub migrations_path: String,
}

#[derive(Debug, Deserialize, Clone)]
pub enum StorageSettings {
    Postgres,
    HashMap,
}

pub async fn get_repository(
    params: StorageSettings,
    logger: slog::Logger,
) -> Result<Box<dyn Repository + Send + Sync>, Box<dyn std::error::Error>> {
    match params {
        StorageSettings::Postgres => {
            let mut c = Config::default();
            c.merge(Environment::with_prefix("TODO_POSTGRES"))?;
            let s = c.try_into::<PostgresSettings>()?;

            let repo = PostgresRepository::new(s.connection_string.as_str()).await?;
            repo.run_migrations(s.migrations_path.as_str()).await?;

            Ok(Box::new(repo))
        }
        StorageSettings::HashMap => Ok(Box::new(HashMapRepository::new(logger))),
    }
}
