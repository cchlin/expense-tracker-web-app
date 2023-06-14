use super::add_group::{AddGroup, AddGroupForm};  // Import AddGroup and AddGroupForm components
use super::add_transaction::AddTransactionForm;  // Import AddTransactionForm component
use super::group::GroupTransaction;  // Import GroupTransaction component
use gloo_net::http::Request;  // Import Request from the gloo_net crate for making HTTP requests
use serde::{Deserialize, Serialize};  // Import Deserialize and Serialize from the serde crate for (de)serialization of data
use serde_wasm_bindgen::to_value;  // Import to_value function for converting data into JsValue
use web_sys::console;  // Import console for logging to the console
use yew::prelude::*;  // Import all entities from the prelude module of the Yew crate
use yew_router::Routable;  // Import Routable from the yew_router crate for defining routes

// Define the routes for the Expense component
#[derive(Clone, Routable, PartialEq)]
pub enum ExpenseRoute {
    #[at("/expense/group/:id")]  // Route for a specific group
    Group { id: i32 },
    #[at("/expense/add_group")]  // Route for adding a new group
    AddGroupForm,
    #[at("/expense/group/:id/add_transaction")]  // Route for adding a new transaction to a group
    AddTransactionForm { id: i32 },
}

// Function to switch between different routes of the Expense component
pub fn expense_setting(route: ExpenseRoute) -> Html {
    match route {
        ExpenseRoute::AddGroupForm => html! { <AddGroupForm /> },  // AddGroupForm route leads to AddGroupForm component
        ExpenseRoute::Group { id } => html! { <GroupTransaction id={id} /> },  // Group route leads to GroupTransaction component
        ExpenseRoute::AddTransactionForm { id } => html! { <AddTransactionForm budget_group_id={id} /> },  // AddTransactionForm route leads to AddTransactionForm component
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Group {
    pub id: i32,
    pub name: String,
    pub budget_amount: f64,
    pub remaining_budget: f64,
}

// impl Group {
//     pub fn fields(&self) -> Vec<String> {
//         vec![
//             format!("{}", self.name),
//             format!("{}", self.budget_amount),
//             format!("{}", self.remaining_budget),
//         ]
//     }
// }

#[derive(Properties, PartialEq, Clone)]
struct GroupProps {
    group: Group,
}

#[function_component(GroupCard)]
fn group_card(GroupProps { group }: &GroupProps) -> Html {
    // let field = group.fields();

    let url = format!("/expense/group/{}", group.id);
    let spent = group.budget_amount - group.remaining_budget;

    html! {
            <div class="card my-4 mx-auto text-bg-light groupcard" style="max-width: 400px;">
                <div class="card-body">
                    <div class="container">
                        <div class="row fs-5">
                            // <a href={url}>
                                <div class="col">{ group.name.to_string() }</div>
                                <div class="col text-end">{ format!("$ {}", group.remaining_budget) }</div>
                            // </a>
                        </div>
                       <div class="row">
                           <div class="col fst-lighter" style="font-size: 13px;">
                                <span>{ format!("${}", group.budget_amount) }</span>
                                <span class="text-danger">{ format!("  - ${:.5}", spent.to_string()) }</span>
                           </div>
                           <div class="col text-end">
                            <a href={url} class="link-primary link-offset-3 link-underline-opacity-0 link-underline-opacity-100-hover">{ "Check" }</a>
                           </div>
                       </div>
                    </div>
                    // { for field.iter().map(|field| html! {
                    //     <p class="text-end">{ field }</p>
                    // }) }
                </div>
            </div>
    }
}

#[derive(Properties, PartialEq)]
struct GroupsProps {
    groups: Vec<Group>,
}

#[function_component(GroupsList)]
fn groups_list(GroupsProps { groups }: &GroupsProps) -> Html {
    groups
        .iter()
        .map(|group| {
            html! {
                <GroupCard group={group.clone()} />
            }
        })
        .collect::<Html>()
}

#[function_component(Expense)]
pub fn expense() -> Html {
    let groups = use_state(Vec::new);
    {
        let groups = groups.clone();
        use_effect_with_deps(
            move |_| {
                // let groups = groups.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_groups: Vec<Group> = Request::get("http://localhost:5001/expense")
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap();

                    let js_value = to_value(&fetched_groups).unwrap();
                    console::log_1(&js_value);

                    groups.set(fetched_groups);
                });
                || ()
            },
            (),
        );
    }

    html! {
        <>
            <h1 class="text-center">{ "Budget" }</h1>
            <GroupsList groups={(*groups).clone()} />
            <AddGroup />
        </>
    }
}
