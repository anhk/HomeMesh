use async_trait::async_trait;
use std::net::ToSocketAddrs;

use super::stream::stream_server::{Stream, StreamServer};
use super::stream::Message;
use tokio::sync::mpsc;
use tokio_stream::{wrappers::ReceiverStream, StreamExt};
use tonic::{transport::Server, Request, Response, Status, Streaming};

pub struct MyStreamServer {}

#[async_trait]
impl super::GrpcStream for MyStreamServer {
    async fn run(&self) {
        // Server::builder()
        //     .add_service(StreamServer::new(self))
        //     .serve("[::1]:50051".to_socket_addrs().unwrap().next().unwrap())
        //     .await
        //     .unwrap();
    }
}

#[tonic::async_trait]
impl Stream for MyStreamServer {
    type PersistentStreamStream = ReceiverStream<Result<Message, Status>>;

    async fn persistent_stream(
        &self,
        request: Request<Streaming<Message>>,
    ) -> Result<Response<Self::PersistentStreamStream>, Status> {
        let mut in_stream = request.into_inner();
        let (tx, rx) = mpsc::channel(2048);

        tokio::spawn(async move {
            while let Some(msg) = in_stream.next().await {
                match msg {
                    Ok(v) => {
                        println!("receive: {:?}", v.code);
                        tx.send(Ok(v)).await.unwrap_or_default();
                    }
                    Err(_e) => break,
                }
            }
        });

        let out_stream = ReceiverStream::new(rx);
        Ok(Response::new(out_stream as Self::PersistentStreamStream))
    }
}
