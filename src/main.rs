use dotenv::dotenv;
use std::env;

mod cache;
mod currency;
mod interactive;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let api_key = env::var("CURRENCY_CONVERTER_API_KEY").expect("API key not set");
    interactive::interactive_mode(&api_key)?;
    Ok(())
}
