use std::fmt::{Display, Formatter};

use derive_more::From;
use gloo::utils::errors::JsError;
use gloo_net::Error;
use thiserror::Error;
use yew::{html, Html};

#[derive(Debug, Error, From)]
pub enum MarketError {
    #[error("JsError error: {0}")]
    JsError(JsError),
    #[error("Env error: {0}")]
    EnvError(EnvErrorType),
    #[error("Gloo net error: {0}")]
    GlooNetError(Error),
    #[error("Serde json error: {0}")]
    SerdeJsonError(serde_json::error::Error),
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
