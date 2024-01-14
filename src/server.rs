use std::net::{TcpListener, TcpStream};

use clap::Args;
use log::{error, info};

#[derive(Args)]
/// use server mode
pub struct ServerOption {
    /// cidr to claim
    #[arg(num_args(0..), short, long, required = false)]
    pub cidr: Vec<String>,

    /// token to authenticate
    #[arg(short, long)]
    pub token: String,

    /// bind address, it will listen on "${bind_addr}:${port}"
    #[arg(short, long, default_value_t = String::from("0.0.0.0"))]
    pub bind_addr: String,

    /// listen port, it will listen on "${bind_addr}:${port}"
    #[arg(short, long, default_value_t = 3399)]
    pub port: u16,
}

pub fn listen(option: &ServerOption) {
    let addr = format!("{}:{}", option.bind_addr, option.port);
    let listener = TcpListener::bind(&addr).unwrap();
    info!("listen on {} ok.", addr);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                info!("New connection: {}", stream.peer_addr().unwrap());
                tokio::spawn(async move { handle_client(stream).await });
            }
            Err(e) => {
                error!("accept failed: {}", e);
            }
        }
    }
}

async fn handle_client(stream: TcpStream) {
    info!("connection [{}] closed", stream.peer_addr().expect(""));
}
