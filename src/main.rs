use clap::Parser;
use mergil::common::Cli;
use mergil::run;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    run(Cli::parse()).await
}
