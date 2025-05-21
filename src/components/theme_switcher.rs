use yew::prelude::*;
use gloo::utils::document_element;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use lucide_yew::{Sun, Moon};

#[function_component(ThemeSwitcher)]
pub fn theme_switcher() -> Html {
    let theme_state = use_state(|| {
        let window = web_sys::window().unwrap();
        let media_query = window
            .match_media("(prefers-color-scheme: dark)")
            .unwrap()
            .unwrap();
        
        let doc = document_element();
        if media_query.matches() {
            doc.class_list().add_1("dark").unwrap();
            true
        } else {
            doc.class_list().remove_1("dark").unwrap();
            false
        }
    });
    
    {
        let theme_state = theme_state.clone();
        use_effect(move || {
            let window = web_sys::window().unwrap();
            let media_query = window
                .match_media("(prefers-color-scheme: dark)")
                .unwrap()
                .unwrap();
            
            let theme_state = theme_state.clone();
            let callback = Closure::wrap(Box::new(move |e: web_sys::Event| {
                let media_query = e.target().unwrap().unchecked_into::<web_sys::MediaQueryList>();
                let doc = document_element();
                if media_query.matches() {
                    doc.class_list().add_1("dark").unwrap();
                    theme_state.set(true);
                } else {
                    doc.class_list().remove_1("dark").unwrap();
                    theme_state.set(false);
                }
            }) as Box<dyn FnMut(_)>);
            
            media_query.add_event_listener_with_callback("change", callback.as_ref().unchecked_ref()).unwrap();
            callback.forget();
            
            || {}
        });
    }

    let onclick = {
        let theme_state = theme_state.clone();
        Callback::from(move |_| {
            let doc = document_element();
            if *theme_state {
                doc.class_list().remove_1("dark").unwrap();
            } else {
                doc.class_list().add_1("dark").unwrap();
            }
            theme_state.set(!*theme_state);
        })
    };

    html! {
        <button
            onclick={onclick}
            class="p-2 rounded-full transition-all duration-200 ease-in-out bg-github-light-button dark:bg-github-dark-button text-github-light-text dark:text-github-dark-text hover:bg-github-light-button-hover dark:hover:bg-github-dark-button-hover"
            title={if *theme_state { "Switch to light mode" } else { "Switch to dark mode" }}
        >
            if *theme_state {
                <Sun class="w-5 h-5" />
            } else {
                <Moon class="w-5 h-5" />
            }
        </button>
    }
}