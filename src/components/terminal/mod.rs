pub mod commands;

use yew::prelude::*;
use web_sys::{HtmlInputElement, KeyboardEvent, InputEvent};
use lucide_yew::Folder;
use lucide_yew::File;

use crate::components::terminal::commands::handle_command;

#[derive(Default, Clone, PartialEq)]
pub struct TerminalHistory {
    pub lines: Vec<String>,
    pub outputs: Vec<String>,
}

fn render_output_line(line: &str) -> Html {
    if line.contains("TAG_ITEM:") {
        let parts: Vec<&str> = line.split("TAG_ITEM:").collect();
        if parts.len() > 1 {
            let prefix = parts[0].to_string();
            let tag_name = parts[1].trim_end().to_string(); // Trim newline
            html! {
                <div class="flex items-center">
                    <span>{ prefix }</span>
                    <Folder class="w-4 h-4 inline-block mr-1" />
                    <span>{ format!("{}/", tag_name) }</span>
                </div>
            }
        } else {
            // Fallback if TAG_ITEM: is at the very start or no content after
            let tag_name = line.strip_prefix("TAG_ITEM:").unwrap_or("").trim_end().to_string();
             html! {
                <div class="flex items-center">
                    <Folder class="w-4 h-4 inline-block mr-1" />
                    <span>{ format!("{}/", tag_name) }</span>
                </div>
            }
        }
    } else if line.contains("BOOKMARK_ITEM:") {
        let parts: Vec<&str> = line.split("BOOKMARK_ITEM:").collect();
        if parts.len() > 1 {
             let prefix = parts[0].to_string();
            let content = parts[1].trim_end().to_string(); // Trim newline
            let content_parts: Vec<&str> = content.splitn(2, ' ').collect();
            let name = content_parts.get(0).unwrap_or(&"").to_string();
            let tags = if content_parts.len() > 1 {
                content_parts[1].to_string()
            } else {
                String::new()
            };
            html! {
                <div class="flex items-center">
                    <span>{ prefix }</span>
                    <File class="w-4 h-4 inline-block mr-1" />
                    <span>{ format!("{}{}", name, tags) }</span>
                </div>
            }
        } else {
            // Fallback if BOOKMARK_ITEM: is at the very start or no content after
             let content = line.strip_prefix("BOOKMARK_ITEM:").unwrap_or("").trim_end().to_string();
            let content_parts: Vec<&str> = content.splitn(2, ' ').collect();
            let name = content_parts.get(0).unwrap_or(&"").to_string();
            let tags = if content_parts.len() > 1 {
                content_parts[1].to_string()
            } else {
                String::new()
            };
            html! {
                <div class="flex items-center">
                    <File class="w-4 h-4 inline-block mr-1" />
                    <span>{ format!("{}{}", name, tags) }</span>
                </div>
            }
        }
    } else if !line.is_empty() {
        html! { <div>{ line.to_string() }</div> }
    } else {
        html! { <div></div> }
    }
}

fn render_output(output: Option<&String>) -> Html {
    if let Some(output) = output {
        let rendered_lines = output.lines().map(|line| render_output_line(line)).collect::<Vec<Html>>();
        html! {
            <div class="ml-4 opacity-90 font-light text-github-light-text dark:text-github-dark-text terminal-output">
                {rendered_lines}
            </div>
        }
    } else {
        html! {
            <div class="ml-4 opacity-90 font-light text-github-light-text dark:text-github-dark-text terminal-output">
                { "" }
            </div>
        }
    }
}

