use std::collections::HashMap;
use std::sync::Arc;

use linked_hash_set::LinkedHashSet;
use yew::{function_component, html, Component, Html, Properties};

#[derive(Properties, PartialEq, Clone)]
pub(crate) struct QuotesProps {
    pub(crate) title: String,
    pub(crate) symbols: Arc<LinkedHashSet<String>>,
    pub(crate) prices: HashMap<String, PriceData>,
}

/// Ценовая информация финансового инструмента
#[derive(PartialEq, Clone, Default, Debug)]
pub(crate) struct PriceData {
    /// Код инструмента
    pub(crate) symbol: String,
    /// Текущая цена
    pub(crate) price: f64,
    /// Цена покупателя
    pub(crate) bid: f64,
    /// Цена продавца
    pub(crate) ask: f64,
    /// Изменение в цене
    pub(crate) change: f64,
    /// Изменение в цене(в процентах)
    pub(crate) change_percentage: f64,
    /// Время получения данных
    pub(crate) time: String,
}

#[function_component]
pub(crate) fn QuotesComponent(props: &QuotesProps) -> Html {
    html! {
           <div class="container">
             <ul class="responsive-table">
               <li class="title">
                 <div>{props.title.clone()}</div>
               </li>
               <li class="table-header">
                 <div class="col col-1">{"Инструмент"}</div>
                 <div class="col col-2">{"Цена"}</div>
                 <div class="col col-3">{"Покупка"}</div>
                 <div class="col col-4">{"Продажа"}</div>
                 <div class="col col-5">{"Изм."}</div>
                 <div class="col col-6">{"Изм. %"}</div>
                 <div class="col col-7">{"Время"}</div>
               </li>
           {
               props.symbols.iter().map(|symbol| {
                   let price = props.prices.get(symbol).map(|value|value.clone()).unwrap_or_default();
                   html!{
                       <li class="table-row">
                         <div class="col col-1" data-label="Инструмент">{symbol}</div>
                         <div class="col col-2" data-label="Цена">{price.price}</div>
                         <div class="col col-3" data-label="Покупка">{price.bid}</div>
                         <div class="col col-4" data-label="Продажа">{price.ask}</div>
                         <div class="col col-5" data-label="Изм.">{price.change}</div>
                         <div class="col col-6" data-label="Изм. %">{price.change_percentage}</div>
                         <div class="col col-7" data-label="Время">{price.time.clone()}</div>
                       </li>
                   }
               }).collect::<Html>()
           }
             </ul>
           </div>
    }
}
