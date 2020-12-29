#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_bunyan;

use slog::Drain;
use tonic::{transport::Server};

use server::todo_service::todo_service_server::TodoServiceServer;
use server::TodoServiceImpl;

mod server;
mod error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let log = get_logger();
    let addr = "[::1]:50051".parse().unwrap();

    info!(log, "started"; "addr" => addr);

    let service = TodoServiceImpl::new(log);

    Server::builder()
        .add_service(TodoServiceServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}

fn get_logger() -> slog::Logger {
    let drain = slog_bunyan::default(std::io::stderr())
        .fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    slog::Logger::root(drain, o!("version" => env!("CARGO_PKG_VERSION"), "service" => "todo"))
}
