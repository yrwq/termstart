use yew::prelude::*;
use log::{info, Level};

mod components;
use components::terminal::Terminal;

#[function_component(App)]
fn app() -> Html {
    html! {
        <main class="min-h-screen bg-github-light-bg dark:bg-github-dark-bg transition-colors duration-200 ease-in-out flex flex-col">
            <div class="flex mt-40 justify-center items-center">
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