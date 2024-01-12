use clap::{Parser, Subcommand};
use crate::{client, server};

#[derive(Parser)]
#[command(author = "Terry.AN", version, about, long_about = None)]
#[command(
    help_template = "{about-section}@Author: {author}, Version: {version}\n\n{usage-heading}\n  {usage}\n\n{all-args}{tab}"
)]
/// Build a mesh network for your home lab
pub struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Server(server::Server),
    Client(client::Client),
}

pub fn parse() -> Command {
    let cli = Cli::parse();
    cli.command
}
