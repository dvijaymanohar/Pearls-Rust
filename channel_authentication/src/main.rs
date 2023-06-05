
use tonic::transport::{Certificate, Channel, Identity, Server, ServerTlsConfig};
use tonic::{Request, Response, Status};
use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::{HelloRequest, HelloResponse};

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

#[derive(Default)]
pub struct GreeterService {}

#[tonic::async_trait]
impl Greeter for GreeterService {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloResponse>, Status> {
        println!("Received a request: {:?}", request);

        let response = hello_world::HelloResponse {
            message: format!("Hello, {}!", request.get_ref().name),
        };

        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load server certificate and private key
    let server_identity = Identity::from_pem(
        include_bytes!("path_to_server_certificate.pem"),
        include_bytes!("path_to_server_private_key.pem"),
    )?;

    // Load client CA certificate
    let client_ca_cert = Certificate::from_pem(include_bytes!("path_to_client_ca_cert.pem"))?;

    // Create a gRPC server with SSL/TLS configuration
    let server = GreeterServer::new(GreeterService::default())
        .tls_config(tonic::transport::ServerTlsConfig::new()
            .identity(server_identity)
            .client_ca_root(client_ca_cert))?;

    // Bind the server to a specific address
    let addr = "127.0.0.1:50051".parse()?;
    server.serve(addr).await?;

    Ok(())
}
