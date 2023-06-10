use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct LayoutProps {
    pub children: Children,
}

#[function_component(Layout)]
pub fn layout(LayoutProps { children }: &LayoutProps) -> Html {
    use yew::{classes, html};
    use super::navbar::Navbar;
    use super::footer::Footer;

    html! {
        <>
            <Navbar />
            <div class={classes!("mx-auto", "w-75", "my-5", "text-center")}>{ children.clone() }</div>
            <Footer />
        </>
    }
}
