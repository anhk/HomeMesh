use super::stream::stream_server::Stream;
use super::stream::Message;
use async_trait::async_trait;
use tokio::sync::mpsc;
use tokio_stream::{wrappers::ReceiverStream, StreamExt};
use tonic::{Request, Response, Status, Streaming};

pub struct MyStreamClient {}

#[async_trait]
impl super::GrpcStream for MyStreamClient {
    async fn run() {
        MyStreamClient {}.connect()
    }
}

impl MyStreamClient {
    fn connect(&self) {}
}
