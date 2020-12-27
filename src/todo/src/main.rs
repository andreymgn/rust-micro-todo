mod server;

use tonic::{transport::Server};
use server::TodoServiceImpl;
use server::todo_service::todo_service_server::TodoServiceServer;


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