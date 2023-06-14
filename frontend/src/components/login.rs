use web_sys::window;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
struct FormInputProps {
    label: String,
    name: String,
    input_type: String,
}

#[function_component(FormInput)]
fn form_input(
    FormInputProps {
        label,
        name,
        input_type,
    }: &FormInputProps,
) -> Html {
    html! {
        <div class="rol d-flex">
            <label class="col m-1" for={name.clone()}>{ label.clone() }</label>
            <input class="col justify-content-end m-1" id={name.clone()} type={input_type.clone()} />
        </div>
    }
}

#[derive(Properties, Clone, PartialEq)]
struct FormButtonProps {
    class: String,
    button_type: String,
    children: Children,
}

#[function_component(FormButton)]
fn form_button(
    FormButtonProps {
        class,
        button_type,
        children,
    }: &FormButtonProps,
) -> Html {
    html! {
        <button class={class.clone()} type={button_type.clone()}>{ children.clone() }</button>
    }
}

#[function_component(Login)]
pub fn login() -> Html {
    let handle_submit = {
        Callback::from(move |event: yew::events::SubmitEvent| {
            event.prevent_default();
            let window = window().expect("error getting window");
            let location = window.location();
            let _ = location.set_href("/expense");
        })
    };

    html! {
        <div class="mx-auto" style="max-width: 350px;">
            <h1 class="title text-center">{ "Sign In" }</h1>
            <form onsubmit={handle_submit}>
                <FormInput label="Email" name="email" input_type="text" />
                <FormInput label="Password" name="password" input_type="password" />
                <div class="container">
                    <div class="row">
                        <div class="col">
                            <FormButton class="btn btn-primary" button_type="submit">{ "Sign In" }</FormButton>
                        </div>
                        <div class="col">
                                <a class="row justify-content-end" style="font-size: 12px;" href="/">{ "Sign Up" }</a>
                                <a class="row justify-content-end" style="font-size: 12px;" href="/">{ "Forgot Password?" }</a>
                        </div>
                    </div>
                </div>
            </form>
        </div>
    }
}
