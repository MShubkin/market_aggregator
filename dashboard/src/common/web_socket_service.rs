use crate::common::env::{MARKET_API_KEY, MARKET_REAL_TIME_PRICE_ROUTE, MARKET_WS_ADDRESS};
use crate::common::MarketResult;
use futures::lock::Mutex;
use futures::stream::{SplitSink, SplitStream};
use futures::{SinkExt, StreamExt};
use gloo_net::websocket::futures::WebSocket;
use gloo_net::websocket::{Message, WebSocketError};
use log::{debug, error, info};
use serde::{Deserialize, Deserializer, Serialize};
use std::borrow::BorrowMut;
use std::collections::HashSet;
use std::fmt::{format, Debug};
use std::sync::Arc;
use std::time::Duration;
use gloo::net::http::Request;
use wasm_bindgen_futures::spawn_local;
use yew::{AttrValue, Callback};
use yew::platform::pinned::RwLock;
use yew::platform::time::sleep;

const NINE_SEC: Duration = Duration::from_secs(9);


pub struct WebSocketService {
    web_socket_writer: Arc<Mutex<SplitSink<WebSocket, Message>>>,
    web_socket_reader: Arc<Mutex<SplitStream<WebSocket>>>,
}

impl WebSocketService {
    pub fn open_ws_connection() -> MarketResult<Self> {
        let web_socket = WebSocket::open(
            format!(
                "{}{}?apikey={}",
                MARKET_WS_ADDRESS, MARKET_REAL_TIME_PRICE_ROUTE, MARKET_API_KEY
            )
            .as_str(),
        )?;
        let split = web_socket.split();
        let web_socket_writer = split.0;
        let web_socket_reader = split.1;
        Ok(WebSocketService {
            web_socket_writer: Arc::new(Mutex::new(web_socket_writer)),
            web_socket_reader: Arc::new(Mutex::new(web_socket_reader)),
        })
    }

    pub fn subscribe_real_time_rates(
        &mut self,
        symbols: HashSet<String>,
        response_callback: Callback<String>,
    ) -> MarketResult<()> {
        let symbols_str = symbols.iter().fold("".to_owned(), |mut acc, x| {
            if !acc.is_empty() {
                acc.push_str(",")
            };
            acc.push_str(x);
            acc
        });
        let msg = format!(
            "{{
                \"action\": \"subscribe\",
                \"params\": {{
                \"symbols\": \"{}\"
                            }}
        }}",
            symbols_str
        );

        let writer = self.web_socket_writer.clone();
        let reader = self.web_socket_reader.clone();
        spawn_local(async move {
            let mut writer = writer.lock().await;
            let result = writer.send(Message::Text(msg.to_string())).await;
            match result {
                Ok(_) => {}
                Err(_) => {}
            }
        });
        spawn_local(async move {
            let mut reader = reader.lock().await;
            while let Some(msg) = reader.next().await {
                match msg {
                    Ok(message) => match message {
                        Message::Text(text) => {
                            response_callback.emit(text);
                        }
                        Message::Bytes(bytes) => {
                            error!("{}", format!("bytes message is not supported"));
                        }
                    },
                    Err(error) => {
                        error!("{}", format!("web socket error {:?}", error));
                    }
                }
            }
        });
        Ok(())
    }

    pub fn heartbeat(
        &mut self
    ) -> MarketResult<()> {
        let msg = format!(
            "{{
                \"action\": \"heartbeat\"
        }}"
        );
        let writer = self.web_socket_writer.clone();
        spawn_local(async move {
            loop {
                let mut writer = writer.lock().await;
                let result = writer.send(Message::Text(msg.to_string())).await;
                match result {
                    Ok(_) => {}
                    Err(error) => { error!("{}", format!("web socket error {:?}", error)); }
                }
                sleep(NINE_SEC).await;
            }
        });
        Ok(())
    }


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

pub enum WSResponseEventType {
    SubscribeStatus,
    Price,
    Heartbeat,
    Unknown,
}

impl From<String> for WSResponseEventType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "price" => WSResponseEventType::Price,
            "heartbeat" => WSResponseEventType::Heartbeat,
            "subscribe-status" => WSResponseEventType::SubscribeStatus,
            _ => WSResponseEventType::Unknown,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PriceMessage {
    pub event: String,
    pub symbol: String,
    pub currency_base: String,
    pub currency_quote: String,
    pub exchange: String,
    pub timestamp: i64,
    pub price: f64,
    pub bid: f64,
    pub ask: f64,
    pub day_volume: f64,
}
