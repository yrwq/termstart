use gloo::dialogs::alert;
use yew::prelude::*;
mod components;
use components::button::Button;

#[function_component(App)]
fn app() -> Html {
    let onclick = Callback::from(|_| {
    });

    html! {
        <main class="m-10">
                <Button text="click" {onclick} />
        </main>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
