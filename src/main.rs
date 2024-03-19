use anyhow::{Context, Result};
use clap::{Parser};
use reqwest::{Client};
use dotenv::dotenv;
use serde_json::{json, Value};

mod currency_api;
mod cli;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let api_key = std::env::var("API_KEY").expect("API_KEY must be provided");

    let args = cli::Cli::parse();
    let currency_api = currency_api::CurrencyApi::new(String::from("http://api.freecurrencyapi.com/v1/latest"));

    let response = Client::new()
        .get(currency_api.all_rates())
        .header("apikey", api_key)
        .send()
        .await?;

    let body = response.text().await?;

    let data: Value = serde_json::from_str(&body)?;
    let text = json!(data["data"]);

    for i in text.as_object().unwrap() {
        println!("{:?} {:}\n", i.0, i.1);
    }
    // println!("{:?}", text.as_object());

    // let client = Client::new();
    // let request = RequestBuilder::build();

    Ok(())
}

// http://api.nbp.pl/api/exchangerates/tables/A/

// let pln = Currency::PLN;
// println!("{:?}", pln);

// println!("{}", &args.source_currency_code);
// println!("{}", &args.target_currency_code);
// println!("{}", &args.amount);