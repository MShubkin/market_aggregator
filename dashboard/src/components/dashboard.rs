use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use std::sync::{Arc, RwLock};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use chrono::{DateTime, FixedOffset, NaiveDateTime, Utc};
use futures::{SinkExt, StreamExt};
use gloo::console;
use gloo_net::websocket::futures::WebSocket;
use gloo_net::websocket::Message;
use linked_hash_set::LinkedHashSet;
use log::{error, info};
use wasm_bindgen_futures::spawn_local;
use wasm_logger::init;
use yew::platform::time::sleep;
use yew::suspense::{Suspension, SuspensionResult};
use yew::{
    function_component, hook, html, use_effect_with_deps, use_state, AttrValue, BaseComponent,
    Component, Context, Html, HtmlResult, Properties, Reducible, UseStateHandle,
};

use crate::common::config::DashboardConfiguration;
use crate::common::entities::{EndOfDay, Quote, ReferenceData};
use crate::common::enums::{QuoteType, QuotesComponentType};
use crate::common::error::MarketError;
use crate::common::utils::format_time;
use crate::common::MarketResult;
use crate::components::quotes::{PriceData, QuotesComponent, QuotesProps};
use crate::components::use_sleep::use_load_data;
use crate::services::restapi::RestApiService;
use crate::services::websocket::{
    PriceMessage, WSResponseEvent, WSResponseEventType, WebSocketService,
};

pub type AppContent = WithLoadingData<DashboardComponent>;

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

#[derive(PartialEq, Default)]
pub struct ReferenceDataState {
    reference_data: Option<ReferenceData>,
}

#[derive(Debug, PartialEq, Properties)]
pub struct DashboardComponentProps {
    reference_data: ReferenceData,
}

pub struct DashboardComponent {
    crypto_currencies_symbols: Arc<LinkedHashSet<String>>,
    currencies_symbols: Arc<LinkedHashSet<String>>,
    indices_symbols: Arc<LinkedHashSet<String>>,
    us_stocks_symbols: Arc<LinkedHashSet<String>>,
    prices: Arc<RwLock<HashMap<String, PriceData>>>,
    end_of_day: Arc<HashMap<String, EndOfDay>>,
    last_quote: Arc<HashMap<String, Quote>>,
    reference_data: Arc<ReferenceData>,
}

pub enum DashboardMessage {
    EndOfDayResponse(HashMap<String, EndOfDay>),
    LastQuoteResponse(HashMap<String, Quote>),
    WebSocketResponse(String),
    MarketErrorResponse(MarketError),
}

impl Component for DashboardComponent {
    type Message = DashboardMessage;
    type Properties = DashboardComponentProps;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_future(async {
            match RestApiService::get_end_of_day_data(
                DashboardConfiguration::get_all_quote_symbols(),
            )
            .await
            {
                Ok(data) => DashboardMessage::EndOfDayResponse(data),
                Err(err) => DashboardMessage::MarketErrorResponse(err),
            }
        });

        ctx.link().send_future(async {
            match RestApiService::get_last_quote(DashboardConfiguration::get_all_quote_symbols())
                .await
            {
                Ok(data) => DashboardMessage::LastQuoteResponse(data),
                Err(err) => DashboardMessage::MarketErrorResponse(err),
            }
        });

        let ws_soket_result = WebSocketService::open_ws_connection();
        match ws_soket_result {
            Ok(mut web_socket) => {
                let response_callback = ctx.link().callback(DashboardMessage::WebSocketResponse);
                web_socket
                    .subscribe_real_time_rates(
                        DashboardConfiguration::get_all_quote_symbols(),
                        response_callback,
                    )
                    .expect("cannot subscribe to real time rates");
                web_socket.heartbeat().expect("heartbeat error");
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
            end_of_day: Default::default(),
            last_quote: Arc::new(Default::default()),
            reference_data: Arc::new(ctx.props().reference_data.clone()),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
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
                            serde_json::from_str(data.as_str()).unwrap_or_default();
                        info!("price_message {:?}", price_message);
                        let mut lock = self.prices.write().unwrap();
                        lock.insert(
                            price_message.symbol.clone(),
                            PriceData {
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

            DashboardMessage::EndOfDayResponse(data) => {
                self.end_of_day = Arc::new(data);
                return false;
            }
            DashboardMessage::LastQuoteResponse(data) => {
                self.last_quote = Arc::new(data);
            }
            DashboardMessage::MarketErrorResponse(error) => {
                info!("MarketErrorResponse response {}", error);
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let crypto_currencies_props = QuotesProps {
            title: "Крипто-валюты".to_owned(),
            component_type: QuotesComponentType::BidAsk,
            quote_type: QuoteType::CryptoCurrency,
            symbols: self.crypto_currencies_symbols.clone(),
            prices: self.get_quote_data(QuoteType::CryptoCurrency),
            end_of_day: self.end_of_day.clone(),
            last_quote: self.last_quote.clone(),
            reference_data: self.reference_data.clone(),
        };
        let currencies_props = QuotesProps {
            title: "Мировые валюты".to_owned(),
            component_type: QuotesComponentType::BidAsk,
            quote_type: QuoteType::Currency,
            symbols: self.currencies_symbols.clone(),
            prices: self.get_quote_data(QuoteType::Currency),
            end_of_day: self.end_of_day.clone(),
            last_quote: self.last_quote.clone(),
            reference_data: self.reference_data.clone(),
        };
        let indices_stock_props = QuotesProps {
            title: "Индексы".to_owned(),
            component_type: QuotesComponentType::OnlyPrice,
            quote_type: QuoteType::Indices,
            symbols: self.indices_symbols.clone(),
            prices: self.get_quote_data(QuoteType::Indices),
            end_of_day: self.end_of_day.clone(),
            last_quote: self.last_quote.clone(),
            reference_data: self.reference_data.clone(),
        };
        let us_stock_props = QuotesProps {
            title: "Акции".to_owned(),
            component_type: QuotesComponentType::OnlyPrice,
            quote_type: QuoteType::USStocks,
            symbols: self.us_stocks_symbols.clone(),
            prices: self.get_quote_data(QuoteType::USStocks),
            end_of_day: self.end_of_day.clone(),
            last_quote: self.last_quote.clone(),
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
    fn get_quote_data(&self, quote_type: QuoteType) -> HashMap<String, PriceData> {
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
