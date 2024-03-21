use std::error::Error;

use anyhow::Result;
use clap::Parser;
use dotenv::dotenv;

mod cli;
mod custom_error;
mod api_client;
mod exchange_tuple;
mod test;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let api_key = std::env::var("API_KEY").expect("API_KEY must be provided");
    let args = cli::Cli::parse();

    let mut client = api_client::ApiClient::new(
        String::from("https://api.freecurrencyapi.com/v1/latest"),
        api_key,
        reqwest::Client::new(),
        serde_json::Map::new()
    );
    match client.get_exchange_rates().await {
        Ok(result) => {},
        Err(err) => println!("{:?}", err)
    }

    let src_curr_code = &args.source_currency_code.to_uppercase();
    let tar_curr_code = &args.target_currency_code.to_uppercase();

    let exchange_result =  client.calculate_exchange(
        &src_curr_code,
        &tar_curr_code,
        &args.amount
    );

    match exchange_result {
        Ok(tup) => println!("Total amount: {:.2} {}\nExchange rate: {}",
                            &tup.amount,
                            &tup.currency_code,
                            &tup.exchange_rate
        ),
        Err(err) => println!("{:?}", err)
    }
    Ok(())
}