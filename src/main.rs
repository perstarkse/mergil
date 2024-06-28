use clap::Parser;
use mergil::common::handle_input;
use mergil::common::process_contents;
use mergil::common::Cli;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let contents = handle_input(&cli).await?;
    process_contents(&cli, &contents).await?;
    Ok(())
}
