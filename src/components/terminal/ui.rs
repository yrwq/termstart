use yew::prelude::*;
use web_sys::HtmlInputElement;
use gloo::storage::{LocalStorage, Storage};

use crate::components::terminal::history::TerminalHistory;
use crate::components::terminal::user::{User, STORAGE_KEY};
use crate::components::terminal::commands::handle_command;

#[function_component(Terminal)]
pub fn terminal() -> Html {
    let input_ref = use_node_ref();
    let history = use_state(TerminalHistory::default);
    let user: UseStateHandle<User> = use_state(|| {
        LocalStorage::get(STORAGE_KEY).unwrap_or_default()
    });

    // Callback to append output to terminal history
    let handle_output = {
        let history = history.clone();
        move |output: String| {
            let mut new_history = (*history).clone();
            new_history.commands.push("(async result)".to_string());
            new_history.outputs.push(output);
            history.set(new_history);
        }
    };

    // Handle command input
    let onkeydown = {
        let input_ref = input_ref.clone();
        let history = history.clone();
        let user = user.clone();
        
        Callback::from(move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                e.prevent_default();
                
                if let Some(input) = input_ref.cast::<HtmlInputElement>() {
                    let command = input.value().trim().to_string();
                    
                    if !command.is_empty() {
                        // Clear input immediately
                        input.set_value("");
                        
                        // Process command first to get output
                        let parts: Vec<&str> = command.split_whitespace().collect();
                        let cmd = parts.get(0).map(|s| *s).unwrap_or("");
                        
                        if cmd == "clear" {
                            // Handle clear command directly
                            history.set(TerminalHistory::default());
                        } else {
                            // Handle other commands
                            let output = handle_command(parts, &user, handle_output.clone());
                            
                            // Update history with both command and output
                            let mut new_history = TerminalHistory {
                                commands: history.commands.clone(),
                                outputs: history.outputs.clone(),
                            };
                            new_history.commands.push(command);
                            new_history.outputs.push(output);
                            history.set(new_history);
                        }
                    }
                }
            }
        })
    };

    // Auto-scroll to bottom
    let scroll_ref = use_node_ref();
    {
        let history = history.clone();
        let scroll_ref = scroll_ref.clone();
        use_effect_with(
            history,
            move |_| {
                if let Some(node) = scroll_ref.cast::<web_sys::HtmlElement>() {
                    node.scroll_into_view();
                }
                || ()
            }
        );
    }

    html! {
        <div class="w-full max-w-3xl mt-8 p-4 bg-github-light-button dark:bg-github-dark-button rounded-lg shadow-lg font-mono">
            <div class="overflow-y-auto h-96 whitespace-pre-wrap">
                <div class="text-github-light-text dark:text-github-dark-text mb-4">
                    {"Welcome to termstart v0.1.0\nType 'help' for available commands.\n"}
                </div>
                {
                    history.commands.iter().enumerate().map(|(i, cmd)| {
                        html! {
                            <div key={i} class="mb-2">
                                <div class="flex items-start text-github-light-text dark:text-github-dark-text">
                                    <span class="text-green-500 mr-2 select-none">{"::"}</span>
                                    <span class="font-bold">{cmd}</span>
                                </div>
                                if let Some(output) = history.outputs.get(i) {
                                    <div class="text-github-light-text dark:text-github-dark-text ml-4 opacity-90 font-light">
                                        {output}
                                    </div>
                                }
                            </div>
                        }
                    }).collect::<Html>()
                }
                <div ref={scroll_ref}></div>
            </div>
            <div class="flex items-center text-github-light-text dark:text-github-dark-text border-t border-github-light-border dark:border-github-dark-border pt-4">
                <span class="text-green-500 mr-2 select-none">{"::"}</span>
                <input
                    type="text"
                    ref={input_ref}
                    {onkeydown}
                    autofocus=true
                    class="flex-1 bg-transparent outline-none border-none text-github-light-text dark:text-github-dark-text"
                    placeholder=" "
                    spellcheck="false"
                    autocomplete="off"
                />
            </div>
        </div>
    }
}
