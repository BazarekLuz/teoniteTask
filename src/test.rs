#[cfg(test)]
mod tests {
    use rstest::rstest;
    use serde_json::json;
    use crate::api_client::ApiClient;
    use super::*;

    #[rstest]
    #[case("USD", "PLN", 1000f64, 3940.3407761000003)]
    #[case("PLN", "USD", 541.02, 137.30284529742653)]
    #[case("PLN", "EUR", 1000.0, 232.0129716813099)]
    #[case("JPY", "CZK", 1000000.52, 152569.68532942946)]
    #[case("GBP", "GBP", 500.0, 500.0)]
    #[case("CZK", "PLN", 1.50, 0.2569920049732127)]
    #[case("USD", "JPY", 632.54, 95351.03223649484)]
    fn test_parametrized_calculate_exchange(
        #[case] src_curr: &str,
        #[case] trg_curr: &str,
        #[case] amount: f64,
        #[case] expected: f64
    ) {
        let data = json!({
            "data": {
                "PLN": 3.9403407761,
                "CZK": 22.9988133863,
                "EUR": 0.9142101729,
                "GBP": 0.7813501173,
                "JPY": 150.7430869771,
                "USD": 1
            }
        }).to_string();

        let mut api_client = ApiClient::new(
            "test".to_string(),
            "test".to_string(),
            reqwest::Client::new(),
            serde_json::Map::new()
        );

        api_client.currencies_map = serde_json::from_str(&data).unwrap();
        let amount = amount;
        let result = api_client.calculate_exchange(
            src_curr,
            trg_curr,
            &amount
        );
        assert_eq!(result.unwrap().amount, expected)
    }

    #[rstest]
    #[case("USD", "JPY")]
    #[case("PLN", "CZK")]
    #[case("GBP", "EUR")]
    fn test_check_if_curr_code_exists(#[case] src_curr: &str, #[case] trg_curr: &str) {
        let data = json!({
            "data": {
                "PLN": 3.9403407761,
                "CZK": 22.9988133863,
                "EUR": 0.9142101729,
                "GBP": 0.7813501173,
                "JPY": 150.7430869771,
                "USD": 1
            }
        }).to_string();

        let mut api_client = ApiClient::new(
            "test".to_string(),
            "test".to_string(),
            reqwest::Client::new(),
            serde_json::Map::new()
        );

        api_client.currencies_map = serde_json::from_str(&data).unwrap();
        let amount = 1f64;
        let result = api_client.calculate_exchange(
            src_curr,
            trg_curr,
            &amount
        );
        assert!(result.is_ok())
    }

    #[rstest]
    #[case("USP", "USD")]
    #[case("M4A", "MP")]
    #[case("GBP", "EUT")]
    fn test_check_if_curr_code_not_exists(#[case] src_curr: &str, #[case] trg_curr: &str) {
        let data = json!({
            "data": {
                "PLN": 3.9403407761,
                "CZK": 22.9988133863,
                "EUR": 0.9142101729,
                "GBP": 0.7813501173,
                "JPY": 150.7430869771,
                "USD": 1
            }
        }).to_string();

        let mut api_client = ApiClient::new(
            "test".to_string(),
            "test".to_string(),
            reqwest::Client::new(),
            serde_json::Map::new()
        );

        api_client.currencies_map = serde_json::from_str(&data).unwrap();
        let amount = 1f64;
        let result = api_client.calculate_exchange(
            src_curr,
            trg_curr,
            &amount
        );
        assert!(result.is_err())
    }

    #[rstest]
    #[case("usd", "jpy")]
    #[case("Eur", "PlN")]
    #[case("PLn", "CZK")]
    fn test_check_lowercase(#[case] src_curr: &str, #[case] trg_curr: &str) {
        let data = json!({
            "data": {
                "PLN": 3.9403407761,
                "CZK": 22.9988133863,
                "EUR": 0.9142101729,
                "GBP": 0.7813501173,
                "JPY": 150.7430869771,
                "USD": 1
            }
        }).to_string();

        let mut api_client = ApiClient::new(
            "test".to_string(),
            "test".to_string(),
            reqwest::Client::new(),
            serde_json::Map::new()
        );

        api_client.currencies_map = serde_json::from_str(&data).unwrap();
        let amount = 1f64;
        let result = api_client.calculate_exchange(
            src_curr,
            trg_curr,
            &amount
        );
        assert!(result.is_ok())
    }
}