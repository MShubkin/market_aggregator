use std::collections::{HashMap, HashSet};

use crate::common::entities::{EndOfDay, Indices, IndicesReferenceData, Quote, Stock, UsStocksReferenceData};
use gloo_net::http::Request;
use log::debug;
use serde::{Deserialize, Serialize};

use crate::common::env::{MARKET_API_KEY, MARKET_EOD_ROUTE, MARKET_INDICES_ROUTE, MARKET_QUOTE_ROUTE, MARKET_REST_ADDRESS, MARKET_STOCKS_ROUTE};
use crate::common::utils::prepare_symbols_for_url;
use crate::common::MarketResult;

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

    pub async fn get_last_quote(symbols: HashSet<String>) -> MarketResult<HashMap<String, Quote>> {
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

    pub async fn get_indices() -> MarketResult<HashMap<String, Indices>> {
        let url = format!("{}{}", MARKET_REST_ADDRESS, MARKET_INDICES_ROUTE);
        debug!("get_indices. url = {}", url);
        let response = Request::get(url.as_str()).send().await?;
        let response_text = response.text().await?;
        let data: IndicesReferenceData = serde_json::from_str(response_text.as_str())?;
        let data: HashMap<String, Indices> = data
            .data
            .into_iter()
            .map(|value| (value.symbol.clone(), value))
            .collect();
        Ok(data)
    }

    pub async fn get_us_stoks() -> MarketResult<HashMap<String, Stock>> {
        let url = format!("{}{}?country=United%20States", MARKET_REST_ADDRESS, MARKET_STOCKS_ROUTE);
        debug!("get_indices. url = {}", url);
        let response = Request::get(url.as_str()).send().await?;
        let response_text = response.text().await?;
        let data: UsStocksReferenceData = serde_json::from_str(response_text.as_str())?;
        let data: HashMap<String, Stock> = data
            .data
            .into_iter()
            .map(|value| (value.symbol.clone(), value))
            .collect();
        Ok(data)
    }
}
