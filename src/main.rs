use clap::Parser;
use mergil::api;
use mergil::input::{self, InputResult};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Additional context or questions
    #[arg(required = false)]
    context: Vec<String>,

    /// Model to use for the API request
    #[arg(short, long, default_value = "deepseek/deepseek-coder")]
    model: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let mut contents = match input::get_input()? {
        InputResult::Content(content) => vec![content],
        InputResult::Cancelled => {
            println!("Operation cancelled.");
            return Ok(());
        }
    };

    // Add command-line arguments to contents
    contents.extend(cli.context);

    if contents.is_empty() {
        println!("No input provided. Exiting.");
        return Ok(());
    }

    println!("Input content:");
    for (i, content) in contents.iter().enumerate() {
        println!("{}. {}", i + 1, content);
    }

    let api_key = api::get_api_key();
    let client = reqwest::Client::new();
    
    match api::send_api_request(&client, &api_key, &cli.model, &contents).await {
        Ok(api_response) => {
            if let Some(choice) = api_response.choices.get(0) {
                let trimmed_content = choice.message.content.trim_start();
                println!("API Response:\n{}", trimmed_content);
            } else {
                println!("No response choices received from the API");
            }
        },
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }

    Ok(())
}