#[function_component(Terminal)]
pub fn terminal() -> Html {
    let input_ref = use_node_ref();
    let completed_lines = use_state(|| Vec::<String>::new());
    let outputs = use_state(|| Vec::<String>::new());
    let current_line = use_state(|| "~ > ".to_string()); // Restore root prompt
    let history_nav_index = use_state(|| -1);
    let is_dark = use_state(|| false);
    let current_tag = use_state(|| None::<String>);

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

    // Callback to append output to history
    let handle_output = {
        let outputs = outputs.clone();
        move |output: String| {
            let mut current_outputs = (*outputs).clone();
            current_outputs.push(output);
            outputs.set(current_outputs);
        }
    };

    // Handle command input
    let onkeydown = {
        let input_ref = input_ref.clone();
        let completed_lines = completed_lines.clone();
        let outputs = outputs.clone();
        let current_line = current_line.clone();
        let history_nav_index = history_nav_index.clone();
        let is_dark = is_dark.clone();
        let current_tag = current_tag.clone();

        Callback::from(move |e: KeyboardEvent| {
            if let Some(input) = input_ref.cast::<HtmlInputElement>() {
                let key = e.key();

                match key.as_str() {
                    "Enter" => {
                        e.prevent_default();
                        let command_line = (*current_line).clone();
                        let command = command_line.strip_prefix(&format!("{}", if let Some(tag) = &*current_tag { format!("/{}/ > ", tag) } else { "~ > ".to_string() }))
                                                .unwrap_or(&command_line).trim().to_string();

                        if !command.is_empty() {
                            // Add command line to completed history
                            let mut new_completed_lines = (*completed_lines).clone();
                            new_completed_lines.push(command_line);
                            completed_lines.set(new_completed_lines);

                            // Process command
                            let parts: Vec<&str> = command.split_whitespace().collect();
                            let cmd_name = parts.get(0).map(|s| *s).unwrap_or("");

                            // Handle internal commands (clear, theme, cd)
                            match cmd_name {
                                "clear" => {
                                    completed_lines.set(Vec::new());
                                    outputs.set(Vec::new());
                                    // Add empty output for clear command's position in output history
                                    let mut current_outputs = (*outputs).clone();
                                    current_outputs.push("".to_string());
                                    outputs.set(current_outputs);
                                },
                                "theme" => {
                                    is_dark.set(!*is_dark);
                                    let output = if *is_dark { "Switched to dark theme" } else { "Switched to light theme" }.to_string();
                                     // Add output for theme command
                                    let mut current_outputs = (*outputs).clone();
                                    current_outputs.push(output);
                                    outputs.set(current_outputs);
                                },
                                "cd" => {
                                    let args: Vec<&str> = parts.get(1..).unwrap_or(&[]).to_vec();
                                    if args.is_empty() {
                                        // cd to root
                                        current_tag.set(None);
                                    } else if args.len() == 1 {
                                        let tag = args[0].to_string();
                                        // In a real scenario, you might want to validate if the tag exists
                                        current_tag.set(Some(tag.clone()));
                                    }
                                    // Add empty output for cd command's position in output history
                                    let mut current_outputs = (*outputs).clone();
                                    current_outputs.push("".to_string());
                                    outputs.set(current_outputs);
                                },
                                _ => {
                                    // Handle other commands using handle_command
                                    // Pass the current tag for 'ls' command if applicable
                                    let command_parts_for_handler: Vec<String> = {
                                        let mut owned_parts: Vec<String> = parts.iter().map(|&s| s.to_string()).collect();
                                        if cmd_name == "ls" {
                                             if let Some(tag) = (*current_tag).clone() {
                                                // If ls is used in a tagged directory, add the tag as an argument
                                                if owned_parts.len() == 1 {
                                                    owned_parts.push(tag);
                                                } else if owned_parts.len() > 1 && !owned_parts[1].starts_with("-") {
                                                    // Only insert if there isn't already an argument that looks like an option
                                                    owned_parts.insert(1, tag);
                                                } else if owned_parts.len() > 1 && owned_parts[1].starts_with("-") {
                                                    // If there's an option, try to insert after it
                                                     if owned_parts.len() > 2 {
                                                        owned_parts.insert(2, tag);
                                                    } else {
                                                        owned_parts.push(tag);
                                                    }
                                                }
                                            }
                                        }
                                        owned_parts
                                    };

                                    let command_parts_str_refs: Vec<&str> = command_parts_for_handler.iter().map(|s| s.as_str()).collect();

                                    // handle_command returns the output string
                                    let output = handle_command(command_parts_str_refs, handle_output.clone()); // Pass handle_output for async commands still
                                    if !output.is_empty() {
                                        let mut current_outputs = (*outputs).clone();
                                        current_outputs.push(output);
                                        outputs.set(current_outputs);
                                    }
                                }
                            }
                        }

                        // Reset current line with new prompt and clear input field
                        let new_prompt = format!("{}", if let Some(tag) = &*current_tag { format!("/{}/ > ", tag) } else { "~ > ".to_string() });
                        current_line.set(new_prompt);
                        input.set_value(&*current_line);
                        history_nav_index.set(-1); // Reset history navigation
                        // Focus input after command execution
                        input.focus().ok();

                    },
                     // Handle history navigation
                     "ArrowUp" => {
                        e.prevent_default();
                        let history_len = completed_lines.len();
                        if history_len > 0 {
                            let mut new_index = *history_nav_index;
                            if new_index < (history_len as i32 - 1) {
                                new_index += 1;
                                history_nav_index.set(new_index);
                                // Load command from history into current line
                                if let Some(historical_line) = completed_lines.get((history_len as i32 - 1 - new_index) as usize) {
                                     current_line.set(historical_line.clone());
                                     input.set_value(historical_line);
                                }
                            }
                         }
                    },
                    "ArrowDown" => {
                        e.prevent_default();
                        let history_len = completed_lines.len();
                        if history_len > 0 {
                            let mut new_index = *history_nav_index;
                            if new_index > 0 {
                                new_index -= 1;
                                history_nav_index.set(new_index);
                                // Load command from history or clear input
                                if let Some(historical_line) = completed_lines.get((history_len as i32 - 1 - new_index) as usize) {
                                    current_line.set(historical_line.clone());
                                    input.set_value(historical_line);
                                } else { // Should not happen if new_index > 0, but as a safeguard
                                    let new_prompt = format!("{}", if let Some(tag) = &*current_tag { format!("/{}/ > ", tag) } else { "~ > ".to_string() });
                                    current_line.set(new_prompt.clone());
                                    input.set_value(&new_prompt);
                                }
                             } else { // Already at the newest entry (or no history navigation yet)
                                // Clear input field and reset current line to just the prompt
                                let new_prompt = format!("{}", if let Some(tag) = &*current_tag { format!("/{}/ > ", tag) } else { "~ > ".to_string() });
                                current_line.set(new_prompt.clone());
                                input.set_value(&new_prompt);
                                history_nav_index.set(-1);
                             }
                        }
                    },
                    // Handle cursor movement and deletion within current line
                    "ArrowLeft" | "ArrowRight" | "Backspace" | "Delete" | "Home" | "End" => {
                        // Allow default browser behavior for now, oninput will sync state
                         // TODO: Implement custom handling for more precise control if needed
                    }
                    _ => {
                        // For other key presses, allow default browser behavior.
                        // The oninput handler will keep current_line state in sync.
                    }
                }
            }
        })
    };

    // Handle input changes - keep current_line state in sync with input field
    let oninput = {
        let current_line = current_line.clone();
        let current_tag = current_tag.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                let value = input.value();
                let prompt = if let Some(tag) = &*current_tag { 
                    format!("/{}/ > ", tag)
                } else { 
                    "~ > ".to_string() 
                };
                
                // Ensure the prompt is always present and can't be deleted
                if !value.starts_with(&prompt) {
                    input.set_value(&prompt);
                    current_line.set(prompt);
                } else {
                    current_line.set(value);
                }
            }
        })
    };

    // Add effect to update prompt when tag changes
    {
        let current_tag = current_tag.clone();
        let current_line = current_line.clone();
        let input_ref = input_ref.clone();
        use_effect_with(current_tag, move |tag| {
            if let Some(input) = input_ref.cast::<HtmlInputElement>() {
                let new_prompt = if let Some(tag) = &**tag {
                    format!("/{}/ > ", tag)
                } else {
                    "~ > ".to_string()
                };
                current_line.set(new_prompt.clone());
                input.set_value(&new_prompt);
            }
            || ()
        });
    }

    // Auto-scroll to bottom effect
    let scroll_ref = use_node_ref();
    {
        let outputs = outputs.clone();
        let scroll_ref = scroll_ref.clone();
        use_effect_with(
            outputs,
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
                        {"Type 'help' for available commands.\n"}
                    </div>
                    {
                        completed_lines.iter().enumerate().map(|(i, line)| {
                             let key = format!("history-{}", i);
                             html! {
                                 <div key={key} class="mb-2">
                                     <div class="flex items-start group">
                                         <span class="text-github-light-text dark:text-github-dark-text mr-2 select-none opacity-80 terminal-prompt">{ &line }</span>
                                     </div>
                                     {render_output(outputs.get(i))}
                                 </div>
                             }
                        }).collect::<Html>()
                    }
                    <div class="flex items-start group">
                        <input
                            type="text"
                            ref={input_ref}
                            value={(*current_line).clone()}
                            {onkeydown}
                            {oninput}
                            autofocus=true
                            class="bg-transparent outline-none border-none text-github-light-text dark:text-github-dark-text w-full terminal-input"
                            placeholder=" "
                            spellcheck="false"
                            autocomplete="off"
                         />
                    </div>
                    <div ref={scroll_ref}></div>
                </div>
            </div>
        </div>
    }
}
