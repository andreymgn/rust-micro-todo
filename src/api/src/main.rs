use todo::todo_service::todo_service_client::TodoServiceClient;
use warp::Filter;

mod todo;
mod error;

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
    let todo_filter = todo::todo_filter(client);
    let routes = health_route.or(todo_filter).recover(error::handle_rejection);

    // Start up the server...
    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}