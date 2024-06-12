/// Quote Type
#[derive(PartialEq, Clone)]
pub enum QuoteType {
    CryptoCurrency,
    Currency,
    Indices,
    USStocks,
}
/// Quotes Component Type
#[derive(PartialEq, Clone, Debug, Default)]
pub enum QuotesComponentType {
    #[default]
    BidAsk,
    OnlyPrice,
}
/// Web socket response event type
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
