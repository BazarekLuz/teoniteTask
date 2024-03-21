use serde_json::{Map, Value};
use crate::custom_error::CustomError;
use crate::exchange_tuple::ExchangeTuple;

pub struct ApiClient {
    api_url: String,
    api_key: String,
    http_client: reqwest::Client,
    pub currencies_map: Map<String, Value>
}

impl ApiClient {
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

    pub async fn get_exchange_rates(&mut self) -> Result<(), CustomError> {
        let response = self.http_client
            .get(&self.api_url)
            .header("apikey", &self.api_key)
            .send()
            .await;

        match response {
            Ok(res) => {
                let text = res.text().await.unwrap();
                let json = serde_json::from_str(&text).unwrap();
                match json {
                    Value::Object(obj) => {
                        Ok(self.currencies_map = obj)
                    },
                    _ => Err(CustomError::new("Something went wrong"))
                }
            },
            Err(err) => {
                Err(CustomError::new(err.to_string().as_str()))
            }
        }
    }

    pub fn calculate_exchange(
        self,
        src_curr_code: &str,
        trg_curr_code: &str,
        amount: &f64
    ) -> Result<ExchangeTuple, CustomError> {
        match &self.currencies_map["data"] {
            Value::Object(obj) => {
                if let (Some(val1), Some(val2)) =
                    (obj.get(src_curr_code.to_uppercase().as_str()),
                     obj.get(trg_curr_code.to_uppercase().as_str())) {
                    let exchange_rate = val2.as_f64().unwrap() / val1.as_f64().unwrap();
                    let ret_amount = amount * &exchange_rate;
                    Ok(ExchangeTuple::new(ret_amount, trg_curr_code.to_string(), exchange_rate))
                } else {
                    Err(CustomError::new(format!("Currency '{}' or '{}' not found", src_curr_code, trg_curr_code).as_str()))
                }
            },
            _ => Err(CustomError::new("Something went wrong"))
        }
    }
}