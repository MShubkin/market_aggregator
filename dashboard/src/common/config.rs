use std::collections::HashSet;

use linked_hash_set::LinkedHashSet;

use crate::common::enums::QuoteType;

pub struct DashboardConfiguration;

impl DashboardConfiguration {
    pub fn get_crypto_currencies_symbols() -> LinkedHashSet<String> {
        let mut s = LinkedHashSet::new();
        s.insert("EOS/USD".to_owned());
        s.insert("ETH/USD".to_owned());
        s.insert("LTC/USD".to_owned());
        s.insert("BTC/USD".to_owned());
        s.insert("ETH/BTC".to_owned());
        s
    }

    pub fn get_currencies_symbols() -> LinkedHashSet<String> {
        let mut s = LinkedHashSet::new();
        s.insert("EUR/USD".to_owned());
        s.insert("EUR/GBP".to_owned());
        s.insert("USD/CNY".to_owned());
        s.insert("USD/JPY".to_owned());
        s.insert("GBP/CHF".to_owned());
        s
    }

    pub fn get_indices_symbols() -> LinkedHashSet<String> {
        let mut s = LinkedHashSet::new();
        s.insert("DJIA".to_owned());
        s.insert("DJT".to_owned()); //Dow Jones Transportation Average
        s.insert("COMP".to_owned()); //NASDAQ Composite Index
        s.insert("NYA".to_owned()); //NYSE Composite Index
        s.insert("SPX".to_owned()); //S&P 500 Index
        s.insert("MID".to_owned()); //S&P MidCap 400
        s.insert("OEX".to_owned()); //S&P 100 Index
        s.insert("NDX".to_owned()); //NASDAQ 100
        s.insert("UKX".to_owned()); //FTSE100
        s.insert("FCHI".to_owned()); //CAC40
        s.insert("DAX".to_owned());
        s.insert("N225".to_owned()); //Nikkei 225
        s.insert("AXJO".to_owned()); //ASX 200
        s.insert("RUT".to_owned()); //Russell 2000 Index
        s
    }

    pub fn get_us_stocks() -> LinkedHashSet<String> {
        let mut s = LinkedHashSet::new();
        s.insert("MSFT".to_owned());
        s.insert("AAPL".to_owned());
        s.insert("NVDA".to_owned());
        s.insert("GOOGL".to_owned());
        s.insert("AMZN".to_owned());
        s.insert("META".to_owned());
        s.insert("BRK.B".to_owned());
        s.insert("BRK.A".to_owned());
        s.insert("LLY".to_owned());
        s.insert("TSM".to_owned());
        s.insert("AVGO".to_owned());
        s.insert("NVO".to_owned());
        s.insert("JPM".to_owned());
        s.insert("TSLA".to_owned());
        s.insert("WMT".to_owned());
        s.insert("XOM".to_owned());
        s.insert("UNH".to_owned());
        s.insert("MA".to_owned());
        s.insert("ASML".to_owned());
        s.insert("PG".to_owned());
        s
    }
    pub fn get_quote_symbols(quote_type: QuoteType) -> LinkedHashSet<String> {
        match quote_type {
            QuoteType::CryptoCurrency => Self::get_crypto_currencies_symbols(),
            QuoteType::Currency => Self::get_currencies_symbols(),
            QuoteType::Indices => Self::get_indices_symbols(),
            QuoteType::USStocks => Self::get_us_stocks(),
        }
    }
    pub fn get_all_quote_symbols() -> HashSet<String> {
        let all = [
            Self::get_crypto_currencies_symbols(),
            Self::get_currencies_symbols(),
            Self::get_indices_symbols(),
            Self::get_us_stocks(),
        ];
        all.into_iter().flatten().collect::<HashSet<_>>()
    }
}
