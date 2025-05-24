pub mod commands;

use yew::prelude::*;
use web_sys::{HtmlInputElement, KeyboardEvent, InputEvent};
use lucide_yew::Folder;
use lucide_yew::File;
use termstart::services::auth::AuthService;
use wasm_bindgen_futures::spawn_local;

use crate::components::terminal::commands::handle_command;

#[derive(Default, Clone, PartialEq)]
pub struct TerminalHistory {
    pub entries: Vec<(String, String, usize)>,
    pub next_id: usize,
}

#[derive(Clone)]
pub enum HistoryAction {
    AddCommand { command: String, id: usize },
    SetOutput { id: usize, command: String, output: String },
    Clear,
}

impl yew::Reducible for TerminalHistory {
    type Action = HistoryAction;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        let mut history = (*self).clone();
        match action {
            HistoryAction::AddCommand { command, id } => {
                history.entries.push((command, "".to_string(), id));
                history.next_id += 1;
            }
            HistoryAction::SetOutput { id, command, output } => {
                if let Some(entry) = history.entries.iter_mut().find(|(_, _, entry_id)| *entry_id == id) {
                    *entry = (command, output, id);
                }
            }
            HistoryAction::Clear => {
                history.entries.clear();
                history.next_id = 0;
            }
        }
        history.into()
    }
}

