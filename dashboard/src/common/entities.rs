use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct EndOfDay {
    pub symbol: String,
    pub exchange: String,
    pub datetime: String,
    pub timestamp: i64,
    pub close: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Quote {
    pub symbol: String,
    pub name: String,
    pub exchange: String,
    #[serde(default)]
    pub mic_code: String,
    #[serde(default)]
    pub currency: String,
    pub datetime: String,
    pub timestamp: i64,
    pub open: String,
    pub high: String,
    pub low: String,
    pub close: String,
    #[serde(default)]
    pub volume: String,
    pub previous_close: String,
    pub change: String,
    pub percent_change: String,
    #[serde(default)]
    pub average_volume: String,
    pub is_market_open: bool,
    #[serde(default)]
    pub fifty_two_week: FiftyTwoWeek,
    #[serde(default)]
    pub rolling_1d_change: String,
    #[serde(default)]
    pub rolling_7d_change: String,
    #[serde(default)]
    pub rolling_period_change: String,
    #[serde(default)]
    pub extended_change: String,
    #[serde(default)]
    pub extended_percent_change: String,
    #[serde(default)]
    pub extended_price: String,
    #[serde(default)]
    pub extended_timestamp: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default, Clone)]
pub struct FiftyTwoWeek {
    pub low: String,
    pub high: String,
    pub low_change: String,
    pub high_change: String,
    pub low_change_percent: String,
    pub high_change_percent: String,
    pub range: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default, Clone)]
pub struct ReferenceData {
    pub indices: HashMap<String, Indices>,
    pub us_stocks: HashMap<String, Stock>,
    pub end_of_day: HashMap<String, EndOfDay>,
    pub last_quote: HashMap<String, Quote>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct IndicesReferenceData {
    pub data: Vec<Indices>,
    pub status: String,
    pub count: u64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default, Clone)]
pub struct Indices {
    pub symbol: String,
    pub name: String,
    pub country: String,
    pub currency: String,
    pub exchange: String,
    pub mic_code: String,
}
#[derive(Serialize, Deserialize, Debug, PartialEq, Default, Clone)]
pub struct UsStocksReferenceData {
    pub data: Vec<Stock>,
    pub status: String,
    pub count: u64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default, Clone)]
pub struct Stock {
    pub symbol: String,
    pub name: String,
    pub country: String,
    pub currency: String,
    pub exchange: String,
    pub mic_code: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PriceMessage {
    pub event: String,
    pub symbol: String,
    #[serde(default)]
    pub currency_base: String,
    #[serde(default)]
    pub currency_quote: String,
    #[serde(default)]
    pub exchange: String,
    #[serde(default)]
    pub timestamp: i64,
    #[serde(default)]
    pub price: f64,
    #[serde(default)]
    pub bid: f64,
    #[serde(default)]
    pub ask: f64,
    #[serde(default)]
    pub day_volume: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WSResponseEvent {
    pub event: String,
}

impl Default for WSResponseEvent {
    fn default() -> Self {
        Self {
            event: "unknown".to_owned(),
        }
    }
}
