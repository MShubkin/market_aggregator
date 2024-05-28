use yew::{function_component, html, Component, Context, Html, Properties};

#[derive(Properties, PartialEq)]
pub(crate) struct CurrencyProps {}

#[function_component]
pub(crate) fn CurrencyComponent(props: &CurrencyProps) -> Html {
    html! {
             <>
    <div class="container">
               <ul class="responsive-table">
                 <li class="table-header">
                   <div class="col col-1">{"Наименование"}</div>
                   <div class="col col-2">{"Покупка"}</div>
                   <div class="col col-3">{"Продажа"}</div>
                   <div class="col col-4">{"Изменение"}</div>
                   <div class="col col-5">{"%"}</div>
                   <div class="col col-6">{"Время"}</div>
                 </li>
                 <li class="table-row">
                   <div class="col col-1" data-label="Job Id">{"42235"}</div>
                   <div class="col col-2" data-label="Customer Name">{"John Doe"}</div>
                   <div class="col col-3" data-label="Amount">{"$350"}</div>
                   <div class="col col-4" data-label="Payment Status">{"Pending"}</div>
                 </li>
               </ul>
             </div>
             </>
         }
}
