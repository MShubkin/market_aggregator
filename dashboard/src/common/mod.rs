use crate::common::error::MarketError;
/// Dashboard Configuration
pub mod config;
/// Common entities
pub mod entities;
/// Common enums
pub mod enums;
/// Env variables
pub mod env;
/// Application Error
pub mod error;
/// Common util functions
pub mod utils;

/// Common result for market aggregator application
pub type MarketResult<T> = Result<T, MarketError>;
