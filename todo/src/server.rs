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

        let todos = self.repo.list().await?;

        Ok(tonic::Response::new(todos.into()))
    }

    async fn create(
        &self,
        request: tonic::Request<pb::CreateRequest>,
    ) -> Result<tonic::Response<pb::Todo>, tonic::Status> {
        debug!(self.logger, "create");

        let todo = self
            .repo
            .create(
                request.get_ref().title.clone(),
                request.get_ref().body.clone(),
            )
            .await?;

        Ok(tonic::Response::new(todo.into()))
    }

    async fn get_by_id(
        &self,
        request: tonic::Request<pb::TodoId>,
    ) -> Result<tonic::Response<pb::Todo>, tonic::Status> {
        debug!(self.logger, "get_by_id";);

        let id = &request.get_ref().id;
        let todo = self.repo.get(id).await?;

        Ok(tonic::Response::new(todo.into()))
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
        let todo = self.repo.update(id, title, body, is_completed).await?;

        Ok(tonic::Response::new(todo.into()))
    }

    async fn delete(
        &self,
        request: tonic::Request<pb::TodoId>,
    ) -> Result<tonic::Response<()>, tonic::Status> {
        debug!(self.logger, "delete";);

        let id = &request.get_ref().id.clone();
        self.repo.delete(id).await?;

        Ok(tonic::Response::new(()))
    }

    async fn complete(
        &self,
        request: tonic::Request<pb::TodoId>,
    ) -> Result<tonic::Response<pb::Todo>, tonic::Status> {
        debug!(self.logger, "complete";);

        let id = &request.get_ref().id;
        let todo = self.repo.complete(id).await?;

        Ok(tonic::Response::new(todo.into()))
    }
}
