mod components; // Import the components module

use components::about::About; // Import the About component from the components module
use components::expense::{expense_setting, Expense, ExpenseRoute}; // Import the Expense and ExpenseRoute components from the expense module inside components module
use components::layout::Layout;  // Import the Layout component from the components module
use components::login::Login;  // Import the Login component from the components module
use yew::prelude::*;  // Import all entities from the prelude module of the Yew crate
use yew_router::prelude::*;  // Import all entities from the prelude module of the yew_router crate


// Define the routes for the application
#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]  // Route for the home page
    Home,
    #[at("/expense")]  // Route for the expense page
    Expense,
    #[at("/expense/*")]  // Route for all pages under the expense page
    ExpenseRoute,
    #[at("/about")]  // Route for the about page
    About,
    #[not_found]  // Route for the 404 not found page
    #[at("/404")]
    NotFound,
}

// Function to switch between different routes
fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Login /> },  // Home route leads to Login component
        Route::Expense => html! { <Expense /> },  // Expense route leads to Expense component
        Route::ExpenseRoute => html! { <Switch<ExpenseRoute> render={expense_setting} /> },  // ExpenseRoute leads to a function that returns different components based on the path
        Route::About => html! { <About /> },  // About route leads to About component
        Route::NotFound => html! { <h1>{ "404" }</h1> },  // NotFound route leads to a 404 message
    }
}

// Main component of the application
#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>  // Use browser router for routing
            <Layout>  // Layout of the application
                <Switch<Route> render={switch} />  // Switch between different routes based on the path
            </Layout>
        </BrowserRouter>
    }
}

// Main function for starting the Yew application
fn main() {
    yew::Renderer::<App>::new().render();
}
