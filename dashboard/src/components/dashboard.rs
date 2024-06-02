use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use chrono::{DateTime, FixedOffset, NaiveDateTime, Utc};
use futures::{SinkExt, StreamExt};
use gloo::console;
use gloo_net::websocket::futures::WebSocket;
use gloo_net::websocket::Message;
use linked_hash_set::LinkedHashSet;
use log::{error, info};
use wasm_bindgen_futures::spawn_local;
use yew::{AttrValue, Component, Context, html, Html, Properties, use_state};

use crate::common::config::DashboardConfiguration;
use crate::common::enums::QuoteType;
use crate::common::MarketResult;
use crate::common::utils::format_time;
use crate::components::quotes::{PriceData, QuotesComponent, QuotesProps};
use crate::services::websocket::{
    PriceMessage, WebSocketService, WSResponseEvent, WSResponseEventType,
};

pub struct DashboardComponent {
    crypto_currencies_symbols: Arc<LinkedHashSet<String>>,
    currencies_symbols: Arc<LinkedHashSet<String>>,
    prices: Arc<RwLock<HashMap<String, PriceData>>>,
}

pub enum DashboardMessage {
    WebSocketResponse(String),
}

impl Component for DashboardComponent {
    type Message = DashboardMessage;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let ws_soket_result = WebSocketService::open_ws_connection();
        match ws_soket_result {
            Ok(mut web_socket) => {
                let response_callback = ctx.link().callback(DashboardMessage::WebSocketResponse);
                web_socket
                    .subscribe_real_time_rates(
                        DashboardConfiguration::get_all_quote_symbols(),
                        response_callback,
                    )
                    .expect("cannot subscribe to real time rates");
                web_socket.heartbeat().expect("heartbeat error");
            }
            Err(error) => {
                error!("Web socket connection Error: {:?}", error);
            }
        }
        Self {
            crypto_currencies_symbols: Arc::new(
                DashboardConfiguration::get_crypto_currencies_symbols(),
            ),
            currencies_symbols: Arc::new(DashboardConfiguration::get_currencies_symbols()),
            prices: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            DashboardMessage::WebSocketResponse(data) => {
                let response_event: WSResponseEvent =
                    serde_json::from_str(data.as_str()).unwrap_or_default();

                match WSResponseEventType::from(response_event.event) {
                    WSResponseEventType::SubscribeStatus => {
                        info!("subscribe status {:?}", data);
                    }
                    WSResponseEventType::Price => {
                        let price_message: PriceMessage =
                            serde_json::from_str(data.as_str()).unwrap_or_default();
                        info!("price_message {:?}", price_message);
                        let mut lock = self.prices.write().unwrap();
                        lock.insert(
                            price_message.symbol.clone(),
                            PriceData {
                                symbol: price_message.symbol,
                                price: price_message.price,
                                bid: price_message.bid,
                                ask: price_message.ask,
                                time: format_time(price_message.timestamp),
                                ..Default::default()
                            },
                        );
                    }
                    WSResponseEventType::Heartbeat => {
                        info!("heart beat status {:?}", data);
                    }
                    WSResponseEventType::Unknown => {
                        info!("unknown message {:?}", data);
                    }
                }
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let crypto_currencies_props = QuotesProps {
            title: "Крипто-валюты".to_owned(),
            symbols: self.crypto_currencies_symbols.clone(),
            prices: self.get_quote_data(QuoteType::CryptoCurrency),
        };
        let currencies_props = QuotesProps {
            title: "Мировые валюты".to_owned(),
            symbols: self.currencies_symbols.clone(),
            prices: self.get_quote_data(QuoteType::Currency),
        };

        html! {
            <div>
                  <div><QuotesComponent ..crypto_currencies_props.clone() /></div>
                  <div><QuotesComponent ..currencies_props.clone() /></div>
            </div>
        }
    }
}

impl DashboardComponent {
    fn get_quote_data(&self, quote_type: QuoteType) -> HashMap<String, PriceData> {
        let symbols = DashboardConfiguration::get_quote_symbols(quote_type);
        let data = self.prices.read().unwrap();
        let filter_data = data
            .iter()
            .filter(|value| symbols.contains(value.0))
            .map(|value| (value.0.clone(), value.1.clone()))
            .collect();
        filter_data
    }
}
