extern crate config;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_bunyan;
#[macro_use]
extern crate async_trait;

use std::str::FromStr;

use slog::Drain;
use tonic::transport::Server;

use server::todo_service::todo_service_server::TodoServiceServer;
use server::TodoServiceImpl;

mod repository;
mod server;
mod settings;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let todo_settings = settings::Settings::new()?;

    let log_level =
        slog::Level::from_str(todo_settings.log_level.as_str()).expect("failed to parse log level");
    let log = get_logger(log_level);

    let addr = format!("0.0.0.0:{}", todo_settings.port)
        .parse()
        .expect("failed to parse socket address");
    let repo = repository::repository::get_repository(todo_settings.storage, log.clone())?;
    let service = TodoServiceImpl::new(log.clone(), repo);
    info!(log, "started"; "addr" => addr);
    Server::builder()
        .add_service(TodoServiceServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}

fn get_logger(log_level: slog::Level) -> slog::Logger {
    let drain = slog_bunyan::default(std::io::stderr())
        .filter_level(log_level)
        .fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    slog::Logger::root(
        drain,
        o!("version" => env!("CARGO_PKG_VERSION"), "service" => "todo"),
    )
}
