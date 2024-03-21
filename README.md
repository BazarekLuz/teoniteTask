**HOW TO USE**
1. to use the application - clone repo to your desired directory. You will also need cargo - Rust package and build manager
2. application uses free API, if you intend to use the app, You need to get your API key from https://app.freecurrencyapi.com/. To get the key, You need to sign into an account, then please paste the key into .env file into API_KEY=_[your key]_
3. open app directory in command line
4. cargo build
5. cargo run -- _<currency_code1>_ _<currency_code2>_ _<amount>_; example usage: 'cargo run -- usd eur 100.50'
6. use 'cargo test' to run unit tests

**ABOUT APPLICATION:**

App uses USD as base currency and main function returns exchange rate alongside calculated new amount based on input currencies.
