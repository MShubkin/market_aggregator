use crate::common::error::MarketError;

pub mod config;
pub mod enums;
pub mod env;
pub mod utils;

pub mod entities;
pub mod error;

pub type MarketResult<T> = Result<T, MarketError>;
