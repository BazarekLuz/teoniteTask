#[derive(Debug)]
pub struct ExchangeTuple {
    pub amount: f64,
    pub currency_code: String,
    pub exchange_rate: f64
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