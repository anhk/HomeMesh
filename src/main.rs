mod cli;

fn main() {
    let args: cli::Args = cli::parse();

    if args.server != None {
        println!("{}", args.server.expect(""));
    }

    println!("Hello, world!");
}
