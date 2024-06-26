use std::env;

    pub async fn setup_api_key() -> Option<String> {
        match env::var("OPENROUTER_API_KEY") {
            Ok(key) => Some(key),
            Err(_) => {
                println!("OPENROUTER_API_KEY not set. Skipping API tests.");
                None
            }
        }
}
