use warp::http::StatusCode;
use warp::reject;

use super::super::error::Error::RPCError;
use crate::todo::service::todo_service as pb;
use crate::todo::models;
use crate::todo::routes::Server;

pub(crate) async fn list_todos(mut server: Server) -> Result<impl warp::Reply, warp::Rejection> {
    let req = tonic::Request::new(pb::ListRequest {});
    let resp = server.todo_client.list(req).await.map_err(|e| {
        error!(server.logger, "list_todos"; "err" => e.to_string());
        reject::custom(RPCError(e))
    })?;

    let body: models::Todos = models::Todos::from(resp.into_inner());

    Ok(warp::reply::json(&body))
}

pub(crate) async fn create_todo(create: models::CreateTodo, mut server: Server) -> Result<impl warp::Reply, warp::Rejection> {
    let req = tonic::Request::new(pb::CreateRequest {
        title: create.title,
        body: create.body,
    });
    let resp = server.todo_client.create(req).await.map_err(|e| {
        error!(server.logger, "create_todo"; "err" => e.to_string());
        reject::custom(RPCError(e))
    })?;

    let body: models::Todo = models::Todo::from(resp.into_inner());

    Ok(warp::reply::json(&body))
}

pub(crate) async fn get_todo(id: String, mut server: Server) -> Result<impl warp::Reply, warp::Rejection> {
    let req = tonic::Request::new(pb::TodoId { id: id.clone() });
    let resp = server.todo_client.get_by_id(req).await.map_err(|e| {
        error!(server.logger, "get_todo"; "err" => e.to_string(), "id" => id);
        reject::custom(RPCError(e))
    })?;

    let body: models::Todo = models::Todo::from(resp.into_inner());

    Ok(warp::reply::json(&body))
}

pub(crate) async fn update_todo(id: String, update: models::UpdateTodo, mut server: Server) -> Result<impl warp::Reply, warp::Rejection> {
    let req = tonic::Request::new(pb::UpdateRequest {
        id: id.clone(),
        title: update.title,
        body: update.body,
        is_completed: update.is_completed,
    });
    let resp = server.todo_client.update(req).await.map_err(|e| {
        error!(server.logger, "update_todo"; "err" => e.to_string(), "id" => id);
        reject::custom(RPCError(e))
    })?;

    let body: models::Todo = models::Todo::from(resp.into_inner());

    Ok(warp::reply::json(&body))
}

pub(crate) async fn delete_todo(id: String, mut server: Server) -> Result<impl warp::Reply, warp::Rejection> {
    let req = tonic::Request::new(pb::TodoId { id: id.clone() });
    server.todo_client.delete(req).await.map_err(|e| {
        error!(server.logger, "delete_todo"; "err" => e.to_string(), "id" => id);
        reject::custom(RPCError(e))
    })?;

    Ok(StatusCode::NO_CONTENT)
}

pub(crate) async fn complete_todo(id: String, mut server: Server) -> Result<impl warp::Reply, warp::Rejection> {
    let req = tonic::Request::new(pb::TodoId { id: id.clone() });
    let resp = server.todo_client.complete(req).await.map_err(|e| {
        error!(server.logger, "complete_todo"; "err" => e.to_string(), "id" => id);
        reject::custom(RPCError(e))
    })?;

    let body = models::Todo::from(resp.into_inner());

    Ok(warp::reply::json(&body))
}
