use yew::prelude::*;
use web_sys::HtmlInputElement;
use gloo::storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use gloo::net::http::Request;

#[derive(Default, Clone)]
struct TerminalHistory {
    commands: Vec<String>,
    outputs: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
struct User {
    username: String,
    email: String,
    token: String,
    is_authenticated: bool,
}

const STORAGE_KEY: &str = "termstart_user";

#[function_component(Terminal)]
pub fn terminal() -> Html {
    let input_ref = use_node_ref();
    let history = use_state(TerminalHistory::default);
    let current_input = use_state(String::new);
    let user: UseStateHandle<User> = use_state(|| {
        LocalStorage::get(STORAGE_KEY).unwrap_or_default()
    });

    let onkeydown = {
        let input_ref = input_ref.clone();
        let history = history.clone();
        let current_input = current_input.clone();
        let user = user.clone();
        
        Callback::from(move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                e.prevent_default();
                let input = input_ref.cast::<HtmlInputElement>().unwrap();
                let command = input.value();
                
                let parts: Vec<&str> = command.trim().split_whitespace().collect();
                let cmd = parts.get(0).map(|s| *s).unwrap_or("");
                
                let output = match cmd {
                    "help" => {
                        if user.is_authenticated {
                            "Available commands: help, clear, version, logout, whoami".to_string()
                        } else {
                            "Available commands: help, clear, version, register <email> <username> <password>, login <email> <password>".to_string()
                        }
                    },
                    "clear" => {
                        input.set_value("");
                        let mut new_history = (*history).clone();
                        new_history.commands.clear();
                        new_history.outputs.clear();
                        history.set(new_history);
                        String::new()
                    },
                    "version" => "termstart v0.1.0".to_string(),
                    "whoami" => {
                        if user.is_authenticated {
                            format!("Logged in as: {}", user.username)
                        } else {
                            "Not logged in".to_string()
                        }
                    },
                    "logout" => {
                        if user.is_authenticated {
                            LocalStorage::delete(STORAGE_KEY);
                            user.set(User::default());
                            "Logged out successfully".to_string()
                        } else {
                            "Not logged in".to_string()
                        }
                    },
                    "register" => {
                        if user.is_authenticated {
                            "Already logged in. Please logout first.".to_string()
                        } else if let (Some(email), Some(username), Some(password)) = (parts.get(1), parts.get(2), parts.get(3)) {
                            let email = email.to_string();
                            let username = username.to_string();
                            let password = password.to_string();
                            let history = history.clone();
                            let user = user.clone();
                            let register_msg = format!("Registering user {}...", username);
                            
                            spawn_local(async move {
                                match Request::post("http://localhost:7070/api/register")
                                    .json(&serde_json::json!({
                                        "email": email,
                                        "username": username,
                                        "password": password
                                    }))
                                    .unwrap()
                                    .send()
                                    .await
                                {
                                    Ok(response) => {
                                        if response.ok() {
                                            match response.json::<User>().await {
                                                Ok(user_data) => {
                                                    LocalStorage::set(STORAGE_KEY, &user_data).unwrap();
                                                    user.set(user_data);
                                                    let mut new_history = (*history).clone();
                                                    new_history.outputs.push("Registration successful! You are now logged in.".to_string());
                                                    history.set(new_history);
                                                }
                                                Err(_) => {
                                                    let mut new_history = (*history).clone();
                                                    new_history.outputs.push("Failed to parse registration response".to_string());
                                                    history.set(new_history);
                                                }
                                            }
                                        } else {
                                            let mut new_history = (*history).clone();
                                            new_history.outputs.push("Registration failed".to_string());
                                            history.set(new_history);
                                        }
                                    }
                                    Err(_) => {
                                        let mut new_history = (*history).clone();
                                        new_history.outputs.push("Network error during registration".to_string());
                                        history.set(new_history);
                                    }
                                }
                            });
                            register_msg
                        } else {
                            "Usage: register <email> <username> <password>".to_string()
                        }
                    },
                    "login" => {
                        if user.is_authenticated {
                            "Already logged in".to_string()
                        } else if let (Some(email), Some(password)) = (parts.get(1), parts.get(2)) {
                            let email = email.to_string();
                            let password = password.to_string();
                            let history = history.clone();
                            let user = user.clone();
                            let login_msg = format!("Logging in as {}...", email);
                            
                            spawn_local(async move {
                                match Request::post("http://localhost:7070/api/login")
                                    .json(&serde_json::json!({
                                        "email": email,
                                        "password": password
                                    }))
                                    .unwrap()
                                    .send()
                                    .await
                                {
                                    Ok(response) => {
                                        if response.ok() {
                                            match response.json::<User>().await {
                                                Ok(mut user_data) => {
                                                    user_data.is_authenticated = true;  // Ensure is_authenticated is set
                                                    LocalStorage::set(STORAGE_KEY, &user_data).unwrap();
                                                    user.set(user_data);
                                                    let mut new_history = (*history).clone();
                                                    new_history.outputs.push("Login successful!".to_string());
                                                    history.set(new_history);
                                                }
                                                Err(_) => {
                                                    let mut new_history = (*history).clone();
                                                    new_history.outputs.push("Failed to parse login response".to_string());
                                                    history.set(new_history);
                                                }
                                            }
                                        } else {
                                            let mut new_history = (*history).clone();
                                            new_history.outputs.push("Invalid credentials".to_string());
                                            history.set(new_history);
                                        }
                                    }
                                    Err(_) => {
                                        let mut new_history = (*history).clone();
                                        new_history.outputs.push("Network error during login".to_string());
                                        history.set(new_history);
                                    }
                                }
                            });
                            login_msg
                        } else {
                            "Usage: login <email> <password>".to_string()
                        }
                    },
                    "" => String::new(),
                    cmd => format!("Command not found: {}", cmd),
                };

                if !output.is_empty() {
                    let mut new_history = (*history).clone();
                    new_history.commands.push(command.clone());
                    new_history.outputs.push(output);
                    history.set(new_history);
                }

                input.set_value("");
                current_input.set(String::new());
            }
        })
    };

    let oninput = {
        let current_input = current_input.clone();
        Callback::from(move |e: InputEvent| {
            let input = e.target_unchecked_into::<HtmlInputElement>();
            current_input.set(input.value());
        })
    };

    html! {
        <div class="w-full max-w-3xl mx-auto mt-8 p-4 bg-github-light-button dark:bg-github-dark-button rounded-lg shadow-lg font-mono">
            <div class="mb-4 overflow-y-auto max-h-96 whitespace-pre-wrap">
                <div class="text-github-light-text dark:text-github-dark-text mb-4">
                    {"Welcome to termstart v0.1.0\nType 'help' for available commands.\n"}
                </div>
                {
                    history.commands.iter().enumerate().map(|(i, cmd)| {
                        html! {
                            <div key={i} class="mb-2">
                                <div class="flex items-start text-github-light-text dark:text-github-dark-text">
                                    <span class="text-green-500 mr-2 select-none">{"$"}</span>
                                    <span class="font-bold">{cmd}</span>
                                </div>
                                <div class="text-github-light-text dark:text-github-dark-text ml-4 opacity-90 font-light">
                                    {&history.outputs[i]}
                                </div>
                            </div>
                        }
                    }).collect::<Html>()
                }
            </div>
            <div class="flex items-center text-github-light-text dark:text-github-dark-text border-t border-github-light-border dark:border-github-dark-border pt-4">
                <span class="text-green-500 mr-2 select-none">{"$"}</span>
                <input
                    type="text"
                    ref={input_ref}
                    {onkeydown}
                    {oninput}
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