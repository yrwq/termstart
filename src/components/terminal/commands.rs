use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use gloo::net::http::Request;
use gloo::storage::{LocalStorage, Storage};
use serde_json::json;

use crate::components::terminal::user::{User, STORAGE_KEY};

pub fn handle_command(
    parts: Vec<&str>,
    user: &UseStateHandle<User>,
) -> String {
    let cmd = parts.get(0).map(|s| *s).unwrap_or("");
    
    match cmd {
        "help" => {
            if user.is_authenticated {
                "Available commands:\n  help     - Show this help message\n  clear    - Clear the terminal\n  version  - Show version information\n  logout   - Log out of your account\n  whoami   - Show current user information".to_string()
            } else {
                "Available commands:\n  help     - Show this help message\n  clear    - Clear the terminal\n  version  - Show version information\n  register - Create a new account\n  login    - Log in to your account".to_string()
            }
        },
        "version" => {
            "termstart v0.1.0".to_string()
        },
        "whoami" => {
            if user.is_authenticated {
                format!("Logged in as: {}\nEmail: {}", user.username, user.email)
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
                
                let user_clone = user.clone();
                
                spawn_local(async move {
                    match Request::post("http://localhost:8080/api/register")
                        .json(&json!({
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
                                        user_clone.set(user_data);
                                    }
                                    Err(_) => {}
                                }
                            }
                        }
                        Err(_) => {}
                    }
                });
                
                "Registration request sent...".to_string()
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
                
                let user_clone = user.clone();
                
                spawn_local(async move {
                    match Request::post("http://localhost:8080/api/login")
                        .json(&json!({
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
                                        user_data.is_authenticated = true;
                                        LocalStorage::set(STORAGE_KEY, &user_data).unwrap();
                                        user_clone.set(user_data);
                                    }
                                    Err(_) => {}
                                }
                            }
                        }
                        Err(_) => {}
                    }
                });
                
                "Login request sent...".to_string()
            } else {
                "Usage: login <email> <password>".to_string()
            }
        },
        "" => String::new(),
        cmd => {
            format!("Command not found: {}\nType 'help' for available commands.", cmd)
        }
    }
}
