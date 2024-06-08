pub enum QuoteType {
    CryptoCurrency,
    Currency,
    Indices,
    USStocks,
}

#[derive(PartialEq, Clone, Debug)]
pub enum QuotesComponentType {
    BidAsk,
    OnlyPrice,
}
impl Default for QuotesComponentType {
    fn default() -> Self {
        QuotesComponentType::BidAsk
    }
}
