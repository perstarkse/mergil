use clap::Parser;
use mergil::api;
use mergil::input::{self, InputResult};
use tokio_stream::StreamExt;
use bat::PrettyPrinter;
use std_prelude::IoWrite;
    use indicatif::{ProgressBar, ProgressStyle};
    use std::io::{self, Read};
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

  pb.set_style(ProgressStyle::default_spinner().template("{spinner:.green}
  Generating...").unwrap());

        let mut output = String::new();

        while let Some(chunk) = stream.next().await {
            match chunk {
                Ok(content) => {
                    output.push_str(&content);
                    if cli.stream {
                        print!("{}", content);
                        io::stdout().flush()?;
                    }
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    break;
                }
            }
        }

        pb.finish_and_clear();

        if !cli.stream {
            if !cli.no_markdown {
                render_markdown(&output);
            } else {
                println!("{}", output);
            }
        }

        Ok(())
    }

    fn render_markdown(content: &str) {
        let mut printer = PrettyPrinter::new();
        printer
            .input_from_bytes(content.as_bytes())
            .language("markdown")
            .print()
            .unwrap();
    }
