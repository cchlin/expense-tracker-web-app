mod components;

use yew::prelude::*;
use yew_router::prelude::*;
use components::navbar::Navbar;
use components::layout::Layout;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/expense")]
    Expense,
    #[at("/about")]
    About,
    #[at("/secure")]
    Secure,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(Secure)]
fn secure() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&Route::Home));
    html! {
        <div>
            <h1>{ "Secure" }</h1>
            <button {onclick}>{ "Go Home" }</button>
        </div>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <h1>{ "Home" }</h1> },
        Route::Expense => html! {
            <h1>{ "Expense Page" }</h1>
        },
        Route::About => html! { <h1>{ "About Page" }</h1> },
        Route::Secure => html! {
            <Secure />
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Layout>
                <Switch<Route> render={switch} />
            </Layout>
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}