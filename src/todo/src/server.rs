use std::collections::HashMap;
use std::sync::Arc;
use std::time::{SystemTime, SystemTimeError};

use tokio::sync::RwLock;

use todo_service as pb;
use todo_service::todo_service_server::TodoService;

pub mod todo_service {
    tonic::include_proto!("todo");
}

pub struct TodoServiceImpl {
    db: Arc<RwLock<HashMap<String, pb::Todo>>>,
    id_generator: libxid::Generator,
}

impl TodoServiceImpl {
    pub(crate) fn new() -> TodoServiceImpl {
        TodoServiceImpl {
            db: Arc::new(RwLock::new(HashMap::new())),
            id_generator: libxid::new_generator(),
        }
    }
}

fn current_timestamp() -> Result<prost_types::Timestamp, SystemTimeError> {
    let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
    let seconds = now.as_secs();
    Ok(prost_types::Timestamp {
        seconds: seconds as i64,
        nanos: 0,
    })
}

#[tonic::async_trait]
impl TodoService for TodoServiceImpl {
    async fn list(
        &self,
        _request: tonic::Request<pb::ListRequest>,
    ) -> Result<tonic::Response<pb::Todos>, tonic::Status> {
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
        let lock = self.db.clone();
        let mut db = lock.write().await;
        let id = match self.id_generator.new_id() {
            Ok(id) => id,
            Err(e) => return Err(tonic::Status::internal(["failed to generate id", &e.to_string()].concat()))
        };
        let now = match current_timestamp() {
            Ok(ts) => ts,
            Err(e) => return Err(tonic::Status::internal(["failed to get time: ", &e.to_string()].concat())),
        };
        let todo = pb::Todo {
            id: id.to_string(),
            title: request.get_ref().title.clone(),
            body: request.get_ref().body.clone(),
            is_completed: false,
            created_at: Some(now.clone()),
            updated_at: Some(now.clone()),
        };
        db.insert(id.to_string(), todo.clone());

        Ok(tonic::Response::new(todo))
    }

    async fn get_by_id(
        &self,
        request: tonic::Request<pb::TodoId>,
    ) -> Result<tonic::Response<pb::Todo>, tonic::Status> {
        let lock = self.db.clone();
        let db = lock.read().await;
        match db.get(&request.get_ref().id) {
            Some(todo) => Ok(tonic::Response::new(todo.clone())),
            None => Err(tonic::Status::not_found("todo not found")),
        }
    }

    async fn update(
        &self,
        request: tonic::Request<pb::UpdateRequest>,
    ) -> Result<tonic::Response<pb::Todo>, tonic::Status> {
        let lock = self.db.clone();
        let mut db = lock.write().await;
        match db.get_mut(&request.get_ref().id) {
            Some(mut todo) => {
                let now = match current_timestamp() {
                    Ok(ts) => ts,
                    Err(e) => return Err(tonic::Status::internal(["failed to get time: ", &e.to_string()].concat())),
                };
                todo.title = request.get_ref().title.clone();
                todo.body = request.get_ref().body.clone();
                todo.is_completed = request.get_ref().is_completed;
                todo.updated_at = Some(now.clone());

                Ok(tonic::Response::new(todo.clone()))
            }
            None => Err(tonic::Status::not_found("todo not found")),
        }
    }
    async fn delete(
        &self,
        request: tonic::Request<pb::TodoId>,
    ) -> Result<tonic::Response<()>, tonic::Status> {
        let lock = self.db.clone();
        let mut db = lock.write().await;
        match db.remove(&request.get_ref().id) {
            Some(_) => Ok(tonic::Response::new(())),
            None => Err(tonic::Status::not_found("todo not found"))
        }
    }
}