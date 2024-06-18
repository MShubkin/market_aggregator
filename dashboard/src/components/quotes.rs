use std::collections::HashMap;
use std::sync::Arc;

use linked_hash_set::LinkedHashSet;
use yew::{classes, function_component, html, Html, Properties};

use crate::common::entities::{RealTimePriceData, ReferenceData};
use crate::common::enums::{QuoteType, QuotesComponentType};
use crate::common::utils::{format_time, round_f64, round_f64_str};

/// Quotes Component Properties
#[derive(Properties, PartialEq, Clone)]
pub struct QuotesProps {
    /// Title of Quote Component
    pub title: String,
    /// Component Type
    pub component_type: QuotesComponentType,
    /// Type of quote
    pub quote_type: QuoteType,
    /// List of symbols
    pub symbols: Arc<LinkedHashSet<String>>,
    /// Real Time prices
    pub prices: HashMap<String, RealTimePriceData>,
    /// Reference Data
    pub reference_data: Arc<ReferenceData>,
}

#[function_component]
pub fn QuotesComponent(props: &QuotesProps) -> Html {
    let css_props = get_css_props(props);

    html! {
           <div class={classes!(css_props.container_classes)}>
             <ul class="responsive-table">
               <li class={classes!(css_props.title_classes)}>
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
                   let mut price_data = DisplayPriceData::default();
                   fill_last_quote(&mut price_data, symbol, props);
                   fill_current_quote(&mut price_data, symbol, props);
                   html!{
                       <li class="table-row">
                         <div class="col col-1" data-label="Инструмент">{symbol_display}</div>
                         <div class="col col-2" data-label="Цена">{price_data.price_value}</div>
                          if props.component_type == QuotesComponentType::BidAsk {
                            <div class="col col-3" data-label="Покупка">{price_data.bid_value}</div>
                            <div class="col col-4" data-label="Продажа">{price_data.ask_value}</div>
                          }
                         <div class={classes!(price_data.change_classes)} data-label="Изм.">{price_data.change_value}</div>
                         <div class={classes!(price_data.change_percent_classes)} data-label="Изм. %">{price_data.percentage_value}</div>
                         <div class="col col-7" data-label="Время">{price_data.time_value}</div>
                       </li>
                   }
               }).collect::<Html>()
           }
             </ul>
           </div>
    }
}

#[derive(Default)]
struct DisplayPriceData {
    price_value: String,
    bid_value: String,
    ask_value: String,
    change_value: String,
    percentage_value: String,
    time_value: String,
    change_classes: Vec<&'static str>,
    change_percent_classes: Vec<&'static str>,
}

fn fill_current_quote(
    display_price_data: &mut DisplayPriceData,
    symbol: &String,
    props: &QuotesProps,
) {
    let eod_price = get_eod_price(symbol, props);
    if let Some(price_data) = props.prices.get(symbol).cloned() {
        display_price_data.price_value = round_f64(price_data.price).to_string();
        display_price_data.time_value.clone_from(&price_data.time);

        if price_data.bid == 0.00 {
            display_price_data.bid_value = round_f64(price_data.price).to_string();
        } else {
            display_price_data.bid_value = round_f64(price_data.bid).to_string();
        }
        if price_data.ask == 0.00 {
            display_price_data.ask_value = round_f64(price_data.price).to_string();
        } else {
            display_price_data.ask_value = round_f64(price_data.ask).to_string();
        }
        if let Some(eod_price) = eod_price {
            let change = price_data.price - eod_price;
            if change == 0.00 {
                "0.00".clone_into(&mut display_price_data.change_value);
                "0.00".clone_into(&mut display_price_data.percentage_value);
            } else if change > 0.00 {
                display_price_data.change_value = format!("+{}", round_f64(change));
                let perc = (change / eod_price) * 100.00;
                display_price_data.percentage_value = format!("+{}", round_f64(perc));
                display_price_data.change_classes.push("color-green");
                display_price_data
                    .change_percent_classes
                    .push("color-green");
            } else if change < 0.00 {
                display_price_data.change_value = format!("{}", round_f64(change));
                let perc = (change / eod_price) * 100.00;
                display_price_data.percentage_value = format!("{}", round_f64(perc));
                display_price_data.change_classes.push("color-red");
                display_price_data.change_percent_classes.push("color-red");
            }
        }
    }
}

fn fill_last_quote(price_data: &mut DisplayPriceData, symbol: &String, props: &QuotesProps) {
    let mut change_classes = vec!["col", "col-5"];
    let mut change_percent_classes = vec!["col", "col-6"];
    if let Some(last_quote) = props.reference_data.last_quote.get(symbol) {
        price_data.price_value = round_f64_str(last_quote.close.clone()).to_string();
        price_data.bid_value = round_f64_str(last_quote.close.clone()).to_string();
        price_data.ask_value = round_f64_str(last_quote.close.clone()).to_string();
        price_data.time_value = format_time(last_quote.timestamp);
        if round_f64_str(last_quote.change.clone()) == 0.00 {
            "0.00".clone_into(&mut price_data.change_value);
            "0.00".clone_into(&mut price_data.percentage_value);
        } else if round_f64_str(last_quote.change.clone()) > 0.00 {
            price_data.change_value = format!("+{}", round_f64_str(last_quote.change.clone()));
            price_data.percentage_value =
                format!("+{}", round_f64_str(last_quote.percent_change.clone()));
            change_classes.push("color-green");
            change_percent_classes.push("color-green");
        } else {
            price_data.change_value = format!("{}", round_f64_str(last_quote.change.clone()));
            price_data.percentage_value =
                format!("{}", round_f64_str(last_quote.percent_change.clone()));
            change_classes.push("color-red");
            change_percent_classes.push("color-red");
        }
    }
    price_data.change_classes = change_classes;
    price_data.change_percent_classes = change_percent_classes;
}

fn get_eod_price(symbol: &String, props: &QuotesProps) -> Option<f64> {
    let mut eod_price: Option<f64> = None;
    if let Some(end_of_day) = props.reference_data.end_of_day.get(symbol) {
        let val = round_f64_str(end_of_day.close.clone());
        eod_price = Some(val);
    }
    eod_price
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

struct CssProps {
    container_classes: Vec<&'static str>,
    title_classes: Vec<&'static str>,
}

fn get_css_props(props: &QuotesProps) -> CssProps {
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
    CssProps {
        container_classes,
        title_classes,
    }
}
