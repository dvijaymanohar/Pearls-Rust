// service.rs

use crate::mypackage::hello_world_server::{HelloWorld, HelloWorldServer};
use crate::mypackage::{HelloRequest, HelloResponse};
use crate::singleton::Singleton;
use tonic::{Request, Response, Status};

pub struct MyHelloWorld {
    singleton: Arc<Mutex<Singleton>>,
}

impl MyHelloWorld {
    pub fn new(singleton: Arc<Mutex<Singleton>>) -> Self {
        MyHelloWorld { singleton }
    }
}

#[tonic::async_trait]
impl HelloWorld for MyHelloWorld {
    async fn hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloResponse>, Status> {
        let name = request.into_inner().name;

        // Access and modify the shared state
        // For example:
        // let mut singleton = self.singleton.lock().unwrap();
        // singleton.increment_counter();

        let reply = HelloResponse {
            message: format!("Hello, {}!", name),
        };

        Ok(Response::new(reply))
    }
}

pub fn create_server(singleton: Arc<Mutex<Singleton>>) -> HelloWorldServer<MyHelloWorld> {
    HelloWorldServer::new(MyHelloWorld::new(singleton))
}
