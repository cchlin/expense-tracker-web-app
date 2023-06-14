use yew::prelude::*;

#[function_component(About)]
pub fn about() -> Html {
    html! {
        <div class="mx-auto text-center" style="max-width: 400px;">
        <h1>{ "About Expense Tracker" }</h1>
        <p>{ "This application was created as a part of a school project for a Rust programming class. It is designed to help users keep track of their daily expenses. It provides functionalities to add, view, and delete transactions and budget groups. Hope it can be a useful tool for people  who wanting to manage money better in the future as the program grows." }</p>

        <h2>{ "Tools Used" }</h2>
        <p>{ "This application is built using Rust and leverages several libraries to create an efficient, user-friendly experience. `yew` powers the frontend interface, with `gloo-net` being used for fetch requests. On the backend, `actix-web` is used for the server. These technologies together make this application fast, secure, and reliable." }</p>
        </div>
    }
}
