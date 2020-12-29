mod error;
mod todo;

use todo::service::todo_service::todo_service_client::TodoServiceClient;
use todo::routes::todo_filter;
use warp::Filter;

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

    let health_route = warp::path("health").map(|| "OK");
    let todo_filter = todo_filter(client);
    let routes = health_route.or(todo_filter).recover(error::handle_rejection);

    // Start up the server...
    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}