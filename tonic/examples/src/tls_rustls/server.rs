/*
 * It sets up a gRPC server with TLS encryption and serves the UnaryEcho service. It demonstrates
 * how to handle incoming requests and extract connection information.
 */

/*
 * This code defines a module named pb and includes the protobuf definitions located in the
 * /grpc.examples.unaryecho package.
 *
 * The pb module contains the generated code for the protocol
 * buffers defined in the /grpc.examples.unaryecho package.
 *
 * Protocol buffer definitions for the Echo service.
 */
pub mod pb {
    tonic::include_proto!("/grpc.examples.unaryecho");
}

/*
 * This section imports the necessary dependencies for the code, including the Hyper HTTP library,
 * protobuf-generated types (EchoRequest, EchoResponse) from the pb module, synchronization
 * primitives (Arc), networking components from Tokio, Rustls TLS-related types (Certificate,
 * PrivateKey, ServerConfig, TlsAcceptor), gRPC-related types (Server, Request, Response, Status),
 * and additional utilities from tower_http.
 */

// Import Hyper HTTP library
use hyper::server::conn::Http;

// protobuf-generated types (EchoRequest, EchoResponse) from the pb module
use pb::{EchoRequest, EchoResponse};

/*
 * synchronization primitives (Arc)  This line imports the Arc (atomic reference count) type from
 * the std::sync module. Arc is used for creating shared ownership of data across multiple threads.
 */
use std::sync::Arc;

/*
 * This line imports the TcpListener struct from the tokio crate. TcpListener is used for accepting
 * TCP connections.
 */
use tokio::net::TcpListener;

/*
 * This line imports various types from the tokio_rustls crate, which provides TLS functionality for
 * the server. These types include Certificate, PrivateKey, ServerConfig, and TlsAcceptor.
 */
use tokio_rustls::{
    rustls::{Certificate, PrivateKey, ServerConfig},
    TlsAcceptor,
};

/*
 * This line imports several types from the tonic crate, which is a gRPC implementation for Rust.
 * These types include Server for creating a gRPC server, Request and Response for handling gRPC
 * requests and responses, and Status for representing gRPC status codes.
 */
use tonic::{transport::Server, Request, Response, Status};

/**
 * This trait provides additional functionality for building HTTP services.
 */
use tower_http::ServiceBuilderExt;

