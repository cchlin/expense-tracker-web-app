mod components;

use components::about::About;
use components::expense::{expense_setting, Expense, ExpenseRoute};
use components::layout::Layout;
use components::login::Login;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/expense")]
    Expense,
    #[at("/expense/*")]
    ExpenseRoute,
    #[at("/about")]
    About,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {
           <Login />
        },
        Route::Expense => html! {
            <Expense />
        },
        Route::ExpenseRoute => html! {
           <Switch<ExpenseRoute> render={expense_setting} />
        },
        Route::About => html! {
           <About />
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
