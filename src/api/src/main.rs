#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_bunyan;

mod error;
mod todo;

use todo::service::todo_service::todo_service_client::TodoServiceClient;
use todo::routes::todo_filter;
use warp::Filter;
use slog::Drain;

#[tokio::main]
async fn main() {
    let todos_addr = "http://[::1]:50051";
    let client = match TodoServiceClient::connect(todos_addr).await {
        Ok(v) => v,
        Err(err) => {
            println!("{}", err.to_string());
            std::process::exit(1);
        }
    };

    let log = get_logger();

    info!(log, "starting";);

    let health_route = warp::path("health").map(|| "OK");
    let todo_filter = todo_filter(log.clone(), client);
    let routes =
        health_route
            .or(todo_filter)
            .recover(error::handle_rejection)
            .with(warp::log::custom(move |info| {
                info!(log, "handled request"; "method" => info.method().as_str(), "path" => info.path(), "status" => info.status().as_str());
            }));

    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}

fn get_logger() -> slog::Logger {
    let drain = slog_bunyan::default(std::io::stderr())
        .fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    slog::Logger::root(drain, o!("version" => env!("CARGO_PKG_VERSION"), "service" => "todo"))
}