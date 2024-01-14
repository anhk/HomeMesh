use env_logger::{Builder, Target};
mod args;
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

    match args::parse() {
        args::Command::Server(option) => server::listen(&option),
        args::Command::Client(option) => client::connect(&option),
    }
    println!("Hello, world!");
}
