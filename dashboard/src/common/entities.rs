use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// End of day data
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct EndOfDay {
    ///  Symbol ticker of the instrument
    pub symbol: String,
    /// Exchange where instrument is traded
    pub exchange: String,
    /// Date in defined timezone referring to when the bar with specified interval was opened
    pub datetime: String,
    /// Time in defined timezone referring to when the bar with specified interval was opened
    pub timestamp: i64,
    /// The most recent end of day close price
    pub close: String,
}

/// Latest quote
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Quote {
    /// Symbol ticker of the instrument
    pub symbol: String,
    /// Name of the instrument
    pub name: String,
    /// Exchange where instrument is traded
    pub exchange: String,
    /// Market identifier code (MIC) under ISO 10383 standard
    #[serde(default)]
    pub mic_code: String,
    /// market identifier code (MIC) under ISO 10383 standard
    #[serde(default)]
    pub currency: String,
    /// Datetime in defined timezone referring to when the bar with specified interval was opened
    pub datetime: String,
    /// Unix timestamp of the last price
    pub timestamp: i64,
    /// Price at the opening of current bar
    pub open: String,
    /// Highest price which occurred during the current bar
    pub high: String,
    /// Lowest price which occurred during the current bar
    pub low: String,
    /// Close price at the end of the ba
    pub close: String,
    #[serde(default)]
    /// Trading volume during the bar
    pub volume: String,
    /// Close price at the end of the previous bar
    pub previous_close: String,
    /// Close - previous_close
    pub change: String,
    /// (close - previous_close) / previous_close * 100
    pub percent_change: String,
    /// Average volume of the specified period
    #[serde(default)]
    pub average_volume: String,
    /// True if market is open; false if closed
    pub is_market_open: bool,
    /// Collection of 52-week metrics
    #[serde(default)]
    pub fifty_two_week: FiftyTwoWeek,
    /// Percent change in price between the current and the backward one, where period is 1 day, available for crypto
    #[serde(default)]
    pub rolling_1d_change: String,
    /// Percent change in price between the current and the backward one, where period is 7 days, available for crypto
    #[serde(default)]
    pub rolling_7d_change: String,
    /// Percent change in price between the current and the backward one, where period specified in request param rolling_period, available for crypto
    #[serde(default)]
    pub rolling_period_change: String,
    /// Diff between the regular close price and the latest extended price
    #[serde(default)]
    pub extended_change: String,
    /// Percent change in price between the regular close price and the latest extended price
    #[serde(default)]
    pub extended_percent_change: String,
    /// Latest extended price
    #[serde(default)]
    pub extended_price: String,
    /// Unix timestamp of the last extended price
    #[serde(default)]
    pub extended_timestamp: String,
}
/// Collection of 52-week metrics
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

/// Application Reference Data
#[derive(Serialize, Deserialize, Debug, PartialEq, Default, Clone)]
pub struct ReferenceData {
    /// Indices
    pub indices: HashMap<String, Indices>,
    /// US Stocks
    pub us_stocks: HashMap<String, Stock>,
    /// End of day data
    pub end_of_day: HashMap<String, EndOfDay>,
    /// Last quote data
    pub last_quote: HashMap<String, Quote>,
}

/// Indices Reference Data
#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct IndicesReferenceData {
    pub data: Vec<Indices>,
    pub status: String,
    pub count: u64,
}
/// Indices Data
#[derive(Serialize, Deserialize, Debug, PartialEq, Default, Clone)]
pub struct Indices {
    /// Instrument symbol (ticker)
    pub symbol: String,
    /// Full name of instrument
    pub name: String,
    /// Country where the index is located
    pub country: String,
    /// Currency in which instrument is traded by ISO 4217 standard
    pub currency: String,
    /// Exchange where instrument is traded
    pub exchange: String,
    /// Market identifier code (MIC) under ISO 10383 standard
    pub mic_code: String,
}
/// US Stocks Reference Data
#[derive(Serialize, Deserialize, Debug, PartialEq, Default, Clone)]
pub struct UsStocksReferenceData {
    pub data: Vec<Stock>,
    pub status: String,
    pub count: u64,
}
/// Stock data
#[derive(Serialize, Deserialize, Debug, PartialEq, Default, Clone)]
pub struct Stock {
    /// symbol ticker of instrument
    pub symbol: String,
    /// symbol name of instrument
    pub name: String,
    /// Country where the stock is located
    pub country: String,
    /// Currency in which instrument is traded by ISO 4217 standard
    pub currency: String,
    /// Exchange where instrument is traded
    pub exchange: String,
    /// Market identifier code (MIC) under ISO 10383 standard
    pub mic_code: String,
}
/// Real-time price message
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PriceMessage {
    /// Type of event
    pub event: String,
    /// Symbol ticker of instrument
    pub symbol: String,
    /// Currency base
    #[serde(default)]
    pub currency_base: String,
    /// Currency quote
    #[serde(default)]
    pub currency_quote: String,
    #[serde(default)]
    pub exchange: String,
    /// Symbol ticker of instrument
    #[serde(default)]
    pub timestamp: i64,
    /// Real-time price for the underlying instrument
    #[serde(default)]
    pub price: f64,
    /// Bid price
    #[serde(default)]
    pub bid: f64,
    /// Ask price
    #[serde(default)]
    pub ask: f64,
    /// Real-time price for the underlying instrument
    #[serde(default)]
    pub day_volume: f64,
}

/// Web socket response event
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
