use super::super::components::sum::Sum;
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_wasm_bindgen::to_value;
use web_sys::{console, window};
use yew::prelude::*;

// A functional component that creates two buttons: one for adding
// a new transaction to a group and one for deleting the group.
#[function_component(GroupButtons)]
fn group_buttons(GroupIdProps { id }: &GroupIdProps) -> Html {
    let id = *id;
    let on_delete_group_click = {
        Callback::from(move |_| {
            let id = id;
            wasm_bindgen_futures::spawn_local(async move {
                let url = format!("http://localhost:5001/expense/group/{}", id);
                let resp = Request::delete(&url).send().await.unwrap();

                if resp.status() != 200 {
                    let jsvalueresp = to_value(&resp.status()).unwrap();
                    console::log_1(&jsvalueresp);
                }
                let window = window().expect("error getting window");
                let location = window.location();
                let _ = location.set_href("/expense");
            })
        })
    };

    let on_add_button = {
        let id = id;
        Callback::from(move |_| {
            let url = format!("/expense/group/{}/add_transaction", id);
            let window = window().expect("Error getting window");
            let location = window.location();
            let _ = location.set_href(&url);
        })
    };

    html! {
        <>
            <div class="container sticky-bottom d-flex justify-content-center mt-3" style="padding-bottom: 80px;">
                <div class="row">
                    <div clas="col">
                        <button type="button" class="btn btn-outline-primary me-1" onclick={on_add_button}>{"Add transaction"}</button>
                        <button type="button" class="btn btn-danger ms-1" onclick={on_delete_group_click}>{"Delete group"}</button>
                    </div>
                </div>
            </div>
        </>
    }
}

// A struct holding the properties for the group ID,
// which is used as props for components that need to handle the group data.
#[derive(Properties, PartialEq, Clone)]
pub struct GroupIdProps {
    pub id: i32,
}

// A struct to hold properties for a transaction.
#[derive(Clone, Serialize, Deserialize, PartialEq)]
struct Transaction {
    id: i32,
    amount: f64,
    description: String,
    date: String,
    budget_group_id: i32,
}

// A struct to hold transaction properties that can be passed into Yew components.
#[derive(Properties, PartialEq, Clone)]
struct TransactionProps {
    transaction: Transaction,
}

// A functional component that creates a card displaying a
// single transaction's details.
#[function_component(TransactionCard)]
fn transaction_card(TransactionProps { transaction }: &TransactionProps) -> Html {
    let transaction = transaction.clone();
    let on_delete_click = {
        let id = transaction.budget_group_id;
        Callback::from(move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let data = json!({
                    "id": transaction.id.clone(),
                    "amount": transaction.amount.clone(),
                    "description": "",
                    "date": "",
                    "budget_group_id": transaction.budget_group_id.clone(),
                });
                let resp = Request::delete("http://localhost:5001/expense/transaction")
                    .json(&data)
                    .unwrap()
                    .send()
                    .await
                    .unwrap();

                if resp.status() != 200 {
                    let jsvalueresp = to_value(&resp.status()).unwrap();
                    console::log_1(&jsvalueresp);
                }
                let group_u = format!("/expense/group/{}", id);
                let window = window().expect("error getting window");
                let location = window.location();
                let _ = location.set_href(&group_u);
            });
        })
    };

    html! {
        <div class="card my-1 mx-auto" style="max-width: 400px;">
            <div>
                <div class="container text-bg-light">
                    <div class="row p-2">
                        <div class="col">
                            <div class="row" style="font-size: 12px;">{ transaction.description.clone() }</div>
                            <div class="row text-secondary" style="font-size: 10px;">{ transaction.date.clone() }</div>
                        </div>
                        <div class="col-auto text-end">
                            <div class="row text-end justify-content-end text-danger" style="font-size: 12px;">{ format!("- $ {}", transaction.amount) }</div>
                                <a class="row text-end justify-content-end link-primary link-offset-3 link-underline-opacity-0 link-underline-opacity-100-hover" style="font-size: 12px;" onclick={on_delete_click}>{ "Delete" }</a>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

// A struct to hold a list of transactions,
// used as a property for the TransactionList component.
#[derive(Properties, PartialEq, Clone)]
struct TransactionsProps {
    transactions: Vec<Transaction>,
}

// A functional component that takes a list of
// transactions and maps each to a TransactionCard component.
#[function_component(TransactionList)]
fn transaction_list(TransactionsProps { transactions }: &TransactionsProps) -> Html {
    transactions
        .iter()
        .map(|transaction| {
            html! {
                <TransactionCard transaction={transaction.clone()} />
            }
        })
        .collect::<Html>()
}

// A functional component that fetches agroup's transactions,
// sums up the total amount spent,and renders the list of transactions and 
// the total amount.
#[function_component(GroupTransaction)]
pub fn show_group_transactions(GroupIdProps { id }: &GroupIdProps) -> Html {
    let transactions = use_state(Vec::new);
    let id = *id;
    {
        let transactions = transactions.clone();
        let id = id;
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let url = format!("http://localhost:5001/expense/group/{}", id);
                    let fetched_transactions: Vec<Transaction> = Request::get(&url)
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap();

                    let js_value = to_value(&fetched_transactions).unwrap();
                    console::log_1(&js_value);

                    transactions.set(fetched_transactions);
                })
            },
            (),
        );
    }

    html! {
        <>
            <div class="mx-auto" style="max-width: 400px;">
                <Sum group_id={id}/>
                <TransactionList transactions={(*transactions).clone()} />
                <GroupButtons id={id}/>
            </div>
        </>
    }
}
