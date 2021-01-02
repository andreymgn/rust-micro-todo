#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_bunyan;

use std::str::FromStr;

use slog::Drain;
use warp::Filter;

use todo::routes::todo_filter;
use todo::service::todo_service::todo_service_client::TodoServiceClient;

mod error;
mod settings;
mod todo;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_settings = settings::Settings::new()?;

    let log_level =
        slog::Level::from_str(api_settings.log_level.as_str()).expect("failed to parse log level");
    let log = get_logger(log_level);

    let client = match TodoServiceClient::connect(api_settings.todo_addr).await {
        Ok(v) => v,
        Err(err) => {
            println!("{}", err.to_string());
            std::process::exit(1);
        }
    };

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

    warp::serve(routes)
        .run(([0, 0, 0, 0], api_settings.port))
        .await;

    Ok(())
}

fn get_logger(log_level: slog::Level) -> slog::Logger {
    let drain = slog_bunyan::default(std::io::stderr())
        .filter_level(log_level)
        .fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    slog::Logger::root(
        drain,
        o!("version" => env!("CARGO_PKG_VERSION"), "service" => "api"),
    )
}
