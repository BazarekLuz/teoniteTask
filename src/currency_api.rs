pub(crate) struct CurrencyApi {
    all_rates: String
}

impl CurrencyApi {
    pub fn new(all_rates: String) -> Self {
        Self { all_rates }
    }

    pub fn all_rates(self) -> String {
        self.all_rates
    }
}