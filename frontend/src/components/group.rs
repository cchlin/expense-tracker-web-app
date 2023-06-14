use gloo_net::http::Request;
use yew::prelude::*;
use web_sys::{console, window};
use serde_wasm_bindgen::to_value;


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

    html! {
        <>
            <div class="container sticky-bottom" style="padding-bottom: 80px;">
                <div class="row">
                    <div clas="col">
                        <button type="button" class="btn btn-outline-primary me-1">{"Add transaction"}</button>
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

#[function_component(GroupTransaction)]
pub fn show_group_transactions(GroupIdProps { id }: &GroupIdProps) -> Html {
    html! {
        <>
            <div class="mx-auto" style="width: 400px;">
                <p>{ format!("Route for group id: {}", id) }</p>
                <GroupButtons id={id}/>
            </div>
        </>
    }
}