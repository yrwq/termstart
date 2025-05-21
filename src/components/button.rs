use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    pub text: String,
    pub onclick: Callback<MouseEvent>,
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let ButtonProps { text, onclick } = props;

    html! {
        <button
            onclick={onclick}
            class="px-4 py-2 rounded transition-colors bg-github-light-button dark:bg-github-dark-button text-github-light-text dark:text-github-dark-text hover:bg-github-light-button-hover dark:hover:bg-github-dark-button-hover"
        >
            { text }
        </button>
    }
}