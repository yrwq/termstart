use yew::prelude::*;
mod components;
use components::theme_switcher::ThemeSwitcher;
use components::terminal::Terminal;

#[function_component(App)]
fn app() -> Html {
    html! {
        <main class="min-h-screen bg-github-light-bg dark:bg-github-dark-bg transition-colors duration-200 ease-in-out p-4">
            <div class="flex justify-end mb-4">
                <ThemeSwitcher />
            </div>
            <Terminal />
        </main>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
