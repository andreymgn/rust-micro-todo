use tonic::transport::Channel;
use warp::http::StatusCode;
use warp::reject;

use super::models;
use super::super::error::Error::RPCError;
use super::todo_service as pb;
use super::todo_service::todo_service_client::TodoServiceClient;

pub async fn list_todos(mut client: TodoServiceClient<Channel>) -> Result<impl warp::Reply, warp::Rejection> {
    let req = tonic::Request::new(pb::ListRequest {});
    let resp = client.list(req).await.map_err(|e| reject::custom(RPCError(e)))?;

    let body: models::Todos = models::Todos::from(resp.into_inner());

    Ok(warp::reply::json(&body))
}

pub async fn create_todo(create: models::CreateTodo, mut client: TodoServiceClient<Channel>) -> Result<impl warp::Reply, warp::Rejection> {
    let req = tonic::Request::new(pb::CreateRequest {
        title: create.title,
        body: create.body,
    });
    let resp = client.create(req).await.map_err(|e| reject::custom(RPCError(e)))?;

    let body: models::Todo = models::Todo::from(resp.into_inner());

    Ok(warp::reply::json(&body))
}

pub async fn get_todo(id: String, mut client: TodoServiceClient<Channel>) -> Result<impl warp::Reply, warp::Rejection> {
    let req = tonic::Request::new(pb::TodoId { id });
    let resp = client.get_by_id(req).await.map_err(|e| reject::custom(RPCError(e)))?;

    let body: models::Todo = models::Todo::from(resp.into_inner());

    Ok(warp::reply::json(&body))
}

pub async fn update_todo(id: String, update: models::UpdateTodo, mut client: TodoServiceClient<Channel>) -> Result<impl warp::Reply, warp::Rejection> {
    let req = tonic::Request::new(pb::UpdateRequest {
        id,
        title: update.title,
        body: update.body,
        is_completed: update.is_completed,
    });
    let resp = client.update(req).await.map_err(|e| reject::custom(RPCError(e)))?;

    let body: models::Todo = models::Todo::from(resp.into_inner());

    Ok(warp::reply::json(&body))
}

pub async fn delete_todo(id: String, mut client: TodoServiceClient<Channel>) -> Result<impl warp::Reply, warp::Rejection> {
    let req = tonic::Request::new(pb::TodoId { id });
    client.delete(req).await.map_err(|e| reject::custom(RPCError(e)))?;

    Ok(StatusCode::NO_CONTENT)
}
