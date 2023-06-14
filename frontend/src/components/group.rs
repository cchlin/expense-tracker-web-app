use gloo_net::http::Request;
use yew::prelude::*;
use web_sys::{console, window};
use serde_wasm_bindgen::to_value;
use serde::{Serialize, Deserialize};


#[function_component(GroupButtons)]
fn group_buttons(GroupIdProps { id }: &GroupIdProps) -> Html {
    let id = id.clone();
    let on_delete_group_click = {
        Callback::from(move |_| {
            let id = id.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let url = format!("http://localhost:5001/expense/group/{}", id);
                let resp = Request::delete(&url)
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
            })
        })
    };

    let on_add_button = {
        let id = id.clone();
        Callback::from(move |_| {
            let url = format!("/expense/group/{}/add_transaction", id);
            let window = window().expect("Error getting window");
            let location = window.location();
            let _ = location.set_href(&url);
        })
    };

    html! {
        <>
            <div class="container sticky-bottom d-flex justify-content-center" style="padding-bottom: 80px;">
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

#[derive(Properties, PartialEq, Clone)]
pub struct GroupIdProps {
    pub id: i32,
}

#[derive(Clone, Serialize, Deserialize)]
struct Transaction {
    id: i32,
    amount: f64,
    description: String,
    date: String,
    budget_group_id: i32,
}

#[derive(Properties, PartialEq, Clone)]
struct TransactionProps {
    transaction: Transaction,
}

#[function_component(TransactionCard)]
fn transaction_card(TransactionProps { transaction }: &TransactionProps) -> Html {
    
}

#[derive(Properties, PartialEq, Clone)]
struct TransactionsProps {
    transactions: Vec<Transaction>,
}

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


#[function_component(GroupTransaction)]
pub fn show_group_transactions(GroupIdProps { id }: &GroupIdProps) -> Html {
    let transactions = use_state(|| vec![]);
    let id = id.clone();
    {
        let transactions = transactions.clone();
        let id = id.clone();
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
            }, ());
    }

    html! {
        <>
            <div class="mx-auto" style="max-width: 400px;">
                <TransactionList transactions={(*transactions).clone()} />
                <GroupButtons id={id}/>
            </div>
        </>
    }
}