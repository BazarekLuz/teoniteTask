use std::error::Error;

use anyhow::Result;
use clap::Parser;
use dotenv::dotenv;
use serde_json::Map;
use crate::custom_error::CustomError;

mod currency_api;
mod cli;
mod custom_error;
mod client;
mod exchange_tuple;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let api_key = std::env::var("API_KEY").expect("API_KEY must be provided");
    let args = cli::Cli::parse();

    // let currency_api = currency_api::CurrencyApi::new(
    //     String::from("https://api.freecurrencyapi.com/v1/latest"),
    //     api_key,
    //     reqwest::Client::new()
    // );

    // let available_currencies = currency_api.get_all_rates().await;
    // let src_curr_code = &args.source_currency_code.to_uppercase();
    // let tar_curr_code = &args.target_currency_code.to_uppercase();
    // let exchange_tuple = currency_api.calculate_exchange(
    //     &src_curr_code,
    //     &tar_curr_code,
    //     &args.amount,
    //     &available_currencies["data"]
    // );

    // if exchange_tuple.is_err() {
    //     println!("{}", &exchange_tuple.unwrap_err())
    // } else {
    //     let tuple = exchange_tuple.unwrap();
    //     println!("Total amount: {:.2} {},\nExchange rate: {}", tuple.0, tuple.1, tuple.2);
    // }

    let mut client = client::ApiClient::new(
        String::from("https://api.freecurrencyapi.com/v1/latest"),
        api_key,
        reqwest::Client::new(),
        serde_json::Map::new()
    );
    match client.get_exchange_rates().await {
        Ok(result) => {},
        Err(err) => println!("{}", err)
    }

    let src_curr_code = &args.source_currency_code.to_uppercase();
    let tar_curr_code = &args.target_currency_code.to_uppercase();

    let exchange_result =  client.calculate_exchange(
        &src_curr_code,
        &tar_curr_code,
        &args.amount);

    // println!("{:#?}", client.currencies_map);
    Ok(())
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}