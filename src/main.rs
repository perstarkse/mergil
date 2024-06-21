mod api;
mod editor;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (contents, _temp_file) = editor::open_editor()?;

    println!("You entered:\n{}", contents);

    let api_key = api::get_api_key();
    let client = reqwest::Client::new();
    let api_response = api::send_api_request(
        &client,
        &api_key,
        "deepseek/deepseek-coder",
        &contents,
    )
    .await?;

    if let Some(choice) = api_response.choices.get(0) {
        let trimmed_content = choice.message.content.trim_start();
        println!("API Response:\n{}", trimmed_content);
    } else {
        println!("Failed to extract content from the API response");
    }

    Ok(())
}
