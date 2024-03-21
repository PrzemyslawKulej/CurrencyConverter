use crate::cache::{CacheEntry, RatesResponse, CURRENCY_CACHE};
use reqwest::blocking;
use std::collections::HashMap;
use std::time::SystemTime;


// Retrieves exchange rates either from cache or API based on the provided base currency.
pub fn get_rates_from_cache_or_api(
    api_key: &str,
    base_currency: &str,
) -> Result<HashMap<String, f64>, Box<dyn std::error::Error>> {
    let mut cache = CURRENCY_CACHE.lock().unwrap();
    if let Some(entry) = cache.get(base_currency) {
        if entry.last_updated.elapsed()?.as_secs() < 3600 {
            println!(
                "Fetching data from cache for base currency: {}",
                base_currency
            );
            return Ok(entry.rates.clone());
        }
    }

    println!(
        "Fetching data from API for base currency: {}",
        base_currency
    );
    let rates_url = format!(
        "http://api.exchangeratesapi.io/v1/latest?access_key={}&base={}",
        api_key, base_currency
    );
    let response = blocking::get(&rates_url)?.text()?;
    let rates_response: RatesResponse = serde_json::from_str(&response)?;

    cache.insert(
        base_currency.to_string(),
        CacheEntry {
            rates: rates_response.rates.clone(),
            last_updated: SystemTime::now(),
        },
    );

    Ok(rates_response.rates)
}

// Performs currency conversion based on the provided parameters and prints the result.
pub fn perform_conversion(
    api_key: &str,
    base_currency: &str,
    target_currency: &str,
    amount: f64,
) -> Result<(), Box<dyn std::error::Error>> {
    // Retrieving rates for the base currency
    let rates = get_rates_from_cache_or_api(api_key, base_currency)?;

    // Finding the rate for the target currency
    if let Some(rate) = rates.get(target_currency) {
        let converted_amount = amount * rate;
        println!(
            "\n{} {} is {:.2} {} after conversion.",
            amount, base_currency, converted_amount, target_currency
        );
    } else {
        println!("Currency not found.");
    }

    Ok(())
}
