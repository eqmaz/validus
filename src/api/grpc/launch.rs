use crate::api::grpc::myservice::{
    greeter_server::{Greeter, GreeterServer},
    HelloReply, HelloRequest,
};
use tonic::{transport::Server, Request, Response, Status};

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(&self, request: Request<HelloRequest>) -> Result<Response<HelloReply>, Status> {
        let reply = HelloReply { message: format!("Hello, {}!", request.into_inner().name) };
        Ok(Response::new(reply))
    }
}

pub fn start_grpc_server_bg() {
    tokio::spawn(async {
        let addr = "[::1]:50051".parse().expect("Invalid address");
        let greeter = MyGreeter::default();

        println!("🚀 gRPC Server listening on {}", addr);

        if let Err(e) = Server::builder().add_service(GreeterServer::new(greeter)).serve(addr).await {
            eprintln!("gRPC Server failed: {}", e);
        }
    });
}
