pub mod commands;

use yew::prelude::*;
use web_sys::HtmlInputElement;

use crate::components::terminal::commands::handle_command;

#[derive(Default, Clone, PartialEq)]
pub struct TerminalHistory {
    pub commands: Vec<String>,
    pub outputs: Vec<String>,
}

#[function_component(Terminal)]
pub fn terminal() -> Html {
    let input_ref = use_node_ref();
    let history = use_state(TerminalHistory::default);

    // Focus effect when component mounts
    {
        let input_ref = input_ref.clone();
        use_effect(move || {
            if let Some(input) = input_ref.cast::<HtmlInputElement>() {
                input.focus().ok();
            }
            || ()
        });
    }

    // Callback to append output to terminal history
    let handle_output = {
        let history = history.clone();
        let input_ref = input_ref.clone();
        move |output: String| {
            let mut new_history = (*history).clone();
            new_history.commands.push(output);
            new_history.outputs.push("".to_string());
            history.set(new_history);
            
            // Focus input after command execution
            if let Some(input) = input_ref.cast::<HtmlInputElement>() {
                input.focus().ok();
            }
        }
    };

    // Handle command input
    let onkeydown = {
        let input_ref = input_ref.clone();
        let history = history.clone();
        
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
                            // Focus input after clear
                            input.focus().ok();
                        } else {
                            // Handle other commands
                            let output = handle_command(parts, handle_output.clone());
                            
                            // Update history with both command and output
                            let mut new_history = TerminalHistory {
                                commands: history.commands.clone(),
                                outputs: history.outputs.clone(),
                            };
                            new_history.commands.push(command);
                            new_history.outputs.push(output);
                            history.set(new_history);
                            
                            // Focus input after command
                            input.focus().ok();
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
        <div class="w-full max-w-3xl p-4 bg-github-light-button dark:bg-github-dark-button rounded-lg shadow-lg font-mono">
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
                                    if !output.is_empty() {
                                        <div class="text-github-light-text dark:text-github-dark-text ml-4 opacity-90 font-light">
                                            {output}
                                        </div>
                                    }
                                }
                            </div>
                        }
                    }).collect::<Html>()
                }
                <div ref={scroll_ref}></div>
            </div>
            <div class="flex items-center mt-2">
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
