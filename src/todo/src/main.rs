use tonic::{transport::Server};

use server::todo_service::todo_service_server::TodoServiceServer;
use server::TodoServiceImpl;

mod server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // defining address for our service
    let addr = "[::1]:50051".parse().unwrap();
    // creating a service
    let service = TodoServiceImpl::new();
    println!("Server listening on {}", addr);
    // adding our service to our server.
    Server::builder()
        .add_service(TodoServiceServer::new(service))
        .serve(addr)
        .await?;
    Ok(())
}