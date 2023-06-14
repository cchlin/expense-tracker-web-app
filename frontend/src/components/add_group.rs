use yew::prelude::*;
use yew_router::prelude::*;
use web_sys::HtmlInputElement;
use serde::{Serialize, Deserialize};
use gloo_net::http::Request;
use web_sys::console;
use serde_wasm_bindgen::to_value;


// #[derive(Clone, Routable, PartialEq)]
// pub enum ExpenseRoute {
//     #[at("/expense/add_group")]
//     AddGroupForm,
// }

// pub fn expense_setting(route: ExpenseRoute) -> Html {
//     match route {
//         ExpenseRoute::AddGroupForm => html! {
//             <AddGroupForm />
//         },
//     }
// }

#[derive(Clone, PartialEq, Properties, Serialize, Deserialize)]
struct FormData {
    name: String,
    budget_amount: f64,
}

#[function_component(AddGroupForm)]
pub fn add_group_form() -> Html {
    let input_name_ref = use_node_ref();
    let input_number_ref = use_node_ref();
    let navigator = use_navigator().unwrap();

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
                    name: value.clone(),
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
                            budget_amount: parsed_value.clone(),
                        });
                }
            }
        })
    };

    let onsubmit = {
        let navigator = navigator.clone();
        let form_state = form_state.clone();
        Callback::from(move |event: yew::events::SubmitEvent| {
            event.prevent_default();
            let form_state = form_state.clone();
            wasm_bindgen_futures::spawn_local(async move {
                // let form_state = form_state.clone();
                let data = FormData {
                    name: form_state.name.clone(),
                    budget_amount: form_state.budget_amount.clone()
                };
                
                let resp = Request::post("http://localhost:5001/expense/add-group")
                    .json(&data)
                    .unwrap()
                    .send()
                    .await
                    .unwrap();

                // if not seccuess, output the status code
                // if resp.status() == 200 {
                // }
                if resp.status() != 200 {
                    let jsvalueresp = to_value(&resp.status()).unwrap();
                    console::log_1(&jsvalueresp);
                }

            });
            navigator.replace(&super::super::Route::Expense);
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
        let navigator = navigator.clone();
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
