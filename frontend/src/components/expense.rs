use yew::prelude::*;
use gloo_net::http::Request;
use serde::{Serialize, Deserialize};
use web_sys::console;
use serde_wasm_bindgen::to_value;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Group {
    pub id: i32,
    pub name: String,
    pub budget_amount: f64,
    pub remaining_budget: f64,
}


#[derive(Properties, PartialEq, Clone)]
pub struct GroupProps {
    pub group: Group,
}

#[function_component(GroupCard)]
pub fn group_card(GroupProps { group }: &GroupProps) -> Html {
    html! {
        <div class={classes!("card", "text-end", "my-5")}>
            <p>{ format!("{}", group.id) }</p>
            <p>{ format!("{}", group.name) }</p>
            <p>{ format!("{}", group.budget_amount) }</p>
            <p>{ format!("{}", group.remaining_budget) }</p>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct GroupsProps {
    groups: Vec<Group>,
}

#[function_component(GroupsList)]
pub fn groups_list(GroupsProps { groups }: &GroupsProps) -> Html {
    groups.iter().map(|group| html! {
        <GroupCard group={group.clone()} />
    }).collect::<Html>()
}

#[function_component(Expense)]
pub fn expense() -> Html {

    let groups = use_state(|| vec![]);
    {
        let groups = groups.clone();
        use_effect_with_deps(move |_| {
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
        }, ());
    }


    html! {
        <>
            <h1 class="text-center">{ "Expense Page in expense.rs" }</h1>
            <GroupsList groups={(*groups).clone()} />
        </>
    }
}
