use yew::prelude::*;
use log::{info, Level};

mod components;
use components::terminal::Terminal;

#[function_component(App)]
fn app() -> Html {
    html! {
        <main class="min-h-screen transition-colors duration-700 ease-in-out flex flex-col">
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