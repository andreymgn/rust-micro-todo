use crate::repository::error::Error;
use crate::repository::model::{Todo, Todos};
use crate::repository::repository::Repository;
use sqlx::error::Error as SQLxError;
use sqlx::migrate::Migrator;
use sqlx::PgPool;
use std::path::Path;

pub struct PostgresRepository {
    pool: PgPool,
    id_generator: libxid::Generator,
}

impl PostgresRepository {
    pub async fn new(connection_string: &str) -> Result<PostgresRepository, SQLxError> {
        let pool = PgPool::connect(connection_string).await?;
        Ok(PostgresRepository {
            pool,
            id_generator: libxid::new_generator(),
        })
    }

    pub async fn run_migrations(&self, migrations_path: &str) -> Result<(), SQLxError> {
        let migrations_path = Path::new(migrations_path);
        let migrator = Migrator::new(migrations_path).await?;
        migrator.run(&self.pool).await?;

        Ok(())
    }
}

#[async_trait]
impl Repository for PostgresRepository {
    async fn list(&self) -> Result<Todos, Error> {
        let query = r#"
SELECT
    id, title, body, is_completed, created_at, updated_at
FROM
    todos
    "#;
        let todos = sqlx::query_as::<_, Todo>(query)
            .fetch_all(&self.pool)
            .await?;

        Ok(todos)
    }

    async fn get(&self, id: &str) -> Result<Todo, Error> {
        let query = r#"
SELECT
    id, title, body, is_completed, created_at, updated_at
FROM
    todos
WHERE
    id = $1
    "#;
        let todo = sqlx::query_as::<_, Todo>(query)
            .bind(id)
            .fetch_one(&self.pool)
            .await?;

        Ok(todo)
    }

    async fn create(&self, title: String, body: String) -> Result<Todo, Error> {
        let query = r#"
INSERT INTO
    todos (id, title, body, is_completed, created_at, updated_at)
VALUES
    ($1, $2, $3, FALSE, NOW(), NOW())
RETURNING
    id, title, body, is_completed, created_at, updated_at
    "#;
        let id = self.id_generator.new_id()?.encode();
        let todo = sqlx::query_as::<_, Todo>(query)
            .bind(id)
            .bind(title)
            .bind(body)
            .fetch_one(&self.pool)
            .await?;

        Ok(todo)
    }

    async fn update(
        &self,
        id: &str,
        title: String,
        body: String,
        is_completed: bool,
    ) -> Result<Todo, Error> {
        let query = r#"
UPDATE
    todos
SET
    title = $2, body = $3, is_completed = $4, updated_at = NOW()
WHERE
    id = $1
RETURNING
    id, title, body, is_completed, created_at, updated_at
    "#;
        let todo = sqlx::query_as::<_, Todo>(query)
            .bind(id)
            .bind(title)
            .bind(body)
            .bind(is_completed)
            .fetch_one(&self.pool)
            .await?;

        Ok(todo)
    }

    async fn delete(&self, id: &str) -> Result<(), Error> {
        let query = r#"
DELETE FROM
    todos
WHERE
    id = $1
    "#;
        sqlx::query(query).bind(id).execute(&self.pool).await?;

        Ok(())
    }

    async fn complete(&self, id: &str) -> Result<Todo, Error> {
        let query = r#"
UPDATE
    todos
SET
    is_completed = TRUE, updated_at = NOW()
WHERE
    id = $1
RETURNING
    id, title, body, is_completed, created_at, updated_at
    "#;
        let todo = sqlx::query_as::<_, Todo>(query)
            .bind(id)
            .fetch_one(&self.pool)
            .await?;

        Ok(todo)
    }
}
