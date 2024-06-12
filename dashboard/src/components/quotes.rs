use std::collections::HashMap;
use std::sync::Arc;

use linked_hash_set::LinkedHashSet;
use yew::{classes, function_component, html, Html, Properties};

use crate::common::entities::ReferenceData;
use crate::common::enums::{QuoteType, QuotesComponentType};
use crate::common::utils::{format_time, round_f64, round_f64_str};

#[derive(Properties, PartialEq, Clone)]
pub(crate) struct QuotesProps {
    pub(crate) title: String,
    pub(crate) component_type: QuotesComponentType,
    pub(crate) quote_type: QuoteType,
    pub(crate) symbols: Arc<LinkedHashSet<String>>,
    pub(crate) prices: HashMap<String, PriceData>,
    pub(crate) reference_data: Arc<ReferenceData>,
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
    let mut container_classes = vec!["container"];
    let mut title_classes = vec!["title"];

    match props.component_type {
        QuotesComponentType::BidAsk => {
            container_classes.push("bid-ask-comp");
            title_classes.push("bid-ask-comp");
        }
        QuotesComponentType::OnlyPrice => {
            container_classes.push("only-price-comp");
            title_classes.push("bid-ask-comp");
        }
    }

    html! {
           <div class={classes!(container_classes)}>
             <ul class="responsive-table">
               <li class={classes!(title_classes)}>
                 <div>{props.title.clone()}</div>
               </li>
               <li class="table-header">
                 <div class="col col-1">{"Инструмент"}</div>
                 <div class="col col-2">{"Цена"}</div>
                    if props.component_type == QuotesComponentType::BidAsk {
                       <div class="col col-3">{"Покупка"}</div>
                       <div class="col col-4">{"Продажа"}</div>
                    }
                 <div class="col col-5">{"Изм."}</div>
                 <div class="col col-6">{"Изм. %"}</div>
                 <div class="col col-7">{"Время"}</div>
               </li>
           {
               props.symbols.iter().map(|symbol| {

                   let symbol_display = get_symbol_name(symbol, &props.quote_type, &props.reference_data);

                   let mut price_value ="".to_owned();
                   let mut bid_value ="".to_owned();
                   let mut ask_value ="".to_owned();
                   let mut change_value ="".to_owned();
                   let mut percentage_value ="".to_owned();
                   let mut time_value ="".to_owned();

               let mut change_classes = vec!["col", "col-5"];
               let mut change_percent_classes = vec!["col", "col-6"];

                   let mut eod_price:Option<f64> = None;
                   if let Some(end_of_day) = props.reference_data.end_of_day.get(symbol){
                       let val = round_f64_str(end_of_day.close.clone());
                       eod_price = Some(val);
                   }

                   if let Some(last_quote) = props.reference_data.last_quote.get(symbol){
                        price_value = round_f64_str(last_quote.close.clone()).to_string();
                        bid_value = round_f64_str(last_quote.close.clone()).to_string();
                        ask_value = round_f64_str(last_quote.close.clone()).to_string();
                        time_value =  format_time(last_quote.timestamp);

                        if round_f64_str(last_quote.change.clone()) == 0.00{
                           "0.00".clone_into(&mut change_value);
                           "0.00".clone_into(&mut percentage_value);
                        }else if round_f64_str(last_quote.change.clone()) > 0.00{
                           change_value = format!("+{}",round_f64_str(last_quote.change.clone()));
                           percentage_value =  format!("+{}",round_f64_str(last_quote.percent_change.clone()));
                           change_classes.push("color-green");
                           change_percent_classes.push("color-green");
                        }else{
                           change_value = format!("{}",round_f64_str(last_quote.change.clone()));
                           percentage_value =  format!("{}",round_f64_str(last_quote.percent_change.clone()));
                           change_classes.push("color-red");
                           change_percent_classes.push("color-red");
                        }
                   }

                if let Some(price_data) = props.prices.get(symbol).cloned(){
                   price_value =  round_f64(price_data.price).to_string();
                   time_value.clone_from(&price_data.time);
                   bid_value =  round_f64(price_data.bid).to_string();
                   ask_value =  round_f64(price_data.ask).to_string();

                   if let Some(eod_price) = eod_price{
                       let change = price_data.price - eod_price;
                       if change == 0.00{
                           "0.00".clone_into(&mut change_value);
                           "0.00".clone_into(&mut percentage_value);
                       }else if change > 0.00{
                           change_value = format!("+{}",round_f64(change));
                           let perc = (change/eod_price)*100.00;
                           percentage_value =  format!("+{}",round_f64(perc));
                           change_classes.push("color-green");
                           change_percent_classes.push("color-green");
                       }else if change < 0.00{
                           change_value = format!("{}", round_f64(change));
                           let perc = (change/eod_price)*100.00;
                           percentage_value = format!("{}",round_f64(perc));
                           change_classes.push("color-red");
                           change_percent_classes.push("color-red");
                       }
                   }
                }
                   html!{
                       <li class="table-row">
                         <div class="col col-1" data-label="Инструмент">{symbol_display}</div>
                         <div class="col col-2" data-label="Цена">{price_value}</div>
                          if props.component_type == QuotesComponentType::BidAsk {
                            <div class="col col-3" data-label="Покупка">{bid_value}</div>
                            <div class="col col-4" data-label="Продажа">{ask_value}</div>
                          }
                         <div class={classes!(change_classes)} data-label="Изм.">{change_value}</div>
                         <div class={classes!(change_percent_classes)} data-label="Изм. %">{percentage_value}</div>
                         <div class="col col-7" data-label="Время">{time_value}</div>
                       </li>
                   }
               }).collect::<Html>()
           }
             </ul>
           </div>
    }
}

fn get_symbol_name(
    symbol: &String,
    quote_type: &QuoteType,
    reference_data: &ReferenceData,
) -> String {
    match quote_type {
        QuoteType::Indices => {
            if let Some(indice) = reference_data.indices.get(symbol) {
                indice.name.clone()
            } else {
                symbol.clone()
            }
        }
        QuoteType::USStocks => {
            if let Some(indice) = reference_data.us_stocks.get(symbol) {
                indice.name.clone()
            } else {
                symbol.clone()
            }
        }
        _ => symbol.clone(),
    }
}
