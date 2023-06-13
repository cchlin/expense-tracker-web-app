use super::add_group::AddGroup;
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use web_sys::console;
use yew::prelude::*;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
struct Group {
    id: i32,
    name: String,
    budget_amount: f64,
    remaining_budget: f64,
}

impl Group {
    pub fn fields(&self) -> Vec<String> {
        vec![
            format!("{}", self.name),
            format!("{}", self.budget_amount),
            format!("{}", self.remaining_budget),
        ]
    }
}

#[derive(Properties, PartialEq, Clone)]
struct GroupProps {
    group: Group,
}

#[function_component(GroupCard)]
fn group_card(GroupProps { group }: &GroupProps) -> Html {
    let field = group.fields();
    html! {
        <div class={classes!("card", "my-4")}>
            { for field.iter().map(|field| html! { <p class="text-end">{ field }</p> }) }
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
    let groups = use_state(|| vec![]);
    {
        let groups = groups.clone();
        use_effect_with_deps(
            move |_| {
                let groups = groups.clone();
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
