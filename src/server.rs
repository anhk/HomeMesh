use std::net::TcpListener;

use clap::Args;
use log::{error, info};

#[derive(Args)]
/// use server mode
pub struct Server {
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

impl Server {
    pub fn listen(&self) {
        let addr = format!("{}:{}", self.bind_addr, self.port);
        let listener = TcpListener::bind(&addr).unwrap();
        info!("listen on {} ok.", addr);

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    info!("New connection: {}", stream.peer_addr().unwrap());
                    // thread::spawn(|| self.handle_client(stream));
                }
                Err(e) => {
                    error!("accept failed: {}", e);
                }
            }
        }
    }

    // fn handle_client(&mut self, mut stream: TcpStream) {}
}
