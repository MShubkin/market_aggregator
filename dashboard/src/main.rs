use crate::common::{create_html_error_message, MarketResult};
use crate::components::dashboard::DashboardComponent;
use futures::{SinkExt, StreamExt};
use gloo::console;
use gloo_net::websocket::{futures::WebSocket, Message};
use js_sys::Date;
use log::{error, info};
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew::{html, Component, Context, Html};

mod common;
mod components;

#[function_component]
pub fn App() -> Html {
    html! {
        <DashboardComponent/>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    info!("Start Market Aggregator Application");

    yew::Renderer::<App>::new().render();
}
