pub mod commands;

use yew::prelude::*;
use web_sys::HtmlInputElement;
use std::collections::VecDeque;

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
    let command_history = use_state(VecDeque::<String>::new);
    let history_index = use_state(|| -1);
    let current_input = use_state(String::new);
    let is_dark = use_state(|| false);

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

    // Theme toggle effect
    use_effect_with(is_dark.clone(), move |is_dark| {
        let document = web_sys::window()
            .unwrap()
            .document()
            .unwrap();
        
        if **is_dark {
            document.document_element().unwrap().class_list().add_1("dark").unwrap();
        } else {
            document.document_element().unwrap().class_list().remove_1("dark").unwrap();
        }
    });

    // Callback to append output to terminal history
    let handle_output = {
        let history = history.clone();
        let input_ref = input_ref.clone();
        let command_history = command_history.clone();
        let history_index = history_index.clone();
        let current_input = current_input.clone();
        move |output: String| {
            let mut new_history = (*history).clone();
            new_history.commands.push(output.clone());
            new_history.outputs.push("".to_string());
            history.set(new_history);
            
            // Add command to history
            let mut new_command_history = (*command_history).clone();
            new_command_history.push_front(output);
            if new_command_history.len() > 100 {
                new_command_history.pop_back();
            }
            command_history.set(new_command_history);
            history_index.set(-1);
            current_input.set(String::new());
            
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
        let command_history = command_history.clone();
        let history_index = history_index.clone();
        let current_input = current_input.clone();
        let is_dark = is_dark.clone();
        
        Callback::from(move |e: KeyboardEvent| {
            if let Some(input) = input_ref.cast::<HtmlInputElement>() {
                match e.key().as_str() {
                    "Enter" => {
                        e.prevent_default();
                        let command = input.value().trim().to_string();
                        
                        // Clear input immediately
                        input.set_value("");
                        current_input.set(String::new());
                        
                        if !command.is_empty() {
                            // Process command first to get output
                            let parts: Vec<&str> = command.split_whitespace().collect();
                            let cmd = parts.get(0).map(|s| *s).unwrap_or("");
                            
                            if cmd == "clear" {
                                // Handle clear command directly
                                history.set(TerminalHistory::default());
                                // Focus input after clear
                                input.focus().ok();
                            } else if cmd == "theme" {
                                // Handle theme toggle
                                is_dark.set(!*is_dark);
                                let output = if *is_dark { "Switched to dark theme" } else { "Switched to light theme" }.to_string();
                                
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
                    },
                    "ArrowUp" => {
                        e.prevent_default();
                        let mut new_index = *history_index;
                        if new_index < (command_history.len() as i32 - 1) {
                            new_index += 1;
                            if let Some(cmd) = command_history.get(new_index as usize) {
                                input.set_value(cmd);
                                current_input.set(cmd.clone());
                            }
                        }
                        history_index.set(new_index);
                    },
                    "ArrowDown" => {
                        e.prevent_default();
                        let mut new_index = *history_index;
                        if new_index > 0 {
                            new_index -= 1;
                            if let Some(cmd) = command_history.get(new_index as usize) {
                                input.set_value(cmd);
                                current_input.set(cmd.clone());
                            } else {
                                input.set_value("");
                                current_input.set(String::new());
                            }
                        } else if new_index == 0 {
                            input.set_value("");
                            current_input.set(String::new());
                            new_index = -1;
                        }
                        history_index.set(new_index);
                    },
                    _ => {}
                }
            }
        })
    };

    // Handle input changes
    let oninput = {
        let current_input = current_input.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                current_input.set(input.value());
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
        <div class="w-full h-screen flex items-center justify-center">
            <div class="w-full max-w-5xl p-4 bg-github-light-bg dark:bg-github-dark-bg rounded-lg shadow-xl font-mono text-github-light-text dark:text-github-dark-text border border-github-light-border dark:border-github-dark-border">
                <div class="flex items-center mb-2 px-2">
                    <div class="flex space-x-2">
                        <div class="w-3 h-3 rounded-full bg-red-500"></div>
                        <div class="w-3 h-3 rounded-full bg-yellow-500"></div>
                        <div class="w-3 h-3 rounded-full bg-green-500"></div>
                    </div>
                    <div class="ml-4 text-sm text-github-light-text dark:text-github-dark-text opacity-70">{"termstart v0.1.0"}</div>
                </div>
                <div class="overflow-y-auto h-[600px] whitespace-pre-wrap bg-github-light-bg dark:bg-github-dark-bg rounded p-4">
                    <div class="mb-4 text-github-light-text dark:text-github-dark-text opacity-70">
                        {"Welcome to termstart v0.1.0\nType 'help' for available commands.\n"}
                    </div>
                    {
                        history.commands.iter().enumerate().map(|(i, cmd)| {
                            html! {
                                <div key={i} class="mb-2">
                                    <div class="flex items-start group">
                                        <span class="text-github-light-text dark:text-github-dark-text mr-2 select-none opacity-80 terminal-prompt">{"$"}</span>
                                        <span class="text-github-light-text dark:text-github-dark-text">{cmd}</span>
                                    </div>
                                    if let Some(output) = history.outputs.get(i) {
                                        if !output.is_empty() {
                                            <div class="ml-4 opacity-90 font-light text-github-light-text dark:text-github-dark-text terminal-output">
                                                {output}
                                            </div>
                                        }
                                    }
                                </div>
                            }
                        }).collect::<Html>()
                    }
                    <div class="flex items-start group">
                        <span class="text-github-light-text dark:text-github-dark-text mr-2 select-none opacity-80 terminal-prompt">{"$"}</span>
                        <input
                            type="text"
                            ref={input_ref}
                            {onkeydown}
                            {oninput}
                            autofocus=true
                            class="bg-transparent outline-none border-none text-github-light-text dark:text-github-dark-text w-full terminal-input"
                            placeholder=" "
                            spellcheck="false"
                            autocomplete="off"
                            value={(*current_input).clone()}
                        />
                    </div>
                    <div ref={scroll_ref}></div>
                </div>
            </div>
        </div>
    }
}