fn render_output_line(line: &str) -> Html {
    let trimmed_line = line.trim();
    if trimmed_line.starts_with("TAG_ITEM:") {
        let tag_name = trimmed_line.strip_prefix("TAG_ITEM:").unwrap_or("").trim();
        html! {
            <div class="flex items-center">
                <Folder class="w-4 h-4 inline-block mr-1" />
                <span>{ format!("{}/", tag_name) }</span>
            </div>
        }
    } else if trimmed_line.starts_with("BOOKMARK_ITEM:") {
        let content = trimmed_line.strip_prefix("BOOKMARK_ITEM:").unwrap_or("").trim();
        if content.starts_with("├── ") || content.starts_with("└── ") {
            let parts: Vec<&str> = content.splitn(2, ' ').collect();
            let prefix = parts.get(0).unwrap_or(&"").trim();
            let name = parts.get(1).unwrap_or(&"").trim();
            html! {
                <div class="flex items-center">
                    <span class="mr-1">{ prefix }</span>
                    <File class="w-4 h-4 inline-block mr-1" />
                    <span>{ name }</span>
                </div>
            }
        } else {
            let content_parts: Vec<&str> = content.splitn(2, ' ').collect();
            let name = content_parts.get(0).unwrap_or(&"").trim().to_string();
            let tags = if content_parts.len() > 1 {
                format!(" [{}]", content_parts[1..].join(" ").trim())
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
        html! { <div class="min-h-[1rem]"></div> }
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
    let history = use_reducer(|| TerminalHistory { entries: Vec::new(), next_id: 0 });
    let current_line = use_state(String::new);
    let history_nav_index = use_state(|| -1);
    let is_dark = use_state(|| false);
    let current_tag = use_state(|| None::<String>);

    {
        let input_ref = input_ref.clone();
        use_effect(move || {
            if let Some(input) = input_ref.cast::<HtmlInputElement>() {
                input.focus().ok();
            }
            || ()
        });
    }

    {
        let is_dark = is_dark.clone();
        use_effect_with(*is_dark, move |&is_dark| {
            let document = web_sys::window()
                .unwrap()
                .document()
                .unwrap();
            
            if is_dark {
                document.document_element().unwrap().class_list().add_1("dark").unwrap();
            } else {
                document.document_element().unwrap().class_list().remove_1("dark").unwrap();
            }
            || ()
        });
    }

    let onkeydown = {
        let input_ref = input_ref.clone();
        let history = history.clone();
        let current_line = current_line.clone();
        let history_nav_index = history_nav_index.clone();
        let is_dark = is_dark.clone();
        let current_tag = current_tag.clone();

        Callback::from(move |e: KeyboardEvent| {
            web_sys::console::log_1(&"[DEBUG] onkeydown handler called".into());
            if let Some(input) = input_ref.cast::<HtmlInputElement>() {
                let key = e.key();

                match key.as_str() {
                    "Enter" => {
                        e.prevent_default();
                        let command_line = (*current_line).clone();
                        let command = command_line.strip_prefix(&format!("{}", if let Some(tag) = &*current_tag { format!("/{}/ > ", tag) } else { "~ > ".to_string() }))
                                                .unwrap_or(&command_line).trim().to_string();

                        if !command.is_empty() {
                            let parts: Vec<String> = command.split_whitespace().map(|s| s.to_string()).collect();
                            let cmd_name = parts.get(0).cloned().unwrap_or_default();

                            match cmd_name.as_str() {
                                "clear" => {
                                    history.dispatch(HistoryAction::Clear);
                                    web_sys::console::log_1(&"[DEBUG] history.set called for clear".into());
                                },
                                "theme" => {
                                    is_dark.set(!*is_dark);
                                },
                                "cd" => {
                                    let args: Vec<&str> = parts.get(1..).unwrap_or(&[]).iter().map(|s| s.as_str()).collect();
                                    if args.is_empty() {
                                        current_tag.set(None);
                                        let id = history.next_id;
                                        history.dispatch(HistoryAction::AddCommand { command: command_line.clone(), id });
                                        web_sys::console::log_1(&"[DEBUG] history.set called for cd to root".into());
                                    } else if args.len() == 1 {
                                        let tag = args[0].to_string();
                                        let current_tag_clone = current_tag.clone();

                                        let command_line_clone = command_line.clone();

                                        let id = history.next_id;
                                        history.dispatch(HistoryAction::AddCommand { command: command_line_clone.clone(), id });
                                        let history_dispatch = history.dispatcher();
                                        spawn_local(async move {
                                            match AuthService::get_current_user() {
                                                Some(user) => {
                                                    if user.id.is_empty() {
                                                        history_dispatch.dispatch(HistoryAction::SetOutput {
                                                            id,
                                                            command: command_line_clone.clone(),
                                                            output: format!("Tag '{}' not found.", tag),
                                                        });
                                                    } else {
                                                        current_tag_clone.set(Some(tag));
                                                        history_dispatch.dispatch(HistoryAction::SetOutput {
                                                            id,
                                                            command: command_line_clone.clone(),
                                                            output: "".to_string(),
                                                        });
                                                    }
                                                }
                                                None => {
                                                    history_dispatch.dispatch(HistoryAction::SetOutput {
                                                        id,
                                                        command: command_line_clone.clone(),
                                                        output: "Not authenticated. Please login first.".to_string(),
                                                    });
                                                }
                                            }
                                        });
                                    }
                                },
                                _ => {
                                    let history = history.clone();
                                    let parts_clone = parts.clone();
                                    let current_tag_clone = current_tag.clone();
                                    let command_line_clone = command_line.clone();

                                    let id = history.next_id;
                                    history.dispatch(HistoryAction::AddCommand { command: command_line_clone.clone(), id });

                                    let history_dispatch = history.dispatcher();
                                    spawn_local(async move {
                                        let mut command_parts = parts_clone;
                                        if cmd_name == "ls" {
                                            if let Some(tag) = &*current_tag_clone {
                                                if command_parts.len() == 1 {
                                                    command_parts.push(tag.clone());
                                                }
                                            }
                                        }

                                        let parts_refs: Vec<&str> = command_parts.iter().map(|s| s.as_str()).collect();
                                        let result = handle_command(parts_refs).await;

                                        web_sys::console::info_1(&format!("Command result for {}: {:?}", command_line_clone, result).into());

                                        let output = match result {
                                            Ok(output) => output,
                                            Err(e) => {
                                                web_sys::console::error_1(&format!("Command execution error: {}", e).into());
                                                if e.to_string().contains("Not authenticated") {
                                                    web_sys::console::log_1(&format!("[DEBUG] Auth status: {:?}", AuthService::get_current_user()).into());
                                                    "Error: Authentication issue detected. Please try logging in again with 'login' command.".to_string()
                                                } else {
                                                    format!("Error: {}", e)
                                                }
                                            },
                                        };

                                        history_dispatch.dispatch(HistoryAction::SetOutput {
                                            id,
                                            command: command_line_clone,
                                            output,
                                        });
                                    });
                                }
                            }
                        }

                        let new_prompt = format!("{}", if let Some(tag) = &*current_tag { format!("/{}/ > ", tag) } else { "~ > ".to_string() });
                        current_line.set(new_prompt.clone());
                        input.set_value(&new_prompt);
                        history_nav_index.set(-1);
                        input.focus().ok();
                    },
                    "ArrowUp" => {
                        e.prevent_default();
                        let history_len = history.entries.len();
                        if history_len > 0 {
                            let mut new_index = *history_nav_index;
                            if new_index < (history_len as i32 - 1) {
                                new_index += 1;
                                history_nav_index.set(new_index);
                                if let Some((command, _, _)) = history.entries.get((history_len as i32 - 1 - new_index) as usize) {
                                    current_line.set(command.clone());
                                    input.set_value(command);
                                }
                            }
                        }
                    },
                    "ArrowDown" => {
                        e.prevent_default();
                        let history_len = history.entries.len();
                        if history_len > 0 {
                            let mut new_index = *history_nav_index;
                            if new_index > 0 {
                                new_index -= 1;
                                history_nav_index.set(new_index);
                                if let Some((command, _, _)) = history.entries.get((history_len as i32 - 1 - new_index) as usize) {
                                    current_line.set(command.clone());
                                    input.set_value(command);
                                } else {
                                    let new_prompt = format!("{}", if let Some(tag) = &*current_tag { format!("/{}/ > ", tag) } else { "~ > ".to_string() });
                                    current_line.set(new_prompt.clone());
                                    input.set_value(&new_prompt);
                                }
                            } else {
                                let new_prompt = format!("{}", if let Some(tag) = &*current_tag { format!("/{}/ > ", tag) } else { "~ > ".to_string() });
                                current_line.set(new_prompt.clone());
                                input.set_value(&new_prompt);
                                history_nav_index.set(-1);
                            }
                        }
                    },
                    "ArrowLeft" | "ArrowRight" | "Backspace" | "Delete" | "Home" | "End" => {

                    },
                    _ => {

                    }
                }
            }
        })
    };

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

                if !value.starts_with(&prompt) {
                    input.set_value(&prompt);
                    current_line.set(prompt);
                } else {
                    current_line.set(value);
                }
            }
        })
    };

    {
        let current_line = current_line.clone();
        let current_tag = current_tag.clone();
        let input_ref = input_ref.clone();
        use_effect_with((*current_tag).clone(), move |tag| {
            if let Some(input) = input_ref.cast::<HtmlInputElement>() {
                let new_prompt = if let Some(tag_str) = tag {
                    format!("/{}/ > ", tag_str)
                } else {
                    "~ > ".to_string()
                };
                current_line.set(new_prompt.clone());
                input.set_value(&new_prompt);
            }
            || ()
        });
    }

    let scroll_ref = use_node_ref();
    {
        let history = history.clone();
        let scroll_ref = scroll_ref.clone();
        use_effect_with((*history).entries.clone(), move |_entries| {
            if let Some(node) = scroll_ref.cast::<web_sys::HtmlElement>() {
                node.scroll_into_view();
            }
            || ()
        });
    }

    html! {
        <>
            <div class="w-full h-screen flex items-center justify-center">
                <div class="w-full max-w-5xl p-4 bg-github-light-bg/70 dark:bg-github-dark-bg/70 rounded-lg shadow-xl font-mono text-github-light-text dark:text-github-dark-text border border-github-light-border dark:border-github-dark-border backdrop-blur-lg">
                    <div class="flex items-center mb-2 px-2">
                        <div class="flex space-x-2">
                            <div class="w-3 h-3 rounded-full bg-red-500"></div>
                            <div class="w-3 h-3 rounded-full bg-yellow-500"></div>
                            <div class="w-3 h-3 rounded-full bg-green-500"></div>
                        </div>
                        <div class="ml-4 text-sm text-github-light-text dark:text-github-dark-text opacity-70">{"termstart v0.1.0"}</div>
                    </div>
                    <div class="overflow-y-auto h-[600px] whitespace-pre-wrap rounded p-4">
                        { 
                            history.entries.iter().enumerate().map(|(i, (command, output, _id))| {
                                 let key = format!("history-{}-{}", i, command);
                                 html! {
                                     <div key={key} class="mb-2">
                                         <div class="flex items-start group">
                                             <span class="text-github-light-text dark:text-github-dark-text mr-2 select-none opacity-80 terminal-prompt">{ &command }</span>
                                         </div>
                                         { render_output(Some(output)) }
                                     </div>
                                 }
                            }).collect::<Html>()
                        }
                        
                        <div class="flex items-start group">
                            <input
                                type="text"
                                ref={input_ref}
                                value={(*current_line).clone()}
                                onkeydown={onkeydown}
                                oninput={oninput}
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
        </>
    }
}
