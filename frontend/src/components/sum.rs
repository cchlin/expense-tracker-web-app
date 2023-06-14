use super::super::components::expense::Group;
use gloo_net::http::Request;
use yew::function_component;
use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct SumProps {
    pub group_id: i32,
}

#[function_component(Sum)]
pub fn sum(SumProps { group_id }: &SumProps) -> Html {
    let group = use_state(|| Group {
        id: 0,
        name: String::from(""),
        budget_amount: 0.0,
        remaining_budget: 0.0,
    });
    let group_id = *group_id;
    {
        let group = group.clone();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let url = format!("http://localhost:5001/expense/{}", group_id);
                    let fetched_group: Group = Request::get(&url)
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap();

                    group.set(Group {
                        id: fetched_group.id,
                        name: fetched_group.name,
                        budget_amount: fetched_group.budget_amount,
                        remaining_budget: fetched_group.remaining_budget,
                    })
                });
            },
            (),
        );
    }

    html! {
        <div class="mx-auto mb-3" style="max-width: 400px;">
            <div class="container">
                <div class="row">
                    <div class="col">{ "Budget" }</div>
                    <div class="col text-end">{ format!("$ {}",group.budget_amount) }</div>
                </div>
                <div class="row">
                    <div class="col">{ "Total expenses" }</div>
                    <div class="col text-end">{ format!("- {:.2}", group.budget_amount - group.remaining_budget) }</div>
                </div>
                <hr />
                <div class="row">
                    <div class="col">{ "Remaining" }</div>
                    <div class="col text-end">{ format!("$ {:.2}", group.remaining_budget) }</div>
                </div>
            </div>
        </div>
    }
}
