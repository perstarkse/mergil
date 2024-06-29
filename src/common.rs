use crate::api;
use crate::input;
use crate::input::InputResult;
use crate::input::RealEditor;
use crate::input::RealStdin;
use crate::input::StdinReader;
use crate::markdown;
use atty::Stream;
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Additional context or questions
    #[arg(required = false)]
    pub context: Vec<String>,

    /// Model to use for the API request
    #[arg(short, long, default_value = "deepseek/deepseek-coder")]
    pub model: String,

    /// Enable debug output
    #[arg(long, default_value = "false")]
    pub debug: bool,

    /// Use Markdown rendering
    #[arg(long, default_value = "false")]
    pub markdown: bool,
}

pub async fn handle_input(cli: &Cli) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut contents = Vec::new();

    // Handle command-line input
    if !cli.context.is_empty() {
        contents.push(cli.context.join(" "));
    }

    let mut real_stdin = RealStdin;
    let real_editor = RealEditor;

    // Check for piped input
    if !atty::is(Stream::Stdin) {
        let mut piped_input = String::new();
        real_stdin.read_to_string(&mut piped_input)?;
        if !piped_input.trim().is_empty() {
            contents.push(piped_input);
        }
    }

    // If no input from args or pipe, open editor (unless NO_EDITOR is set)
    if cli.context.is_empty() && std::env::var("NO_EDITOR").is_err() {
        match input::get_input(true, &mut real_stdin, &real_editor)? {
            InputResult::Content(content) => contents.push(content),
            InputResult::Cancelled => {
                if cli.debug {
                    println!("Operation cancelled.");
                }
                return Ok(contents);
            }
        }
    }

    Ok(contents)
}

pub async fn process_contents(
    cli: &Cli,
    contents: &[String],
) -> Result<(), Box<dyn std::error::Error>> {
    if cli.debug {
        println!("Model: {}", cli.model);
        println!("Markdown: {}", cli.markdown);
        println!("Input content:");
        for (i, content) in contents.iter().enumerate() {
            println!("{}. {}", i + 1, content);
        }
    }

    if contents.is_empty() {
        if cli.debug {
            println!("No input provided. Exiting.");
        }
        return Ok(());
    }

    // Skip API call when running tests
    if std::env::var("RUST_TEST").is_err() {
        let api_key = api::get_api_key();
        let client = reqwest::Client::new();

        let response =
            api::send_api_request(&client, &api_key, &cli.model, &contents, cli.markdown, None)
                .await?;

        let skin = markdown::create_madskin();

        if cli.markdown {
            skin.print_text(&response);
        } else {
            println!("{}", response);
        }
    }

    Ok(())
}
