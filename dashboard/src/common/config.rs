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
    pub fn get_quote_symbols(quote_type: QuoteType) -> LinkedHashSet<String> {
        match quote_type {
            QuoteType::CryptoCurrency => Self::get_crypto_currencies_symbols(),
            QuoteType::Currency => Self::get_currencies_symbols(),
        }
    }
    pub fn get_all_quote_symbols() -> HashSet<String> {
        let all = [
            Self::get_crypto_currencies_symbols(),
            Self::get_currencies_symbols(),
        ];
        let combined = all.into_iter().flatten().collect::<HashSet<_>>();
        combined
    }
}
