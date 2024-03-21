## Project name

Currency Converter

## Description

Currency Converter is a command-line tool written in Rust, designed to enable users to convert amounts between various currencies based on real-time exchange rates fetched from an API.

## Technical Specifications

- Programming Language: Rust
- External Libraries: reqwest for HTTP requests, serde for JSON parsing.

## Features

- Fetch real-time exchange rates from a free currency conversion API.
- Accept three command-line arguments: source currency code, target currency code, and the amount to be converted.
- Calculate and display the converted amount in the target currency, including the exchange rate used for the conversion.
- Handle common errors, such as network issues, invalid currency codes, or API request limits, gracefully.
- Include a Dockerfile for easy testing and deployment within Docker environments.

## How to Build and Run

1. Prerequisites:

- Rust programming environment (Cargo and Rust compiler).
- An API key for the currency conversion API (obtain one from the API provider's website(https://exchangeratesapi.io/)).

2. Building the Application:

- Clone the repository to your local machine.
- Navigate to the project directory and run cargo build --release to compile the project.

3. Running the Application:

- Run the application using cargo run

## Authors

- [@PrzemyslawKulej](https://www.github.com/PrzemyslawKulej)

## Obtaining an API Key

- Visit the currency conversion API's website and sign up for an API key(https://exchangeratesapi.io/).
- Set the API key in your environment variables or directly in the application as instructed by the API provider.

## CI/CD Configuration

This project includes a .gitlab-ci.yml file for Continuous Integration and Continuous Deployment using GitLab CI/CD. It covers Docker image building.
