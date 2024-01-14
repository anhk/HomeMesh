use crate::{client, server};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author = "Terry.AN", version, about, long_about = None)]
#[command(
    help_template = "{about-section}@Author: {author}, Version: {version}\n\n{usage-heading}\n  {usage}\n\n{all-args}{tab}"
)]
/// Build a mesh network for your home lab
pub struct Arguments {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Server(server::ServerOption),
    Client(client::ClientOption),
}

pub fn parse() -> Command {
    let args = Arguments::parse();
    args.command
}
