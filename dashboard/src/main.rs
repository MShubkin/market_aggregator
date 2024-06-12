use log::info;
use yew::prelude::*;
use yew::{html, Html};

use crate::components::dashboard::AppContent;

mod common;
mod components;
mod services;

#[function_component]
pub fn App() -> Html {
    let fallback = html! {<div class="center"><div class="loader"></div></div>};
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
