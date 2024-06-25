use clap::Parser;
use mergil::api;
use mergil::input::{self, InputResult};
use mergil::markdown;
use tokio_stream::StreamExt;
    use indicatif::{ProgressBar, ProgressStyle};
    use std::io::{self, Write};
    use atty::Stream;

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

    /// Disable Markdown rendering
    #[arg(long, default_value = "false")]
    no_markdown: bool,

    /// Enable streaming output
    #[arg(long, default_value = "false")]
    stream: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
        let cli = Cli::parse();

        let mut contents = Vec::new();

        // Handle command-line input
        if !cli.context.is_empty() {
            contents.push(cli.context.join(" "));
        }

        if contents.is_empty() {
            // Determine if we should force the editor to open
            let force_editor = atty::is(Stream::Stdin);

            // Handle piped input or open editor
            match input::get_input(force_editor)? {
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
    
//      match api::send_api_request(&client, &api_key, &cli.model, &contents, cli.no_markdown).await {
//         Ok(api_response) => {
//             if let Some(choice) = api_response.choices.get(0) {
//                 let trimmed_content = choice.message.content.trim_start();
//                 if !cli.no_markdown && markdown::is_markdown(trimmed_content) {
//                     markdown::render_markdown(trimmed_content);
//                 } else {
//                     println!("{}", trimmed_content);
//                 }
//             } else {
//                 if cli.debug {
//                     println!("No response choices received from the API");
//                 }
//             }
//         },
//         Err(e) => {
//             eprintln!("Error: {}", e);
//         }
//     }

//     Ok(())
// }
let mut stream = api::send_api_request(&client, &api_key, &cli.model,
  &contents, cli.no_markdown, cli.stream).await?;

        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.green} {wide_msg}")
                .unwrap()
        );

        let mut full_content = String::new();

        while let Some(chunk) = stream.next().await {
            match chunk {
                Ok(content) => {
                    full_content.push_str(&content);
                    let formatted = markdown::format_markdown(&full_content);
                    pb.set_message(formatted);
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    break;
                }
            }
        }

        pb.finish_and_clear();

        // Display the final content
        if !cli.no_markdown {
            markdown::render_markdown(&full_content);
        } else {
            println!("{}", full_content);
        }

        Ok(())
    }
