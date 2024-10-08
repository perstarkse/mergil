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
    #[arg(short, long, default_value = "anthropic/claude-3.5-sonnet")]
    pub model: String,

    /// Model to use for the simpler thinking
    #[arg(short, long, default_value = "meta-llama/llama-3.1-405b")]
    pub cheap_model: String,

    /// Enable debug output
    #[arg(long, default_value = "false")]
    pub debug: bool,

    /// Use Markdown rendering
    #[arg(long, default_value = "false")]
    pub markdown: bool,

    /// Enable pre-processing mode
    #[arg(long, default_value = "false")]
    pub preprocess: bool,
}

pub async fn handle_input(cli: &Cli) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut contents = if !cli.context.is_empty() {
        vec![cli.context.join(" ")]
    } else {
        Vec::new()
    };

    if !atty::is(Stream::Stdin) {
        let mut piped_input = String::new();
        RealStdin.read_to_string(&mut piped_input)?;
        if !piped_input.trim().is_empty() {
            contents.push(piped_input);
        }
    }

    if cli.context.is_empty() && std::env::var("NO_EDITOR").is_err() {
        let mut real_stdin = RealStdin;
        match input::get_input(true, &mut real_stdin, &RealEditor)? {
            InputResult::Content(content) => contents.push(content),
            InputResult::Cancelled => {
                if cli.debug {
                    println!("Operation cancelled.");
                }
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

        let mut input_contents = contents.to_vec();

        if cli.preprocess {
            let preprocessed_message = api::send_api_request(
                &client,
                &api_key,
                &cli.model,
                &input_contents,
                cli.markdown,
                None,
                true,
            )
            .await?;
            if cli.debug {
                println!("Preprocessed message: {}", preprocessed_message);
            }
            // Replace the last message with the preprocessed message
            if !input_contents.is_empty() {
                *input_contents.last_mut().unwrap() = preprocessed_message;
            } else {
                input_contents.push(preprocessed_message);
            }
        }

        let response = api::send_api_request(
            &client,
            &api_key,
            &cli.model,
            &input_contents,
            cli.markdown,
            None,
            false,
        )
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
