use tonic::transport::Channel;
use warp::Filter;

use crate::todo::service::todo_service::todo_service_client::TodoServiceClient;
use crate::todo::handlers;
use crate::todo::models;

pub fn todo_filter(
    client: TodoServiceClient<Channel>
) -> impl Filter<Extract=impl warp::Reply, Error=warp::Rejection> + Clone {
    list_todos(client.clone())
        .or(get_todo(client.clone()))
        .or(create_todo(client.clone()))
        .or(update_todo(client.clone()))
        .or(delete_todo(client.clone()))
        .or(complete_todo(client.clone()))
}

fn list_todos(
    client: TodoServiceClient<Channel>
) -> impl Filter<Extract=impl warp::Reply, Error=warp::Rejection> + Clone {
    warp::path!("todos")
        .and(warp::get())
        .and(with_client(client))
        .and_then(handlers::list_todos)
}

fn create_todo(
    client: TodoServiceClient<Channel>
) -> impl Filter<Extract=impl warp::Reply, Error=warp::Rejection> + Clone {
    warp::path!("todos")
        .and(warp::post())
        .and(json_create_body())
        .and(with_client(client))
        .and_then(handlers::create_todo)
}

fn get_todo(
    client: TodoServiceClient<Channel>
) -> impl Filter<Extract=impl warp::Reply, Error=warp::Rejection> + Clone {
    warp::path!("todos" / String)
        .and(warp::get())
        .and(with_client(client))
        .and_then(handlers::get_todo)
}

fn update_todo(
    client: TodoServiceClient<Channel>
) -> impl Filter<Extract=impl warp::Reply, Error=warp::Rejection> + Clone {
    warp::path!("todos" / String)
        .and(warp::put())
        .and(json_update_body())
        .and(with_client(client))
        .and_then(handlers::update_todo)
}

fn delete_todo(
    client: TodoServiceClient<Channel>
) -> impl Filter<Extract=impl warp::Reply, Error=warp::Rejection> + Clone {
    warp::path!("todos" / String)
        .and(warp::delete())
        .and(with_client(client))
        .and_then(handlers::delete_todo)
}

fn complete_todo(
    client: TodoServiceClient<Channel>
) -> impl Filter<Extract=impl warp::Reply, Error=warp::Rejection> + Clone {
    warp::path!("todos" / String / "complete")
        .and(warp::post())
        .and(with_client(client))
        .and_then(handlers::complete_todo)
}

fn with_client(
    client: TodoServiceClient<Channel>
) -> impl Filter<Extract=(TodoServiceClient<Channel>, ), Error=std::convert::Infallible> + Clone {
    warp::any().map(move || client.clone())
}

fn json_create_body() -> impl Filter<Extract=(models::CreateTodo, ), Error=warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

fn json_update_body() -> impl Filter<Extract=(models::UpdateTodo, ), Error=warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}