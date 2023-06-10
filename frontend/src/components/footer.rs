use yew::prelude::*;
use chrono::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    use yew::{classes, html};

    let current_year = Local::now().year();

    html! {
        <footer>
            <p class={classes!("text-center", "fixed-bottom", "bg-body-tertiary", "p-2", "mx-auto", "rounded-3")} style="width: fit-content;">{ format!("© {} Expense Tracker", current_year) }</p>
        </footer>
    }
}