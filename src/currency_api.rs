use serde_json::{Map, Value};
use crate::custom_error::CustomError;

pub struct CurrencyApi {
    api_url: String,
    api_key: String,
    http_client: reqwest::Client,
    currencies_map: Map<String, Value>
}

impl CurrencyApi {
    pub fn new(
        api_url: String,
        api_key: String,
        http_client: reqwest::Client,
        currencies_map: Map<String, Value>
    ) -> Self {
        Self {
            api_url,
            api_key,
            http_client,
            currencies_map
        }
    }

    pub async fn get_all_rates(&self) -> Value {
        let response = self.http_client
            .get(&self.api_url)
            .header("apikey", &self.api_key)
            .send()
            .await
            .unwrap();

        let body = match response.status(){
            reqwest::StatusCode::OK => {
                response.text().await.unwrap()
            },
            reqwest::StatusCode::TOO_MANY_REQUESTS => {
                String::from("Too many requests, limit set at 10 per minute / 5000 per month")
            }
            _ => {
                panic!("Error occurred while unwrapping response body");
            }
        };

        serde_json::from_str(&body).unwrap()
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