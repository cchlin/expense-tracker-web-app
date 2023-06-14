use yew::prelude::*;


#[derive(Properties, PartialEq, Clone)]
pub struct GroupIdProps {
    pub id: i32,
}

#[function_component(GroupTransaction)]
pub fn show_group_transactions(GroupIdProps { id }: &GroupIdProps) -> Html {
    html! {
        <>
            <div class="mx-auto">
                <p>{ format!("Route for group id: {}", id) }</p>
            </div>
        </>
    }
}