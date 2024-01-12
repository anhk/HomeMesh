use clap::Args;

#[derive(Args)]
/// use client mode
pub struct Client {
    /// the address of server:port, use client mode if set [default: <empty>]
    #[arg(short, long)]
    pub server: String,

    /// cidr to claim
    #[arg(num_args(0..), short, long, required = true)]
    pub cidr: Vec<String>,

    /// token to authenticate
    #[arg(short, long)]
    pub token: String,
}
