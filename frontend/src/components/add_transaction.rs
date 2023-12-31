use gloo_net::http::Request;
use serde::Serialize;
use serde_wasm_bindgen::to_value;
use web_sys::{console, window, HtmlInputElement};
use yew::prelude::*;

// A struct that holds properties for a transaction,
// used in HTTP requests for creating a new transaction.
#[derive(Serialize)]
pub struct TransactionData {
    amount: f64,
    description: String,
    budget_group_id: i32,
}

// A struct that holds the group ID for which a transaction will be added, 
// used as properties for the AddTransactionForm component.
#[derive(Clone, PartialEq, Properties)]
pub struct AddTransactionFormProps {
    pub budget_group_id: i32,
}

// A functional component that creates a form for adding a new transaction.
// The form handles user input, data changes, form submission, and form reset.
#[function_component(AddTransactionForm)]
pub fn add_transaction_form(
    AddTransactionFormProps { budget_group_id }: &AddTransactionFormProps,
) -> Html {
    let input_amount_ref = use_node_ref();
    let input_description_ref = use_node_ref();
    let budget_group_id = *budget_group_id;

    let form_state = use_state(|| TransactionData {
        amount: 0.0,
        description: String::from(""),
        budget_group_id
    });

    let on_description_change = {
        let form_state = form_state.clone();
        let input_description_ref = input_description_ref.clone();
        Callback::from(move |_| {
            if let Some(input) = input_description_ref.cast::<HtmlInputElement>() {
                let value = input.value();
                form_state.set(TransactionData {
                    amount: form_state.amount,
                    description: value,
                    budget_group_id: form_state.budget_group_id,
                })
            }
        })
    };

    let on_amount_change = {
        let form_state = form_state.clone();
        let input_amount_ref = input_amount_ref.clone();
        Callback::from(move |_| {
            if let Some(input) = input_amount_ref.cast::<HtmlInputElement>() {
                let value = input.value();
                if let Ok(parsed_value) = value.parse::<f64>() {
                    form_state.set(TransactionData {
                        amount: parsed_value,
                        description: form_state.description.clone(),
                        budget_group_id: form_state.budget_group_id,
                    });
                }
            }
        })
    };

    let onsubmit = {
        let form_state = form_state;
        Callback::from(move |event: yew::events::SubmitEvent| {
            event.prevent_default();
            let form_state = form_state.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let data = TransactionData {
                    amount: form_state.amount,
                    description: form_state.description.clone(),
                    budget_group_id: form_state.budget_group_id,
                };

                let resp = Request::post("http://localhost:5001/expense/group")
                    .json(&data)
                    .unwrap()
                    .send()
                    .await
                    .unwrap();

                if resp.status() != 200 {
                    let jsvalueresp = to_value(&resp.status()).unwrap();
                    console::log_1(&jsvalueresp);
                };

                let id = budget_group_id;
                let url = format!("/expense/group/{}", id);

                let window = window().expect("error getting window");
                let location = window.location();
                let _ = location.set_href(&url);
            });
        })
    };

    html! {
        <form class="mx-auto" style="width: 250px;" onsubmit={onsubmit}>
        <div class="mb-3 text-end">
            <label for="description" class="form-label">{ "Description" }</label>
            <input type="text" class="form-control text-end" id="description" aria-describedby="descriptionHelp" ref={input_description_ref} onchange={on_description_change}/>
            <div id="descriptionHelp" class="form-text">{ "coffee / pizza / ..." }</div>
        </div>
        <div class="mb-3 text-end">
            <label for="amount" class="form-label">{ "Amount" }</label>
            <input type="text" class="form-control text-end" id="amount" placeholder="Number up to 2 decimal places" pattern="[0-9]*[.]?[0-9]{0,2}" ref={input_amount_ref} onchange={on_amount_change}/>
        </div>
        <div class="d-flex justify-content-end">
            <button type="reset" class="btn btn-secondary me-1">{ "Reset" }</button>
            <button type="submit" class="btn btn-primary ms-1">{ "Submit" }</button>
        </div>
    </form>
    }
}
