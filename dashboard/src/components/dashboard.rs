use crate::common::web_socket_service::{WebSocketService};
use futures::{SinkExt, StreamExt};
use gloo::console;
use gloo_net::websocket::futures::WebSocket;
use gloo_net::websocket::Message;
use log::{error, info};
use wasm_bindgen_futures::spawn_local;
use yew::{html, use_state, AttrValue, Component, Context, Html, Properties};
use crate::common::MarketResult;

#[derive(Properties, PartialEq)]
pub struct DashboardProps {}

pub struct DashboardComponent {
    //msg: String,
}

pub enum DashboardMessage {
    WebSocketResponse(AttrValue),
}

impl Component for DashboardComponent {
    type Message = DashboardMessage;
    type Properties = DashboardProps;

    fn create(ctx: &Context<Self>) -> Self {
        info!("DashboardComponent create method start");
        info!("Openning web socket connection...");

        let ws_soket_result = WebSocketService::open_ws_connection();

        match ws_soket_result {
            Ok(mut web_socket) => {
                info!("Websocket connection opened successfully!");

                let response_callback = ctx.link().callback(DashboardMessage::WebSocketResponse);

                info!("Subscribe to real time prices!");
                let v = web_socket.subscribe_real_time_rates(response_callback);

            }
            Err(error) => {
                error!("Web socket connection Error: {:?}", error);
            }
        }

        info!("DashboardComponent create method end");
        Self {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        info!("DashboardComponent update method");
        match msg {
            DashboardMessage::WebSocketResponse(data) => {
                //info!("web socket response: {}", data.as_str());
            },
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        info!("DashboardComponent view method");
        html! {
            <>
            {"hello!!!"}
            </>
        }
    }
}
