use common::{handle_input, process_contents, Cli};

pub mod api;
pub mod common;
pub mod input;
pub mod markdown;

pub async fn run(cli: Cli) -> Result<(), Box<dyn std::error::Error>> {
    let contents = handle_input(&cli).await?;
    process_contents(&cli, &contents).await?;
    Ok(())
}
