use yew::prelude::*;

#[function_component(Navbar)]
pub fn navbar() -> Html {
    use yew::{classes, html};

    html! {
            <nav class={classes!("navbar", "navbar-expand-sm", "bg-body-tertiary")}>
                <div class={classes!("container-fluid")}>
                    <a class={classes!("navbar-brand")} href="#">{ "Expense Tracker" }</a>
                    <button class={classes!("navbar-toggler")} type="button" data-bs-toggle="collapse" data-bs-target="#navbarSupportedContent" aria-controls="navbarSupportedContent" aria-expanded="false" aria-label="Toggle navigation">
                        <span class={classes!("navbar-toggler-icon")}></span>
                    </button>
                    <div class={classes!("collapse", "navbar-collapse")} id="navbarSupportedContent">
                        <ul class={classes!("navbar-nav", "ms-auto", "mb-2", "mb-lg-0")}>
                            <li class={classes!("nav-item")}>
                                <a class={classes!("nav-link")} aria-current="page" href="/">{ "Home" }</a>
                            </li>
                            <li class={classes!("nav-item")}>
                                <a class={classes!("nav-link")} href="/expense">{ "Expense" }</a>
                            </li>
                            <li class={classes!("nav-item")}>
                                <a class={classes!("nav-link")} href="/about">{ "About" }</a>
                            </li>
                        </ul>
                    </div>
                </div>
            </nav>
    }
}
