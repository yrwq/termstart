use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use gloo::net::http::Request;
use gloo::storage::{LocalStorage, Storage};
use serde_json::json;

use crate::components::terminal::{User, STORAGE_KEY};

pub fn handle_command(
    parts: Vec<&str>,
    user: &UseStateHandle<User>,
    handle_output: impl Fn(String) + 'static + Clone,
) -> String {
    let cmd = parts.get(0).map(|s| *s).unwrap_or("");
    
    match cmd {
        "help" => {
            if user.is_authenticated {
                let mut help = "Available commands:\n  help     - Show this help message\n  clear    - Clear the terminal\n  version  - Show version information\n  logout   - Log out of your account\n  whoami   - Show current user information".to_string();
                
                if user.is_admin {
                    help.push_str("\n\nAdmin commands:\n  users    - List all registered users\n  debug    - Show API debug information");
                }
                
                help
            } else {
                "Available commands:\n  help     - Show this help message\n  clear    - Clear the terminal\n  version  - Show version information\n  register - Create a new account\n  login    - Log in to your account".to_string()
            }
        },
        "version" => {
            "termstart v0.1.0".to_string()
        },
        "whoami" => {
            if user.is_authenticated {
                let mut info = format!("Logged in as: {}\nEmail: {}", user.username, user.email);
                if user.is_admin {
                    info.push_str("\nRole: Administrator");
                }
                info
            } else {
                "Not logged in".to_string()
            }
        },
        "users" => {
            if user.is_authenticated && user.is_admin {
                let user_clone = user.clone();
                
                spawn_local(async move {
                    match Request::get("http://localhost:8080/api/admin/users")
                        .header("Authorization", &format!("Bearer {}", user_clone.token))
                        .send()
                        .await
                    {
                        Ok(response) => {
                            if response.ok() {
                                match response.json::<Vec<User>>().await {
                                    Ok(users) => {
                                        let output = users.iter()
                                            .map(|u| format!("Username: {}\nEmail: {}\nAdmin: {}\n", u.username, u.email, u.is_admin))
                                            .collect::<Vec<String>>()
                                            .join("\n");
                                        // TODO: Display output in terminal
                                    }
                                    Err(_) => {}
                                }
                            }
                        }
                        Err(_) => {}
                    }
                });
                
                "Fetching user list...".to_string()
            } else {
                "Permission denied".to_string()
            }
        },
        "debug" => {
            if user.is_authenticated && user.is_admin {
                let user_clone = user.clone();
                let handle_output = handle_output.clone();
                
                spawn_local(async move {
                    match Request::get("http://localhost:8080/api/admin/debug")
                        .header("Authorization", &format!("dummy_token_{}", user_clone.email))
                        .send()
                        .await
                    {
                        Ok(response) => {
                            if response.ok() {
                                match response.text().await {
                                    Ok(debug_info) => {
                                        handle_output(format!("Debug Information:\n{}", debug_info));
                                    }
                                    Err(_) => {
                                        handle_output("Error: Failed to parse debug information".to_string());
                                    }
                                }
                            } else {
                                handle_output("Error: Failed to fetch debug information".to_string());
                            }
                        }
                        Err(_) => {
                            handle_output("Error: Failed to connect to server".to_string());
                        }
                    }
                });
                
                "Fetching API debug information...".to_string()
            } else {
                "Permission denied".to_string()
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
