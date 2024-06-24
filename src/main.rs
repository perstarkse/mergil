use clap::Parser;
use mergil::api;
use mergil::input::{self, InputResult};
use mergil::markdown;
use tokio_stream::StreamExt;

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

    let mut contents = match input::get_input()? {
        InputResult::Content(content) => vec![content],
        InputResult::Cancelled => {
            if cli.debug {
                println!("Operation cancelled.");
            }
            return Ok(());
        }
    };

    contents.extend(cli.context);

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
    let mut stream = api::send_api_request(&client, &api_key, &cli.model, &contents, cli.no_markdown, cli.stream).await?;

    while let Some(chunk) = stream.next().await {
        match chunk {
            Ok(content) => {
                markdown::render_markdown(&content);
                // if !cli.no_markdown && markdown::is_markdown(&content) {
                //     markdown::render_markdown(&content);
                // } else {
                //     println!("NO MARKDOWN");
                //     print!("{}", content);
                //     io::stdout().flush().unwrap();
                // }
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                break;
            }
        }
    }

    Ok(())
}
