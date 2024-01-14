use std::net::TcpStream;

use clap::Args;
use log::info;

#[derive(Args)]
/// use client mode
pub struct ClientOption {
    /// the address of server, it will connect ${server_addr}:${port} [default: <empty>],
    #[arg(short, long, default_value_t = String::from("127.0.0.1"))]
    pub server_addr: String,

    /// the port of server, it will connect ${server_addr}:${port}
    #[arg(short, long, default_value_t = 3399)]
    pub port: u16,

    /// cidr to claim
    #[arg(num_args(0..), short, long, required = false)]
    pub cidr: Vec<String>,

    /// token to authenticate
    #[arg(short, long)]
    pub token: String,
}

pub fn connect(option: &ClientOption) {
    let addr = format!("{}:{}", option.server_addr, option.port);
    info!("connect to {}", addr);
    let stream = TcpStream::connect(addr).expect("connect failed");
    stream.set_nodelay(true).expect("set nodelay failed");
}
