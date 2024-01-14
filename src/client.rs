use clap::Args;

#[derive(Args)]
/// use client mode
pub struct Client {
    /// the address of server, it will connect ${server_addr}:${port} [default: <empty>],
    #[arg(short, long)]
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

impl Client {
    pub fn connect(&self) {}
}
