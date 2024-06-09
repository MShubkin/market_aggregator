use log::info;
use std::rc::Rc;
use std::time::Duration;
use wasm_bindgen::convert::WasmAbi;
use yew::platform::spawn_local;

use crate::common::entities::ReferenceData;
use crate::services::restapi::RestApiService;
use yew::platform::time::sleep;
use yew::prelude::*;
use yew::suspense::{Suspension, SuspensionHandle, SuspensionResult};

#[derive(PartialEq)]
pub struct LoadDataState {
    suspension: Suspension,
    handle: Option<SuspensionHandle>,
    reference_data: ReferenceData,
}

impl LoadDataState {
    fn new() -> Self {
        let (suspension, handle) = Suspension::new();
        Self {
            suspension,
            handle: Some(handle),
            reference_data: Default::default(),
        }
    }
}

impl Reducible for LoadDataState {
    type Action = ();

    fn reduce(self: Rc<Self>, _action: Self::Action) -> Rc<Self> {
        Self::new().into()
    }
}

#[hook]
pub fn use_load_data() -> SuspensionResult<ReferenceData> {
    let load_data_state = use_state(LoadDataState::new);
    if load_data_state.suspension.resumed() {
        Ok(load_data_state.reference_data.clone())
    } else {
        let state = load_data_state.clone();
        load_data(state);
        Err(load_data_state.suspension.clone())
    }
}

pub fn load_data(state: UseStateHandle<LoadDataState>) {
    spawn_local(async move {
        let indices = RestApiService::get_indices().await.unwrap();
        let us_stocks = RestApiService::get_us_stoks().await.unwrap();
        state.set(LoadDataState {
            suspension: state.suspension.clone(),
            handle: None, //drop handler and resume render
            reference_data: ReferenceData { indices, us_stocks},
        });
    });
}
