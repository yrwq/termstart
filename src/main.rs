use yew::prelude::*;
use log::{info, Level};

mod components;
use components::theme_switcher::ThemeSwitcher;
use components::terminal::Terminal;

#[function_component(App)]
fn app() -> Html {
    html! {
        <main class="bg-github-light-bg dark:bg-github-dark-bg transition-colors duration-200 ease-in-out p-4">
            <div class="flex justify-end mb-4">
                <ThemeSwitcher />
            </div>
            <div class="flex justify-center items-center">
                <Terminal />
            </div>
        </main>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(Level::Debug));
    info!("Application starting...");
    yew::Renderer::<App>::new().render();
}