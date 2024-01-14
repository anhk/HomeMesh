use env_logger::{Builder, Target};
mod cli;
mod client;
mod server;

#[tokio::main]
async fn main() {
    // env_logger init
    let mut log_builder = Builder::from_default_env();
    log_builder
        .target(Target::Stdout)
        .filter(None, log::LevelFilter::Debug)
        .init();
    // env_logger::init();

    let args = cli::parse();
    match args {
        cli::Command::Server(option) => {
            server::listen(&option);
        }
        cli::Command::Client(option) => {
            client::connect(&option);
        }
    }
    println!("Hello, world!");
}
