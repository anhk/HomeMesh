use async_trait::async_trait;
use std::io::{self};

pub mod stream {
    tonic::include_proto!("stream");
}

mod client;
mod server;

#[async_trait]
pub trait GrpcStream {
    async fn run();
}

pub fn alloc_grpc_server() -> Result<impl GrpcStream, io::Error> {
    Ok(server::MyStreamServer {})
}

pub fn alloc_grpc_client() -> Result<impl GrpcStream, io::Error> {
    Ok(client::MyStreamClient {})
}
