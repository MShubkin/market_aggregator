use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use linked_hash_set::LinkedHashSet;
use log::{error, info};
use yew::{
    function_component, html, BaseComponent, Component, Context, Html, HtmlResult, Properties,
};

use crate::common::config::DashboardConfiguration;
use crate::common::entities::{PriceMessage, RealTimePriceData, ReferenceData, WSResponseEvent};
use crate::common::enums::{QuoteType, QuotesComponentType, WSResponseEventType};
use crate::common::error::MarketError;
use crate::common::utils::format_time;
use crate::components::quotes::{QuotesComponent, QuotesProps};
use crate::components::suspense::use_load_data;
use crate::services::websocket::WebSocketService;

pub type AppContent = WithLoadingData<DashboardComponent>;

/// Function component responsible for loading reference data
#[function_component]
pub fn WithLoadingData<Comp>() -> HtmlResult
where
    Comp: BaseComponent<Properties = DashboardComponentProps>,
{
    let reference_data = use_load_data()?;

    Ok(
        yew::virtual_dom::VChild::<Comp>::new(DashboardComponentProps { reference_data }, None)
            .into(),
    )
}

/// Dashboard Component Properties
#[derive(Debug, PartialEq, Properties)]
pub struct DashboardComponentProps {
    reference_data: ReferenceData,
}

/// Dashboard struct Component
pub struct DashboardComponent {
    crypto_currencies_symbols: Arc<LinkedHashSet<String>>,
    currencies_symbols: Arc<LinkedHashSet<String>>,
    indices_symbols: Arc<LinkedHashSet<String>>,
    us_stocks_symbols: Arc<LinkedHashSet<String>>,
    prices: Arc<RwLock<HashMap<String, RealTimePriceData>>>,
    reference_data: Arc<ReferenceData>,
}
/// Dashboard Component Messages
pub enum DashboardMessage {
    /// Web socket response message
    WebSocketResponse(String),
    /// Market error message
    MarketError(MarketError),
}

impl Component for DashboardComponent {
    type Message = DashboardMessage;
    type Properties = DashboardComponentProps;

    fn create(ctx: &Context<Self>) -> Self {
        let ws_soket_connection_result = WebSocketService::open_ws_connection();
        match ws_soket_connection_result {
            Ok(mut web_socket) => {
                let success_response_callback =
                    ctx.link().callback(DashboardMessage::WebSocketResponse);
                let error_response_callback = ctx.link().callback(DashboardMessage::MarketError);
                let subscribe_result = web_socket.subscribe_real_time_rates(
                    DashboardConfiguration::get_all_quote_symbols(),
                    success_response_callback,
                    error_response_callback.clone(),
                );
                match subscribe_result {
                    Ok(_) => {
                        info!("Web socket subscribe Success");
                    }
                    Err(error) => {
                        error!("Web socket subscribe Error: {:?}", error);
                    }
                }

                let heartbeat_result = web_socket.heartbeat(error_response_callback.clone());
                match heartbeat_result {
                    Ok(()) => {
                        info!("Web socket heartbeat Success");
                    }
                    Err(error) => {
                        error!("Web socket heartbeat Error: {:?}", error);
                    }
                }
            }
            Err(error) => {
                error!("Web socket connection Error: {:?}", error);
            }
        }
        Self {
            crypto_currencies_symbols: Arc::new(
                DashboardConfiguration::get_crypto_currencies_symbols(),
            ),
            currencies_symbols: Arc::new(DashboardConfiguration::get_currencies_symbols()),
            indices_symbols: Arc::new(DashboardConfiguration::get_indices_symbols()),
            us_stocks_symbols: Arc::new(DashboardConfiguration::get_us_stocks()),
            prices: Arc::new(RwLock::new(HashMap::new())),
            reference_data: Arc::new(ctx.props().reference_data.clone()),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            DashboardMessage::WebSocketResponse(data) => {
                let response_event: WSResponseEvent =
                    serde_json::from_str(data.as_str()).unwrap_or_default();

                match WSResponseEventType::from(response_event.event) {
                    WSResponseEventType::SubscribeStatus => {
                        info!("subscribe status {:?}", data);
                    }
                    WSResponseEventType::Price => {
                        let price_message: PriceMessage =
                            serde_json::from_str(data.as_str()).unwrap();
                        info!("price_message {:?}", price_message);
                        let mut lock = self.prices.write().unwrap();
                        lock.insert(
                            price_message.symbol.clone(),
                            RealTimePriceData {
                                symbol: price_message.symbol,
                                price: price_message.price,
                                bid: price_message.bid,
                                ask: price_message.ask,
                                time: format_time(price_message.timestamp),
                                ..Default::default()
                            },
                        );
                    }
                    WSResponseEventType::Heartbeat => {
                        info!("heart beat status {:?}", data);
                    }
                    WSResponseEventType::Unknown => {
                        info!("unknown message {:?}", data);
                    }
                }
            }
            DashboardMessage::MarketError(error) => {
                info!("MarketErrorResponse response {}", error);
            }
        }
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let crypto_currencies_props = QuotesProps {
            title: "Крипто-валюты".to_owned(),
            component_type: QuotesComponentType::BidAsk,
            quote_type: QuoteType::CryptoCurrency,
            symbols: self.crypto_currencies_symbols.clone(),
            prices: self.get_quote_data(QuoteType::CryptoCurrency),
            reference_data: self.reference_data.clone(),
        };
        let currencies_props = QuotesProps {
            title: "Мировые валюты".to_owned(),
            component_type: QuotesComponentType::BidAsk,
            quote_type: QuoteType::Currency,
            symbols: self.currencies_symbols.clone(),
            prices: self.get_quote_data(QuoteType::Currency),
            reference_data: self.reference_data.clone(),
        };
        let indices_stock_props = QuotesProps {
            title: "Индексы".to_owned(),
            component_type: QuotesComponentType::OnlyPrice,
            quote_type: QuoteType::Indices,
            symbols: self.indices_symbols.clone(),
            prices: self.get_quote_data(QuoteType::Indices),
            reference_data: self.reference_data.clone(),
        };
        let us_stock_props = QuotesProps {
            title: "Акции".to_owned(),
            component_type: QuotesComponentType::OnlyPrice,
            quote_type: QuoteType::USStocks,
            symbols: self.us_stocks_symbols.clone(),
            prices: self.get_quote_data(QuoteType::USStocks),
            reference_data: self.reference_data.clone(),
        };
        html! {
              <div class="row">
                  <div class="column1">
                        <div><QuotesComponent ..crypto_currencies_props.clone() /></div>
                        <div><QuotesComponent ..currencies_props.clone() /></div>
                  </div>
                 <div class="column2">
                        <div><QuotesComponent ..indices_stock_props.clone() /></div>
                  </div>
                  <div class="column3">
                        <div><QuotesComponent ..us_stock_props.clone() /></div>
                  </div>
            </div>
        }
    }
}

impl DashboardComponent {
    fn get_quote_data(&self, quote_type: QuoteType) -> HashMap<String, RealTimePriceData> {
        let symbols = DashboardConfiguration::get_quote_symbols(quote_type);
        let data = self.prices.read().unwrap();
        let filter_data = data
            .iter()
            .filter(|value| symbols.contains(value.0))
            .map(|value| (value.0.clone(), value.1.clone()))
            .collect();
        filter_data
    }
}