/**
 * The main function is marked with the #[tokio::main] attribute, indicating it is the entry point
 * of the application. It sets up the gRPC server and handles incoming requests. This is the entry
 * point of the program, using the tokio runtime. It returns a Result indicating success or an error.
 *
 * This attribute macro from the tokio crate marks the following async function as the entry point
 * of the program.
 */
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /*
     * This function reads TLS certificate and key files from the specified directory. This line
     * constructs a PathBuf pointing to the data directory relative to the project's manifest
     * directory.
     *
     * CARGO_MANIFEST_DIR is an environment variable that contains the path to the manifest
     * directory of the current Rust project.
     */
    let data_dir = std::path::PathBuf::from_iter([std::env!("CARGO_MANIFEST_DIR"), "data"]);

    /*
     * This code opens the server.pem file in the tls subdirectory of the data directory and reads
     * its contents. This block of code loads the server's TLS certificate from the file tls/server.pem
     */
    let certs = {
        let fd = std::fs::File::open(data_dir.join("tls/server.pem"))?;

        let mut buf = std::io::BufReader::new(&fd);

        /*
         * It uses the rustls_pemfile crate to parse the PEM file and extract the certificates which
         * are converted into a Vec<Certificate>. These files are necessary to configure the TLS
         * settings for the server.
         *
         * Certificates extracted by the rustls_pemfile crate are then collected into a vector of
         * Certificate objects.
         */
        rustls_pemfile::certs(&mut buf)?
            .into_iter()
            .map(Certificate)
            .collect()
    };

    /*
     * Similar to the previous code, this section opens the server.key file, reads its contents,
     * and uses rustls_pemfile to parse the PEM file and extract the private key. The private key is
     * then converted into a PrivateKey.
     *
     * This block of code loads the server's private key from the file tls/server.key
     */
    let key = {
        let fd = std::fs::File::open(data_dir.join("tls/server.key"))?;
        let mut buf = std::io::BufReader::new(&fd);

        /*
         * extracts the private key using the rustls_pemfile crate. The private key is then stored
         * in a PrivateKey object.
         */
        rustls_pemfile::pkcs8_private_keys(&mut buf)?
            .into_iter()
            .map(PrivateKey)
            .next()
            .unwrap()

        // let key = std::fs::read(data_dir.join("tls/server.key"))?;
        // PrivateKey(key)
    };

    /*
     * The TLS configuration is built using ServerConfig::builder(). The server is configured with
     * safe defaults, no client authentication (with_no_client_auth()), and a single
     * certificate (with_single_cert()).
     */

    /*
     * This line creates/constructs a ServerConfig object for TLS configuration
     * The with_safe_defaults() method sets the default configuration values for secure TLS communication.
     * The with_no_client_auth() method disables client authentication.
     * The with_single_cert() method sets the server certificate and private key for the TLS configuration.
     */
    let mut tls = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(certs, key)?;

    /*
     * The ALPN (Application-Layer Protocol Negotiation) protocols are set to only support HTTP/2 (h2).
     * This line sets the Application-Layer Protocol Negotiation (ALPN) protocols for the TLS
     * configuration. Here, only the "h2" protocol is specified, indicating HTTP/2.
     */
    tls.alpn_protocols = vec![b"h2".to_vec()];

    /*
     * These lines create an instance of the EchoServer struct, which implements the gRPC service
     * and wrap it with a pb::echo_server::EchoServer.
     */
    let server = EchoServer::default();

    /*
     * The Server::builder() creates a new gRPC server builder.
     * The add_service() method adds the EchoServer service implementation to the server.
     * The into_service() method converts the server builder into a Service instance.
     */
    let svc = Server::builder()
        .add_service(pb::echo_server::EchoServer::new(server))
        .into_service();

    /*
     * This code creates an HTTP server instance using Hyper and configures it to only support HTTP/2.
     */
    let mut http = Http::new();
    http.http2_only(true);

    /*
     * These lines bind the server to a specific TCP listener address and create a TlsAcceptor from
     * the configured TLS settings. This line binds a TCP listener to the address [::1]:50051,
     * which represents the loopback address on port 50051. The listener will accept incoming TCP
     * connections.
     */
    let listener = TcpListener::bind("[::1]:50051").await?;

    /*
     * This line creates a TlsAcceptor from the configured TLS server configuration.
     * The Arc::new() function is used to wrap the TLS configuration in an atomic reference count
     * for shared ownership across multiple threads.
     */
    let tls_acceptor = TlsAcceptor::from(Arc::new(tls));

    /*
     * This line starts an infinite loop for accepting incoming connections.
     */
    loop {
        /*
         * This line awaits an incoming TCP connection using the accept() method of the TcpListener.
         * It either returns the accepted connection and the client's address or an error.
         */
        let (conn, addr) = match listener.accept().await {
            Ok(incoming) => incoming,
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
                continue;
            }
        };

        /*
         * These lines create clones of the http, tls_acceptor, and svc variables for use in the
         * spawned async task. This is necessary to ensure that each task operates independently.
         */
        let http = http.clone();
        let tls_acceptor = tls_acceptor.clone();
        let svc = svc.clone();

        /*
         * This line spawns a new async task using the tokio::spawn() function. The task handles the
         * incoming connection asynchronously.
         */
        tokio::spawn(async move {
            let mut certificates = Vec::new();

            /*
             * These lines initialize a vector to store the client's certificates and use the
             * TlsAcceptor to perform the TLS handshake with the client. The client's certificates
             * are extracted and stored in the certificates vector.
             */
            let conn = tls_acceptor
                .accept_with(conn, |info| {
                    if let Some(certs) = info.peer_certificates() {
                        for cert in certs {
                            certificates.push(cert.clone());
                        }
                    }
                })
                .await
                .unwrap();

            /*
             * This line builds a tower::Service using the ServiceBuilder. An extension ConnInfo
             * containing the client's address and certificates is added, and the svc service is
             * wrapped with this extension.
             */
            let svc = tower::ServiceBuilder::new()
                .add_extension(Arc::new(ConnInfo { addr, certificates }))
                .service(svc);

            /*
             * The http.serve_connection() function is used to serve the gRPC connection using the
             * provided service chain. This line serves the established TLS connection using the
             * http instance and the wrapped service svc.
             */

            http.serve_connection(conn, svc).await.unwrap();
        });
    }
}

/* struct to store connection information */
#[derive(Debug)]
struct ConnInfo {
    addr: std::net::SocketAddr,
    certificates: Vec<Certificate>,
}

/* EchoResult type alias for convenience */
type EchoResult<T> = Result<Response<T>, Status>;

#[derive(Default)]
pub struct EchoServer;

#[tonic::async_trait]
impl pb::echo_server::Echo for EchoServer {
    /*
     * The unary_echo function receives a gRPC request, extracts connection information, and
     * responds with an EchoResponse containing the received message.
     */
    async fn unary_echo(&self, request: Request<EchoRequest>) -> EchoResult<EchoResponse> {
        let conn_info = request.extensions().get::<Arc<ConnInfo>>().unwrap();
        println!(
            "Got a request from: {:?} with certs: {:?}",
            conn_info.addr, conn_info.certificates
        );

        let message = request.into_inner().message;
        Ok(Response::new(EchoResponse { message }))
    }
}
