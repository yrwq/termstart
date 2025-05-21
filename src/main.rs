use gloo::dialogs::alert;
use yew::prelude::*;
mod components;
use components::button::Button;
use components::theme_switcher::ThemeSwitcher;

#[function_component(App)]
fn app() -> Html {
    html! {
        <main class="m-10 transition-colors duration-200 ease-in-out">
            <ThemeSwitcher />
        </main>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
