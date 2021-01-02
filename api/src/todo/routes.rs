use tonic::transport::Channel;
use warp::Filter;

use crate::todo::handlers;
use crate::todo::models;
use crate::todo::service::todo_service::todo_service_client::TodoServiceClient;

#[derive(Clone)]
pub(crate) struct Server {
    pub logger: slog::Logger,
    pub todo_client: TodoServiceClient<Channel>,
}

pub fn todo_filter(
    logger: slog::Logger,
    client: TodoServiceClient<Channel>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let server = Server {
        logger,
        todo_client: client,
    };

    list_todos(server.clone())
        .or(get_todo(server.clone()))
        .or(create_todo(server.clone()))
        .or(update_todo(server.clone()))
        .or(delete_todo(server.clone()))
        .or(complete_todo(server.clone()))
}

fn list_todos(
    server: Server,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("todos")
        .and(warp::get())
        .and(with_server(server))
        .and_then(handlers::list_todos)
}

fn create_todo(
    server: Server,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("todos")
        .and(warp::post())
        .and(json_create_body())
        .and(with_server(server))
        .and_then(handlers::create_todo)
}

fn get_todo(
    server: Server,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("todos" / String)
        .and(warp::get())
        .and(with_server(server))
        .and_then(handlers::get_todo)
}

fn update_todo(
    server: Server,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("todos" / String)
        .and(warp::put())
        .and(json_update_body())
        .and(with_server(server))
        .and_then(handlers::update_todo)
}

fn delete_todo(
    server: Server,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("todos" / String)
        .and(warp::delete())
        .and(with_server(server))
        .and_then(handlers::delete_todo)
}

fn complete_todo(
    server: Server,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("todos" / String / "complete")
        .and(warp::post())
        .and(with_server(server))
        .and_then(handlers::complete_todo)
}

fn with_server(
    server: Server,
) -> impl Filter<Extract = (Server,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || server.clone())
}

fn json_create_body(
) -> impl Filter<Extract = (models::CreateTodo,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

fn json_update_body(
) -> impl Filter<Extract = (models::UpdateTodo,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}
