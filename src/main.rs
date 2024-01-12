mod cli;
mod client;
mod server;

fn main() {
    let args = cli::parse();

    match args {
        cli::Command::Server(_server) => {}
        cli::Command::Client(_client) => {}
    }
    println!("Hello, world!");
}
