use std::collections::HashMap;
use std::sync::Arc;
use std::time::SystemTime;

use tokio::sync::RwLock;

use todo_service as pb;
use todo_service::todo_service_server::TodoService;

use crate::error::Error;

pub mod todo_service {
    tonic::include_proto!("todo");
}

pub struct TodoServiceImpl {
    logger: slog::Logger,
    db: Arc<RwLock<HashMap<String, pb::Todo>>>,
    id_generator: libxid::Generator,
}

impl TodoServiceImpl {
    pub(crate) fn new(log: slog::Logger) -> TodoServiceImpl {
        TodoServiceImpl {
            logger: log,
            db: Arc::new(RwLock::new(HashMap::new())),
            id_generator: libxid::new_generator(),
        }
    }
}

fn current_timestamp() -> prost_types::Timestamp {
    let now = SystemTime::now();
    now.into()
}

#[tonic::async_trait]
impl TodoService for TodoServiceImpl {
    async fn list(
        &self,
        _request: tonic::Request<pb::ListRequest>,
    ) -> Result<tonic::Response<pb::Todos>, tonic::Status> {
        debug!(self.logger, "list";);
        let lock = self.db.clone();
        let db = lock.read().await;
        let mut todos = Vec::with_capacity(db.len());
        for v in db.values() {
            todos.push(v.clone());
        }

        let result = pb::Todos {
            todos,
        };

        Ok(tonic::Response::new(result))
    }

    async fn create(
        &self,
        request: tonic::Request<pb::CreateRequest>,
    ) -> Result<tonic::Response<pb::Todo>, tonic::Status> {
        debug!(self.logger, "create");
        let lock = self.db.clone();
        let mut db = lock.write().await;
        let id = self.id_generator.new_id()
            .map_err(|e| {
                error!(self.logger, "failed to generate xid"; "err" => e.to_string());
                Error::IDGenerationError(e)
            })?;
        let now = current_timestamp();
        let todo = pb::Todo {
            id: id.encode(),
            title: request.get_ref().title.clone(),
            body: request.get_ref().body.clone(),
            is_completed: false,
            created_at: Some(now.clone()),
            updated_at: Some(now.clone()),
        };
        db.insert(id.encode(), todo.clone());

        Ok(tonic::Response::new(todo))
    }

    async fn get_by_id(
        &self,
        request: tonic::Request<pb::TodoId>,
    ) -> Result<tonic::Response<pb::Todo>, tonic::Status> {
        debug!(self.logger, "get_by_id";);
        let lock = self.db.clone();
        let db = lock.read().await;
        let id = &request.get_ref().id;
        match db.get(id) {
            Some(todo) => Ok(tonic::Response::new(todo.clone())),
            None => {
                error!(self.logger, "todo not found"; "id" => id);
                Err(tonic::Status::not_found("todo not found"))
            }
        }
    }

    async fn update(
        &self,
        request: tonic::Request<pb::UpdateRequest>,
    ) -> Result<tonic::Response<pb::Todo>, tonic::Status> {
        debug!(self.logger, "update";);
        let lock = self.db.clone();
        let mut db = lock.write().await;
        let id = &request.get_ref().id;
        match db.get_mut(id) {
            Some(mut todo) => {
                let now = current_timestamp();
                todo.title = request.get_ref().title.clone();
                todo.body = request.get_ref().body.clone();
                todo.is_completed = request.get_ref().is_completed;
                todo.updated_at = Some(now.clone());

                Ok(tonic::Response::new(todo.clone()))
            }
            None => {
                error!(self.logger, "todo not found"; "id" => id);
                Err(tonic::Status::not_found("todo not found"))
            }
        }
    }

    async fn delete(
        &self,
        request: tonic::Request<pb::TodoId>,
    ) -> Result<tonic::Response<()>, tonic::Status> {
        debug!(self.logger, "delete";);
        let lock = self.db.clone();
        let mut db = lock.write().await;
        let id = &request.get_ref().id;
        match db.remove(id) {
            Some(_) => Ok(tonic::Response::new(())),
            None => {
                error!(self.logger, "todo not found"; "id" => id);
                Err(tonic::Status::not_found("todo not found"))
            }
        }
    }

    async fn complete(
        &self,
        request: tonic::Request<pb::TodoId>,
    ) -> Result<tonic::Response<pb::Todo>, tonic::Status> {
        debug!(self.logger, "complete";);
        let lock = self.db.clone();
        let mut db = lock.write().await;
        let id = &request.get_ref().id;
        match db.get_mut(id) {
            Some(mut todo) => {
                if todo.is_completed {
                    return Err(tonic::Status::invalid_argument("todo is already completed"));
                }

                let now = current_timestamp();
                todo.is_completed = true;
                todo.updated_at = Some(now);
                Ok(tonic::Response::new(todo.clone()))
            }
            None => {
                error!(self.logger, "todo not found"; "id" => id);
                Err(tonic::Status::not_found("todo not found"))
            }
        }
    }
}