use gloo_net::http::Request;  // Import Request from the gloo_net crate for making HTTP requests
use serde::{Deserialize, Serialize};  // Import Deserialize and Serialize from the serde crate for (de)serialization of data
use serde_wasm_bindgen::to_value;  // Import to_value function for converting data into JsValue
use web_sys::{console, window, HtmlInputElement};  // Import some web_sys entities for interacting with the web API
use yew::prelude::*;  // Import all entities from the prelude module of the Yew crate
use yew_router::prelude::*;  // Import all entities from the prelude module of the yew_router crate

#[derive(Clone, PartialEq, Properties, Serialize, Deserialize)]
struct FormData {
    name: String,
    budget_amount: f64,
}

#[function_component(AddGroupForm)]
pub fn add_group_form() -> Html {
    let input_name_ref = use_node_ref();
    let input_number_ref = use_node_ref();

    let form_state = use_state(|| FormData {
        name: String::from(""),
        budget_amount: 0.0,
    });

    let on_name_change = {
        let form_state = form_state.clone();
        let input_name_ref = input_name_ref.clone();
        Callback::from(move |_| {
            if let Some(input) = input_name_ref.cast::<HtmlInputElement>() {
                let value = input.value();
                // let form_state = form_state.clone();
                form_state.set(FormData {
                    name: value,
                    budget_amount: form_state.budget_amount,
                });
            }
        })
    };

    let on_number_change = {
        let form_state = form_state.clone();
        let input_number_ref = input_number_ref.clone();
        Callback::from(move |_| {
            if let Some(input) = input_number_ref.cast::<HtmlInputElement>() {
                let value = input.value();
                // let form_state = form_state.clone();
                if let Ok(parsed_value) = value.parse::<f64>() {
                    form_state.set(FormData {
                        name: form_state.name.clone(),
                        budget_amount: parsed_value,
                    });
                }
            }
        })
    };

    let onsubmit = {
        Callback::from(move |event: yew::events::SubmitEvent| {
            event.prevent_default();
            let form_state = form_state.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let data = FormData {
                    name: form_state.name.clone(),
                    budget_amount: form_state.budget_amount,
                };

                let resp = Request::post("http://localhost:5001/expense/add-group")
                    .json(&data)
                    .unwrap()
                    .send()
                    .await
                    .unwrap();

                if resp.status() != 200 {
                    let jsvalueresp = to_value(&resp.status()).unwrap();
                    console::log_1(&jsvalueresp);
                }

                let window = window().expect("error getting window");
                let location = window.location();
                let _ = location.set_href("/expense");
            });
        })
    };

    html! {
    <form class="mx-auto" style="width: 250px;" onsubmit={onsubmit}>
        <div class="mb-3 text-end">
            <label for="budgetName" class="form-label">{ "Budget Name" }</label>
            <input type="text" class="form-control text-end" id="budgetName" aria-describedby="budgetNameHelp" ref={input_name_ref} onchange={on_name_change}/>
            <div id="budgetNameHelp" class="form-text">{ "Name your budget" }</div>
        </div>
        <div class="mb-3 text-end">
            <label for="amount" class="form-label">{ "The Cost for budget" }</label>
            <input type="text" class="form-control text-end" id="amount" placeholder="Number up to 2 decimal places" pattern="[0-9]*[.]?[0-9]{0,2}" ref={input_number_ref} onchange={on_number_change} />
        </div>
        <div class="d-flex justify-content-end">
            <button type="reset" class="btn btn-secondary me-1">{ "Reset" }</button>
            <button type="submit" class="btn btn-primary ms-1">{ "Submit" }</button>
        </div>
    </form>
    }
}

#[function_component(AddGroup)]
pub fn add_group() -> Html {
    let navigator = use_navigator().unwrap();

    let add_button = {
        let navigator = navigator;
        Callback::from(move |_| navigator.push(&super::super::ExpenseRoute::AddGroupForm))
    };

    html! {
        <>
            <div class="d-flex justify-content-center sticky-bottom" style="padding-bottom: 80px;">
                <button type="button" class="btn btn-primary fs-3 btn-lg" onclick={add_button}>{ "+" }</button>
            </div>
        </>
    }
}
