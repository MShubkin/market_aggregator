//pub mod env;
pub mod env;
pub mod web_socket_service;

use derive_more::From;
use gloo::utils::errors::JsError;
use std::fmt::{Display, Formatter};
use thiserror::Error;
use yew::{html, Html};

pub type MarketResult<T> = std::result::Result<T, MarketError>;

#[derive(Debug, Error, From)]
pub enum MarketError {
    #[error("JsError error: {0}")]
    JsError(JsError),
    #[error("Env error: {0}")]
    EnvError(EnvErrorType),
}

#[derive(Debug, Clone)]
pub enum EnvErrorType {
    CannotLoadEnv(String),
    EnvWasNotSet(String),
    EnvHasWrongFormat(String),
}

impl Display for EnvErrorType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            EnvErrorType::CannotLoadEnv(msg) => {
                write!(f, "{}", msg)
            }
            EnvErrorType::EnvWasNotSet(msg) => {
                write!(f, "{}", msg)
            }
            EnvErrorType::EnvHasWrongFormat(msg) => {
                write!(f, "{}", msg)
            }
        }
    }
}

pub fn create_html_error_message(error: MarketError) -> Html {
    html! {
        <h1>{error}</h1>
    }
}