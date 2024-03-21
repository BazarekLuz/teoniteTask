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
            // reqwest::StatusCode::TOO_MANY_REQUESTS => {
            //
            // }
            _ => {
                panic!("Blad");
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
                    let exchange_rate = val1.as_f64().unwrap() * val2.as_f64().unwrap();
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

// _ => Err("Something went wrong".to_string())

// fn find_strings_in_json(json: &Value, string1: &str, string2: &str) -> Result<(), String> {
//     match json {
//         Value::Object(obj) => {
//             if let (Some(_), Some(_)) = (obj.get(string1), obj.get(string2)) {
//                 Ok(())
//             } else {
//                 Err(format!("Strings '{}' and/or '{}' not found in JSON", string1, string2))
//             }
//         }
//         _ => Err("JSON must be an object".to_string()),
//     }
// }

// 429 - too many requests

// let xd = match response.status() {
//     reqwest::StatusCode::OK => {
//         println!("Success");
//         let body = response.text().await.unwrap();
//         serde_json::from_str::<&str>(body.as_str()).unwrap();
//     },
//     reqwest::StatusCode::UNAUTHORIZED => {
//         println!("XD");
//         let body = response.text().await.unwrap();
//         serde_json::from_str::<String>(&body).unwrap();
//     },
//     _ => {
//         panic!("Coś kurwa poszlo nie tak");
//     }
// };

// println!("{:#?}", &response);

// najpierw dzielenie do dolarów, potem mnożenie do waluty