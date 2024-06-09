use futures::{SinkExt, StreamExt};
use gloo::console;
use gloo_net::websocket::{futures::WebSocket, Message};
use js_sys::Date;
use log::{error, info};
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew::{html, Component, Context, Html};

use crate::common::MarketResult;
use crate::components::dashboard::{AppContent, DashboardComponent, WithLoadingData};

mod common;
mod components;
mod services;

#[function_component]
pub fn App() -> Html {
    //let fallback = html! {<PleaseWait/>};
    let fallback = html! {<div>{"Loading..."}</div>};
    /*
        html! {
                   <Suspense fallback={fallback}>
                            <WithLoadingData/>
                    </Suspense>
        }
    */
    html! {
        <Suspense {fallback}>
            <AppContent />
        </Suspense>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    info!("Start Market Aggregator Application");

    yew::Renderer::<App>::new().render();
}

#[function_component(PleaseWait)]
fn please_wait() -> Html {
    html! {<h1>{"Loading data..."}</h1>}
}
