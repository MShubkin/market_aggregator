use crate::common::env::{MARKET_API_KEY, MARKET_EOD_ROUTE, MARKET_QUOTE_ROUTE, MARKET_REST_ADDRESS};
use crate::common::utils::prepare_symbols_for_url;
use crate::common::MarketResult;
use gloo_net::http::Request;
use log::debug;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

pub struct RestApiService;

impl RestApiService {
    pub async fn get_end_of_day_data(
        symbols: HashSet<String>,
    ) -> MarketResult<HashMap<String, EndOfDay>> {
        let url = format!(
            "{}{}?symbol={}&apikey={}",
            MARKET_REST_ADDRESS,
            MARKET_EOD_ROUTE,
            prepare_symbols_for_url(symbols),
            MARKET_API_KEY
        );
        debug!("get_end_of_day_data. url = {}", url);
        let response = Request::get(url.as_str()).send().await?;
        let response_text = response.text().await?;
        let data: HashMap<String, EndOfDay> = serde_json::from_str(response_text.as_str())?;
        Ok(data)
    }

    pub async fn get_last_quote(
        symbols: HashSet<String>,
    ) -> MarketResult<HashMap<String, Quote>> {
        let url = format!(
            "{}{}?symbol={}&interval=1day&apikey={}",
            MARKET_REST_ADDRESS,
            MARKET_QUOTE_ROUTE,
            prepare_symbols_for_url(symbols),
            MARKET_API_KEY
        );
        debug!("get_last_quote. url = {}", url);
        let response = Request::get(url.as_str()).send().await?;
        let response_text = response.text().await?;
        let data: HashMap<String, Quote> = serde_json::from_str(response_text.as_str())?;
        Ok(data)
    }

}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct EndOfDay {
    pub symbol: String,
    pub exchange: String,
    pub datetime: String,
    pub timestamp: i64,
    pub close: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct FiftyTwoWeek {
    pub low: String,
    pub high: String,
    pub low_change: String,
    pub high_change: String,
    pub low_change_percent: String,
    pub high_change_percent: String,
    pub range: String,
}
