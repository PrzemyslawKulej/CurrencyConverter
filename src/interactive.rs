use crate::currency::{get_rates_from_cache_or_api, perform_conversion};
use std::io::{self, Write};

pub fn interactive_mode(api_key: &str) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        println!("Welcome to the Currency Converter!");

        let base_currency = ask_for_currency("Enter the base currency code (e.g., EUR): ")?;
        display_available_currencies(&api_key, &base_currency)?;

        let target_currency = ask_for_currency("Enter the target currency code (e.g., USD): ")?;
        let amount = ask_for_amount("Enter the amount to convert: ")?;

        perform_conversion(&api_key, &base_currency, &target_currency, amount)?;

        println!("Do you want to perform another conversion? (y/n): ");
        let mut decision = String::new();
        io::stdin().read_line(&mut decision)?;
        if decision.trim().eq_ignore_ascii_case("n") {
            break;
        }
    }

    Ok(())
}

pub fn ask_for_currency(prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
    print!("{}", prompt);
    io::stdout().flush()?;
    let mut currency = String::new();
    io::stdin().read_line(&mut currency)?;
    Ok(currency.trim().to_uppercase())
}

pub fn ask_for_amount(prompt: &str) -> Result<f64, Box<dyn std::error::Error>> {
    print!("{}", prompt);
    io::stdout().flush()?;
    let mut amount_str = String::new();
    io::stdin().read_line(&mut amount_str)?;
    let amount = amount_str.trim().parse()?;
    Ok(amount)
}

pub fn display_available_currencies(
    api_key: &str,
    base_currency: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let rates = get_rates_from_cache_or_api(api_key, base_currency)?;

    println!(
        "\nAvailable currencies with rates relative to {}:",
        base_currency
    );

    let mut rates_vec: Vec<(&String, &f64)> = rates.iter().collect();
    rates_vec.sort_by_key(|k| k.0);

    let mut count = 0;
    for (currency, rate) in rates_vec.iter() {
        print!("{:<5}: {:<10.2} ", currency, rate);
        count += 1;
        if count % 9 == 0 {
            println!();
        }
    }
    if count % 9 != 0 {
        println!();
    }

    Ok(())
}
