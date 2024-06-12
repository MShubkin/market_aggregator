use yew::platform::spawn_local;
use yew::prelude::*;
use yew::suspense::{Suspension, SuspensionHandle, SuspensionResult};

use crate::common::config::DashboardConfiguration;
use crate::common::entities::ReferenceData;
use crate::services::restapi::RestApiService;

/// State for Suspense Component
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

/// Loading reference market data
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

fn load_data(state: UseStateHandle<LoadDataState>) {
    spawn_local(async move {
        let indices = RestApiService::get_indices().await.unwrap();
        let us_stocks = RestApiService::get_us_stoks().await.unwrap();
        let end_of_day =
            RestApiService::get_end_of_day_data(DashboardConfiguration::get_all_quote_symbols())
                .await
                .unwrap();
        let last_quote =
            RestApiService::get_last_quote(DashboardConfiguration::get_all_quote_symbols())
                .await
                .unwrap();

        state.set(LoadDataState {
            suspension: state.suspension.clone(),
            handle: None, //drop handler and resume render
            reference_data: ReferenceData {
                indices,
                us_stocks,
                end_of_day,
                last_quote,
            },
        });
    });
}
