pub mod commands;

use yew::prelude::*;
use web_sys::{HtmlInputElement, KeyboardEvent, InputEvent};
use lucide_yew::Folder;
use lucide_yew::File;
use termstart::services::auth::AuthService;
use termstart::services::bookmark::BookmarkService;
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen::{JsCast, closure::Closure};
use std::rc::Rc;
use std::cell::RefCell;
use termstart::config::Config;

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
    let current_line = use_state(|| "".to_string());
    let history_nav_index = use_state(|| -1);
    let is_dark = use_state(|| {
        if let Some(window) = web_sys::window() {
            if let Some(media_query) = window.match_media("(prefers-color-scheme: dark)").ok().flatten() {
                return media_query.matches();
            }
        }
        false
    });
    let current_tag = use_state(|| None::<String>);

    let available_commands = ["help", "fetch", "theme", "version", "whoami", "clear", "cd", "register", "login", "logout", "ls", "cat", "touch", "open", "rm", "tag", "search", "tree"];
    let tags = use_state(|| Vec::<String>::new());
    let bookmarks = use_state(|| Vec::<String>::new());
    let suggestion = use_state(|| None::<String>);

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
        use_effect(move || {
            if let Some(window) = web_sys::window() {
                if let Some(media_query) = window.match_media("(prefers-color-scheme: dark)").ok().flatten() {
                    let callback = Closure::wrap(Box::new(move |e: web_sys::MediaQueryListEvent| {
                        is_dark.set(e.matches());
                    }) as Box<dyn FnMut(_)>);
                    
                    media_query.set_onchange(Some(callback.as_ref().unchecked_ref()));
                    callback.forget();
                }
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

    {
        let tags = tags.clone();
        let bookmarks = bookmarks.clone();
        use_effect(move || {
            spawn_local(async move {
                if let Some(user) = AuthService::get_current_user() {
                    if !user.id.is_empty() {
                        let config = Config::load();
                        let bookmark_service = BookmarkService::new(
                            config.supabase_url.clone(),
                            config.supabase_key.clone(),
                        );
                        
                        match bookmark_service.get_bookmarks(None).await {
                            Ok(bookmarks_list) => {
                                let mut all_tags = std::collections::HashSet::new();
                                for bookmark in &bookmarks_list {
                                    all_tags.extend(bookmark.tags.clone());
                                }
                                tags.set(all_tags.into_iter().collect());
                                bookmarks.set(bookmarks_list.into_iter().map(|b| b.name).collect());
                            }
                            Err(_) => {}
                        }
                    }
                }
            });
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
        let available_commands = available_commands.clone();
        let tags = tags.clone();
        let bookmarks = bookmarks.clone();

        Callback::from(move |e: KeyboardEvent| {
            web_sys::console::log_1(&"[DEBUG] onkeydown handler called".into());
            if let Some(input) = input_ref.cast::<HtmlInputElement>() {
                let key = e.key();
                web_sys::console::log_2(&"[DEBUG] Key pressed:".into(), &key.clone().into());

                match key.as_str() {
                    "Enter" => {
                        e.prevent_default();
                        let prompt = if let Some(tag) = &*current_tag { format!("/{}/ > ", tag) } else { "~ > ".to_string() };
                        let command_line = format!("{}{}", prompt, (*current_line).clone());
                        let command = (*current_line).trim().to_string();

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
                                    let command_line_clone = command_line.clone();
                                    let current_tag_clone = current_tag.clone();

                                    let id = history.next_id;
                                    history.dispatch(HistoryAction::AddCommand { command: command_line_clone.clone(), id });

                                    let history_dispatch = history.dispatcher();
                                    spawn_local(async move {
                                        let mut command_parts: Vec<String> = command.split_whitespace().map(|s| s.to_string()).collect();
                                        let current_tag_inside_async = current_tag_clone.clone();
                                        
                                        // Add current tag to ls command if in a tagged directory
                                        if command_parts.get(0).map(|s| s.as_str()) == Some("ls") {
                                            if let Some(tag) = &*current_tag_inside_async {
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

                        current_line.set("".to_string());
                        input.set_value("");
                        history_nav_index.set(-1);
                        input.focus().ok();
                    },
                    "Tab" => {
                        e.prevent_default();
                        let value = input.value();
                        let parts: Vec<&str> = value.split_whitespace().collect();
                        
                        if parts.is_empty() {
                            return;
                        }

                        let current_word = parts.last().unwrap();
                        let prefix = parts[..parts.len()-1].join(" ");
                        let prefix = if prefix.is_empty() { String::new() } else { format!("{} ", prefix) };

                        let completions = if parts.len() == 1 {
                            // Command completion
                            available_commands.iter()
                                .filter(|cmd| cmd.starts_with(current_word))
                                .map(|s| s.to_string())
                                .collect::<Vec<_>>()
                        } else if AuthService::get_current_user().is_none() {
                            // No completions if not authenticated
                            Vec::new()
                        } else if parts[0] == "cd" || parts[0] == "ls" {
                            // Tag completion for cd and ls commands
                            tags.iter()
                                .filter(|tag| tag.starts_with(current_word))
                                .map(|s| s.to_string())
                                .collect::<Vec<_>>()
                        } else if parts[0] == "cat" || parts[0] == "open" || parts[0] == "rm" {
                            // Bookmark completion for cat, open, rm commands
                            bookmarks.iter()
                                .filter(|name| name.starts_with(current_word))
                                .map(|s| s.to_string())
                                .collect::<Vec<_>>()
                        } else if parts[0] == "tag" && parts.len() >= 3 {
                            // Tag completion for tag command's tag arguments
                            if parts[1] == "add" || parts[1] == "remove" {
                                tags.iter()
                                    .filter(|tag| tag.starts_with(current_word))
                                    .map(|s| s.to_string())
                                    .collect::<Vec<_>>()
                            } else {
                                Vec::new()
                            }
                        } else if parts[0] == "tag" && parts.len() == 2 {
                            // Bookmark completion for tag command's bookmark argument
                            bookmarks.iter()
                                .filter(|name| name.starts_with(current_word))
                                .map(|s| s.to_string())
                                .collect::<Vec<_>>()
                        } else {
                            Vec::new()
                        };

                        if completions.len() == 1 {
                            // Single completion
                            let completion = completions[0].clone();
                            let new_value = format!("{}{}", prefix, completion);
                            current_line.set(new_value.clone());
                            input.set_value(&new_value);
                            input.set_selection_range(new_value.len() as u32, new_value.len() as u32).ok();
                        } else if completions.len() > 1 {
                            // Multiple completions - show them inline
                            let mut output = String::new();
                            let mut max_length = 0;
                            
                            // First pass: calculate max length
                            for completion in &completions {
                                max_length = max_length.max(completion.len());
                            }
                            
                            // Second pass: format completions in columns
                            let columns = 3;
                            let rows = (completions.len() + columns - 1) / columns;
                            
                            for row in 0..rows {
                                for col in 0..columns {
                                    if let Some(completion) = completions.get(row + col * rows) {
                                        output.push_str(&format!("{:<width$}  ", completion, width = max_length));
                                    }
                                }
                                output.push('\n');
                            }
                            
                            let id = history.next_id;
                            history.dispatch(HistoryAction::SetOutput { 
                                id, 
                                command: format!("{}{}", prefix, current_word), 
                                output 
                            });
                        }
                    },
                    "ArrowLeft" => {
                         if let Some(start) = input.selection_start().ok().flatten().map(|s| s as usize) {
                            web_sys::console::log_2(&"[DEBUG] ArrowLeft - selectionStart:".into(), &start.into());
                            let new_pos = start.saturating_sub(1);
                            input.set_selection_range(new_pos as u32, new_pos as u32).ok();
                             web_sys::console::log_2(&"[DEBUG] ArrowLeft - new cursor_position:".into(), &new_pos.into());
                        }
                    },
                    "ArrowRight" => {
                        if let Some(start) = input.selection_start().ok().flatten().map(|s| s as usize) {
                            web_sys::console::log_2(&"[DEBUG] ArrowRight - selectionStart:".into(), &start.into());
                             let new_pos = start + 1;
                             input.set_selection_range(new_pos as u32, new_pos as u32).ok();
                             web_sys::console::log_2(&"[DEBUG] ArrowRight - new cursor_position:".into(), &new_pos.into());
                        }
                    },
                    "ArrowUp" => {
                        e.prevent_default();
                        let current_index = *history_nav_index;
                        if current_index < (history.entries.len() as i32 - 1) {
                            let new_index = current_index + 1;
                            history_nav_index.set(new_index);
                            if let Some((command, _, _)) = history.entries.get((history.entries.len() - 1 - new_index as usize)) {
                                let prompt = if let Some(tag) = &*current_tag { format!("/{}/ > ", tag) } else { "~ > ".to_string() };
                                let command_text = command.strip_prefix(&prompt).unwrap_or(command);
                                current_line.set(command_text.to_string());
                                input.set_value(command_text);
                                input.set_selection_range(command_text.len() as u32, command_text.len() as u32).ok();
                            }
                        }
                    },
                    "ArrowDown" => {
                        e.prevent_default();
                        let current_index = *history_nav_index;
                        if current_index > 0 {
                            let new_index = current_index - 1;
                            history_nav_index.set(new_index);
                            if let Some((command, _, _)) = history.entries.get((history.entries.len() - 1 - new_index as usize)) {
                                let prompt = if let Some(tag) = &*current_tag { format!("/{}/ > ", tag) } else { "~ > ".to_string() };
                                let command_text = command.strip_prefix(&prompt).unwrap_or(command);
                                current_line.set(command_text.to_string());
                                input.set_value(command_text);
                                input.set_selection_range(command_text.len() as u32, command_text.len() as u32).ok();
                            }
                        } else if current_index == 0 {
                            history_nav_index.set(-1);
                            current_line.set("".to_string());
                            input.set_value("");
                        }
                    },
                    "Home" => {
                        let prompt = if let Some(tag) = &*current_tag { format!("/{}/ > ", tag) } else { "~ > ".to_string() };
                        let new_pos = prompt.len();
                        input.set_selection_range(new_pos as u32, new_pos as u32).ok();
                        web_sys::console::log_2(&"[DEBUG] Home - new cursor_position:".into(), &new_pos.into());
                    },
                    "End" => {
                        let new_pos = input.value().len();
                        input.set_selection_range(new_pos as u32, new_pos as u32).ok();
                        web_sys::console::log_2(&"[DEBUG] End - new cursor_position:".into(), &new_pos.into());
                    },
                    "Backspace" | "Delete" => {
                         if let Some(start) = input.selection_start().ok().flatten().map(|s| s as usize) {
                             web_sys::console::log_2(&"[DEBUG] Backspace/Delete - selectionStart:".into(), &start.into());
                             let new_pos = if key.as_str() == "Backspace" { start.saturating_sub(1) } else { start };
                              web_sys::console::log_2(&"[DEBUG] Backspace/Delete - new cursor_position:".into(), &new_pos.into());
                         }
                    },
                    _ => {
                    }
                }
            }
        })
    };

    let oninput = {
        let current_line = current_line.clone();
        let input_ref = input_ref.clone();
        let available_commands = available_commands.clone();
        let tags = tags.clone();
        let bookmarks = bookmarks.clone();
        let suggestion = suggestion.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = input_ref.cast::<HtmlInputElement>() {
                let value = input.value();
                current_line.set(value.clone());
                
                // Generate suggestion
                let parts: Vec<&str> = value.split_whitespace().collect();
                if !parts.is_empty() {
                    let current_word = parts.last().unwrap();
                    if current_word.is_empty() {
                        suggestion.set(None);
                        return;
                    }
                    let prefix = parts[..parts.len()-1].join(" ");
                    let prefix = if prefix.is_empty() { String::new() } else { format!("{} ", prefix) };

                    let completions = if parts.len() == 1 {
                        // Command completion
                        available_commands.iter()
                            .filter(|cmd| cmd.starts_with(current_word))
                            .map(|s| s.to_string())
                            .collect::<Vec<_>>()
                    } else if AuthService::get_current_user().is_none() {
                        // No completions if not authenticated
                        Vec::new()
                    } else if parts[0] == "cd" || parts[0] == "ls" {
                        // Tag completion for cd and ls commands
                        tags.iter()
                            .filter(|tag| tag.starts_with(current_word))
                            .map(|s| s.to_string())
                            .collect::<Vec<_>>()
                    } else if parts[0] == "cat" || parts[0] == "open" || parts[0] == "rm" {
                        // Bookmark completion for cat, open, rm commands
                        bookmarks.iter()
                            .filter(|name| name.starts_with(current_word))
                            .map(|s| s.to_string())
                            .collect::<Vec<_>>()
                    } else if parts[0] == "tag" && parts.len() >= 3 {
                        // Tag completion for tag command's tag arguments
                        if parts[1] == "add" || parts[1] == "remove" {
                            tags.iter()
                                .filter(|tag| tag.starts_with(current_word))
                                .map(|s| s.to_string())
                                .collect::<Vec<_>>()
                        } else {
                            Vec::new()
                        }
                    } else if parts[0] == "tag" && parts.len() == 2 {
                        // Bookmark completion for tag command's bookmark argument
                        bookmarks.iter()
                            .filter(|name| name.starts_with(current_word))
                            .map(|s| s.to_string())
                            .collect::<Vec<_>>()
                    } else {
                        Vec::new()
                    };

                    if completions.len() == 1 {
                        let completion = completions[0].clone();
                        if completion != current_word.to_string() {
                            suggestion.set(Some(format!("{}{}", prefix, completion)));
                        } else {
                            suggestion.set(None);
                        }
                    } else {
                        suggestion.set(None);
                    }
                } else {
                    suggestion.set(None);
                }
            }
        })
    };

    // Add effect to update prompt when tag changes
    {
        let current_tag = current_tag.clone();
        let input_ref = input_ref.clone();
        use_effect_with((*current_tag).clone(), move |tag| {
            if let Some(input) = input_ref.cast::<HtmlInputElement>() {
                let new_prompt = if let Some(tag_str) = tag {
                    format!("/{}/ > ", tag_str)
                } else {
                    "~ > ".to_string()
                };
                // Only set cursor position after the prompt
                let prompt_len = new_prompt.len();
                input.set_selection_range(prompt_len as u32, prompt_len as u32).ok();
            }
            || ()
        });
    }

    // Auto-scroll to bottom effect
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
                <div class="w-full max-w-5xl p-4 bg-github-light-bg/70 dark:bg-github-dark-bg/70 rounded-lg shadow-xl font-mono text-github-light-text dark:text-github-dark-text border border-github-light-border dark:border-github-dark-border backdrop-blur-sm">
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
                            <span class="text-github-light-text dark:text-github-dark-text mr-2 select-none opacity-80 terminal-prompt">{if let Some(tag) = &*current_tag { format!("/{}/ > ", tag) } else { "~ > ".to_string() }}</span>
                            <div class="relative flex-grow">
                                <input
                                    type="text"
                                    ref={input_ref}
                                    value={(*current_line).clone()}
                                    onkeydown={onkeydown}
                                    oninput={oninput}
                                    autofocus=true
                                    class={classes!(
                                        "bg-transparent",
                                        "outline-none",
                                        "border-none",
                                        "flex-grow",
                                        "terminal-input",
                                        {
                                            let command_text = (*current_line).split_whitespace().next().unwrap_or("").to_string();
                                            let is_command_available = available_commands.contains(&command_text.as_str());
                                            if is_command_available && !command_text.is_empty() {
                                                "text-[#238636]"
                                            } else if !command_text.is_empty() {
                                                "text-[#f85149]"
                                            } else {
                                                ""
                                            }
                                        }
                                    )}
                                    placeholder=" "
                                    spellcheck="false"
                                    autocomplete="off"
                                />
                                if let Some(sugg) = &*suggestion {
                                    <div class="absolute top-0 left-0 pointer-events-none text-github-light-text/30 dark:text-github-dark-text/30">
                                        { sugg }
                                    </div>
                                }
                            </div>
                        </div>
                        <div ref={scroll_ref}></div>
                    </div>
                </div>
            </div>
        </>
    }
}
