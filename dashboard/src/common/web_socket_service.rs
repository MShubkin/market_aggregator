use crate::common::env::{MARKET_API_KEY, MARKET_REAL_TIME_PRICE_ROUTE, MARKET_WS_ADDRESS};
use crate::common::MarketResult;
use futures::stream::{SplitSink, SplitStream};
use futures::{SinkExt, StreamExt};
use gloo_net::websocket::futures::WebSocket;
use gloo_net::websocket::Message;
use std::fmt::format;
use std::sync::Arc;
use log::info;
use wasm_bindgen_futures::spawn_local;
use yew::{AttrValue, Callback};
use std::borrow::BorrowMut;
use futures::lock::Mutex;
use gloo::console;
use yew::platform::pinned::RwLock;

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

    pub fn subscribe_real_time_rates(&mut self, response_callback: Callback<AttrValue>) -> MarketResult<()> {
        let msg = "{
            \"action\": \"subscribe\",
            \"params\": {
            \"symbols\": \"AAPL,INFY,TRP,QQQ,IXIC,EUR/USD,USD/JPY,BTC/USD,ETH/BTC\"
                        }
        }";

        let writer = self.web_socket_writer.clone();
        let reader = self.web_socket_reader.clone();

        spawn_local(async move {
            let mut writer = writer.lock().await;
            let result = writer.send(Message::Text(msg.to_string())).await;
        });

        spawn_local(async move {
            let mut reader = reader.lock().await;
            while let Some(msg) = reader.next().await {
                info!("{}", format!("receive {:?}", msg))
            }
        });
        Ok(())
    }
}
