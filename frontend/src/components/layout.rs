use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct LayoutProps {
    pub children: Children,
}

#[function_component(Layout)]
pub fn layout(LayoutProps { children }: &LayoutProps) -> Html {
    use super::footer::Footer;
    use super::navbar::Navbar;
    use yew::html;

    html! {
        <>
            <Navbar />
            <div class="mx-auto w-50 my-5" style="min-width: 350px;">{ children.clone() }</div>
            <Footer />
        </>
    }
}
