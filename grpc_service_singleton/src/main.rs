use helloworld::greeter_server::{Greeter, GreeterServer};
use helloworld::{HelloRequest, HelloResponse};
use std::sync::{Arc, Mutex};
use tonic::{Request, Response, Status};

// Define the gRPC service
pub struct GreeterService {
    // Add any necessary fields here
}

impl GreeterService {
    // Create a new instance of the service
    pub fn new() -> Self {
        GreeterService {
            // Initialize fields as needed
        }
    }
}

// Implement the gRPC service trait
#[tonic::async_trait]
impl Greeter for GreeterService {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloResponse>, Status> {
        let name = request.into_inner().name;

        // Perform any necessary logic
        let message = format!("Hello, {}!", name);

        // Create the response
        let response = HelloResponse {
            message: message.clone(),
        };

        Ok(Response::new(response))
    }
}

// Create a singleton instance of the service
lazy_static::lazy_static! {
    static ref GREETER_SERVICE: Arc<Mutex<GreeterService>> = Arc::new(Mutex::new(GreeterService::new()));
}

// Define the main function to start the gRPC server
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse()?;
    let greeter_service = GREETER_SERVICE.clone();

    // Create the gRPC server
    let server = GreeterServer::builder()
        .add_service(GreeterServer::new(greeter_service))
        .serve(addr);

    println!("Server listening on {}", addr);

    // Start the server
    server.await?;

    Ok(())
}
