use std::collections::HashSet;
use std::rc::Rc;
use std::time::Duration;

use futures::lock::Mutex;
use futures::stream::{SplitSink, SplitStream};
use futures::{SinkExt, StreamExt};
use gloo_net::websocket::futures::WebSocket;
use gloo_net::websocket::Message;
use wasm_bindgen_futures::spawn_local;
use yew::platform::time::sleep;
use yew::Callback;

use crate::common::env::{MARKET_API_KEY, MARKET_REAL_TIME_PRICE_ROUTE, MARKET_WS_ADDRESS};
use crate::common::error::MarketError;
use crate::common::utils::prepare_symbols_for_url;
use crate::common::MarketResult;

const NINE_SEC: Duration = Duration::from_secs(9);
/// Twelve Data Web Socket Client
pub struct WebSocketService {
    web_socket_writer: Rc<Mutex<SplitSink<WebSocket, Message>>>,
    web_socket_reader: Rc<Mutex<SplitStream<WebSocket>>>,
}

impl WebSocketService {
    /// Opening web socket connection
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
            web_socket_writer: Rc::new(Mutex::new(web_socket_writer)),
            web_socket_reader: Rc::new(Mutex::new(web_socket_reader)),
        })
    }
    /// This function subscribes and gets real-time price streaming from the exchange.
    pub fn subscribe_real_time_rates(
        &mut self,
        symbols: HashSet<String>,
        success_response_callback: Callback<String>,
        error_response_callback: Callback<MarketError>,
    ) -> MarketResult<()> {
        let msg = format!(
            "{{
                \"action\": \"subscribe\",
                \"params\": {{
                \"symbols\": \"{}\"
                            }}
        }}",
            prepare_symbols_for_url(symbols)
        );

        let writer = self.web_socket_writer.clone();
        let reader = self.web_socket_reader.clone();
        let error_callback = error_response_callback.clone();
        spawn_local(async move {
            let mut writer = writer.lock().await;
            let result = writer.send(Message::Text(msg.to_string())).await;
            if let Err(error) = result {
                error_callback.emit(MarketError::WebSocketError(error))
            }
        });
        let error_callback_reader = error_response_callback.clone();
        spawn_local(async move {
            let mut reader = reader.lock().await;
            while let Some(msg) = reader.next().await {
                match msg {
                    Ok(message) => match message {
                        Message::Text(text) => {
                            success_response_callback.emit(text);
                        }
                        Message::Bytes(_) => {
                            error_callback_reader.emit(MarketError::General(
                                "Bytes message is not supported".to_string(),
                            ));
                        }
                    },
                    Err(error) => error_callback_reader.emit(MarketError::WebSocketError(error)),
                }
            }
        });
        Ok(())
    }
    /// Sending "heartbeat" events to the server every 9 seconds. This will make sure to keep the connection stable
    pub fn heartbeat(&mut self, error_callback: Callback<MarketError>) -> MarketResult<()> {
        let msg = "{{
                \"action\": \"heartbeat\"
        }}"
        .to_string();
        let writer = self.web_socket_writer.clone();
        spawn_local(async move {
            loop {
                let mut writer = writer.lock().await;
                let result = writer.send(Message::Text(msg.to_string())).await;
                if let Err(error) = result {
                    error_callback.emit(MarketError::WebSocketError(error))
                }
                sleep(NINE_SEC).await;
            }
        });
        Ok(())
    }
}
