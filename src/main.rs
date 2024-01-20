use env_logger::{Builder, Target};
mod args;
mod client;
mod server;
mod tun;
use std::io::Read;

#[tokio::main]
async fn main() {
    // env_logger init
    let mut log_builder = Builder::from_default_env();
    log_builder
        .target(Target::Stdout)
        .filter(None, log::LevelFilter::Debug)
        .init();
    // env_logger::init();

    let mut data = [0u8; 2048];
    let mut tun = tun::alloc_tun().unwrap();

    while match tun.read(&mut data) {
        Ok(size) => {
            println!("read {} from tun", size);
            true
        }
        Err(_e) => false,
    } {}

    match args::parse() {
        args::Command::Server(option) => server::listen(&option),
        args::Command::Client(option) => client::connect(&option),
    }
    println!("Hello, world!");
}
