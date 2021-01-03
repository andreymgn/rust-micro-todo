use todo_service as pb;
use todo_service::todo_service_server::TodoService;

use crate::repository::repository::Repository;

pub mod todo_service {
    tonic::include_proto!("todo");
}

pub struct TodoServiceImpl {
    logger: slog::Logger,
    repo: Box<dyn Repository + Send + Sync>,
}

impl TodoServiceImpl {
    pub(crate) fn new(
        logger: slog::Logger,
        repo: Box<dyn Repository + Send + Sync>,
    ) -> TodoServiceImpl {
        TodoServiceImpl { logger, repo }
    }
}

#[tonic::async_trait]
impl TodoService for TodoServiceImpl {
    async fn list(
        &self,
        _request: tonic::Request<pb::ListRequest>,
    ) -> Result<tonic::Response<pb::Todos>, tonic::Status> {
        debug!(self.logger, "list";);

        let result = self.repo.list().await;

        match result {
            Ok(todos) => {
                debug!(self.logger, "list result"; "result" => ?todos);
                Ok(tonic::Response::new(todos.into()))
            }
            Err(e) => {
                error!(self.logger, "list"; "err" => ?e);
                Err(e.into())
            }
        }
    }

    async fn create(
        &self,
        request: tonic::Request<pb::CreateRequest>,
    ) -> Result<tonic::Response<pb::Todo>, tonic::Status> {
        debug!(self.logger, "create");

        let title = request.get_ref().title.clone();
        let body = request.get_ref().body.clone();

        let result = self.repo.create(title, body).await;
        match result {
            Ok(todo) => {
                debug!(self.logger, "create result"; "result" => ?todo);
                Ok(tonic::Response::new(todo.into()))
            }
            Err(e) => {
                error!(self.logger, "create"; "err" => ?e);
                Err(e.into())
            }
        }
    }

    async fn get_by_id(
        &self,
        request: tonic::Request<pb::TodoId>,
    ) -> Result<tonic::Response<pb::Todo>, tonic::Status> {
        debug!(self.logger, "get_by_id";);

        let id = &request.get_ref().id;

        let result = self.repo.get(id).await;
        match result {
            Ok(todo) => {
                debug!(self.logger, "get_by_id result"; "result" => ?todo);
                Ok(tonic::Response::new(todo.into()))
            }
            Err(e) => {
                error!(self.logger, "get_by_id"; "err" => ?e);
                Err(e.into())
            }
        }
    }

    async fn update(
        &self,
        request: tonic::Request<pb::UpdateRequest>,
    ) -> Result<tonic::Response<pb::Todo>, tonic::Status> {
        debug!(self.logger, "update";);

        let id = &request.get_ref().id;
        let title = request.get_ref().title.clone();
        let body = request.get_ref().body.clone();
        let is_completed = request.get_ref().is_completed;

        let result = self.repo.update(id, title, body, is_completed).await;
        match result {
            Ok(todo) => {
                debug!(self.logger, "update result"; "result" => ?todo);
                Ok(tonic::Response::new(todo.into()))
            }
            Err(e) => {
                error!(self.logger, "update"; "err" => ?e);
                Err(e.into())
            }
        }
    }

    async fn delete(
        &self,
        request: tonic::Request<pb::TodoId>,
    ) -> Result<tonic::Response<()>, tonic::Status> {
        debug!(self.logger, "delete";);

        let id = &request.get_ref().id.clone();
        let result = self.repo.delete(id).await;

        match result {
            Ok(_) => Ok(tonic::Response::new(())),
            Err(e) => {
                error!(self.logger, "delete"; "err" => ?e);
                Err(e.into())
            }
        }
    }

    async fn complete(
        &self,
        request: tonic::Request<pb::TodoId>,
    ) -> Result<tonic::Response<pb::Todo>, tonic::Status> {
        debug!(self.logger, "complete";);

        let id = &request.get_ref().id;
        let result = self.repo.complete(id).await;

        match result {
            Ok(todo) => {
                debug!(self.logger, "complete result"; "result" => ?todo);
                Ok(tonic::Response::new(todo.into()))
            }
            Err(e) => {
                error!(self.logger, "complete"; "err" => ?e);
                Err(e.into())
            }
        }
    }
}
