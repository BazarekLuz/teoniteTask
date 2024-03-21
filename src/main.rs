use anyhow::Result;
use clap::Parser;
use dotenv::dotenv;

mod currency_api;
mod cli;
mod custom_error;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let api_key = std::env::var("API_KEY").expect("API_KEY must be provided");
    let args = cli::Cli::parse();

    let currency_api = currency_api::CurrencyApi::new(
        String::from("http://api.freecurrencyapi.com/v1/latest"),
        api_key,
        reqwest::Client::new()
    );

    let available_currencies = currency_api.get_all_rates().await;
    let src_curr_code = &args.source_currency_code.to_uppercase();
    let tar_curr_code = &args.target_currency_code.to_uppercase();
    let x = currency_api.calculate_exchange(
        &src_curr_code,
        &tar_curr_code,
        &args.amount,
        &available_currencies["data"]
    );

    if x.is_err() {
        println!("{}", &x.unwrap_err())
    } else {
        let tuple = x.unwrap();
        println!("Total amount: {:.2} {},\nExchange rate: {}", tuple.0, tuple.1, tuple.2);
    }
    Ok(())
}