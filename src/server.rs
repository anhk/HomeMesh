use clap::Args;

#[derive(Args)]
/// use server mode
pub struct Server {
    /// cidr to claim
    #[arg(num_args(0..), short, long, required = true)]
    pub cidr: Vec<String>,

    /// token to authenticate
    #[arg(short, long)]
    pub token: String,
}
