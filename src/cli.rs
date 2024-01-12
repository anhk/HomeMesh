use clap::Parser;

#[derive(Parser, Debug)]
#[command(author = "Terry.AN", version, about, long_about = None)]
#[command(help_template = "{about-section}@Author: {author}, Version: {version}\n\n{usage-heading}\n  {usage}\n\n{all-args}{tab}")]
/// Build a mesh network for your home lab
pub struct Args {
    /// the address of server:port, use client mode if set [default: <empty>]
    #[arg(short, long)]
    pub server: Option<String>,

    /// cidr to claim
    #[arg(num_args(0..), short, long, required = true)]
    pub cidr: Vec<String>,

    /// token to authenticate
    #[arg(short, long)]
    pub token: String,
}

pub fn parse() -> Args {
    Args::parse()
}
