use atty::Stream;
use clap::Parser;
use mergil::api;
use mergil::input::{self, InputResult};
use mergil::markdown;
use std::io::{self, Read};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Additional context or questions
    #[arg(required = false)]
    context: Vec<String>,

    /// Model to use for the API request
    #[arg(short, long, default_value = "deepseek/deepseek-coder")]
    model: String,

    /// Enable debug output
    #[arg(long, default_value = "false")]
    debug: bool,

    /// Use Markdown rendering
    #[arg(long, default_value = "false")]
    markdown: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let mut contents = Vec::new();

    // Handle command-line input
    if !cli.context.is_empty() {
        contents.push(cli.context.join(" "));
    }

    // Check for piped input
    if !atty::is(Stream::Stdin) {
        let mut piped_input = String::new();
        io::stdin().read_to_string(&mut piped_input)?;
        if !piped_input.trim().is_empty() {
            contents.push(piped_input);
        }
    }

    // If no input from args or pipe, open editor
    if cli.context.is_empty() {
        match input::get_input(true)? {
            InputResult::Content(content) => contents.push(content),
            InputResult::Cancelled => {
                if cli.debug {
                    println!("Operation cancelled.");
                }
                return Ok(());
            }
        }
    }

    if contents.is_empty() {
        if cli.debug {
            println!("No input provided. Exiting.");
        }
        return Ok(());
    }

    if cli.debug {
        println!("Input content:");
        for (i, content) in contents.iter().enumerate() {
            println!("{}. {}", i + 1, content);
        }
    }

    let api_key = api::get_api_key();
    let client = reqwest::Client::new();

    let response =
        api::send_api_request(&client, &api_key, &cli.model, &contents, cli.markdown).await?;

    let skin = markdown::create_madskin();

    if cli.markdown {
        skin.print_text(&response);
    } else {
        println!("{}", response);
    }

    Ok(())
}
