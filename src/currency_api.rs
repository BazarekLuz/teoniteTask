use clap::error::ContextKind::Custom;
use reqwest::Response;
use serde_json::Value;
use crate::custom_error::CustomError;

pub struct CurrencyApi {
    api_url: String,
    api_key: String,
    http_client: reqwest::Client
}

impl CurrencyApi {
    pub fn new(
        api_url: String,
        api_key: String,
        http_client: reqwest::Client
    ) -> Self {
        Self {
            api_url,
            api_key,
            http_client
        }
    }

    pub async fn get_all_rates(&self) -> Result<&Value, CustomError> {
        let response = self.http_client
            .get(&self.api_url)
            .header("apikey", &self.api_key)
            .send()
            .await
            .unwrap();

        // match response.status() {
        //     reqwest::StatusCode::OK => {
        //         match response {
        //             Value::Object(obj) => {
        //
        //             }
        //         }
        //     }
        // }

        // let body = match response.status(){
        //     reqwest::StatusCode::OK => {
        //         response.text().await.unwrap()
        //     },
        //     // reqwest::StatusCode::TOO_MANY_REQUESTS => {
        //     //
        //     // }
        //     _ => {
        //         panic!("Blad");
        //     }
        // };


        // serde_json::from_str(&body).unwrap()

        // match response.status() {
        //     reqwest::StatusCode::OK => {
        //         let body = response.text().await.unwrap();
        //         Ok(serde_json::from_str(&body).unwrap())
        //     },
        //     reqwest::StatusCode::TOO_MANY_REQUESTS => {
        //         Err(CustomError::new("Too many requests! Limit is 10 requests per minute/ 5k per month."))
        //     },
        //     _ => {
        //         Err(CustomError::new("Different error"))
        //     }
        // }
    }

    pub fn calculate_exchange<'a>(
        &'a self,
        source_currency_code: &'a String,
        target_currency_code: &'a String,
        amount: &f64,
        currencies: &'a Value
    ) -> Result<(f64, &str, f64), CustomError>{
        match currencies {
            Value::Object(obj) => {
                if let (Some(val1), Some(val2)) = (obj.get(source_currency_code), obj.get(target_currency_code)) {
                    let exchange_rate = val2.as_f64().unwrap() / val1.as_f64().unwrap();
                    let ret_amount = amount * exchange_rate ;
                    Ok((ret_amount, target_currency_code, exchange_rate))
                } else {
                    Err(CustomError::new(format!("Currency '{}' or '{}' not found", source_currency_code, target_currency_code).as_str()))
                }
            },
            _ => Err(CustomError::new("Something went wrong"))
        }
    }
}