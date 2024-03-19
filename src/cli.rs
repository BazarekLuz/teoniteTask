use clap::Parser;

#[derive(Parser)]
pub(crate) struct Cli {
    source_currency_code: String,
    target_currency_code: String,
    amount: i32
}