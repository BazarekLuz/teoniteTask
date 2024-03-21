use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    pub source_currency_code: String,
    pub target_currency_code: String,
    pub amount: f64
}