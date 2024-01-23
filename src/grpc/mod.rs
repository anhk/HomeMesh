use std::io::{self};

pub trait Stream {
    fn run();
}

pub struct MyStream {}
impl Stream for MyStream {
    fn run() {}
}

pub fn alloc_grpc_server() -> Result<impl Stream, io::Error> {
    Ok(MyStream {})
}

pub fn alloc_grpc_client() -> Result<impl Stream, io::Error> {
    Ok(MyStream {})
}
