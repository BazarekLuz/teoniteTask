pub struct ExchangeTuple {
    amount: f64,
    currency_code: String,
    exchange_rate: f64
}

impl ExchangeTuple {
    pub fn new(
        amount: f64,
        currency_code: String,
        exchange_rate: f64
    ) -> Self {
        Self {
            amount,
            currency_code,
            exchange_rate
        }
    }

}