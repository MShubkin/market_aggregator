use derive_more::From;
use gloo::utils::errors::JsError;
use thiserror::Error;

#[derive(Debug, Error, From)]
pub enum MarketError {
    #[error("General error: {0}")]
    General(String),
    #[error("JsError error: {0}")]
    JsError(JsError),
    #[error("Gloo net error: {0}")]
    GlooNetError(gloo_net::Error),
    #[error("Serde json error: {0}")]
    SerdeJsonError(serde_json::error::Error),
    #[error("Web socket error: {0}")]
    WebSocketError(gloo_net::websocket::WebSocketError),
}
