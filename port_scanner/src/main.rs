mod cli;
mod config;
mod result;
mod scanner;

use clap::Parser;

use cli::Cli;
use config::Config;
use result::PortState;
use scanner::engine;

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    let config = Config::new(
        args.target,
        args.ports,
        args.concurrency,
        args.scan_type,
    );

    let results = engine::run_scan(&config).await;

    for r in results {
        match r.state {
            PortState::Open => println!("Port {} open", r.port),
            PortState::Closed => println!("Port {} closed", r.port),
            PortState::Filtered => println!("Port {} filtered", r.port),
        }
    }
}
