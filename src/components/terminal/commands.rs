use wasm_bindgen_futures::spawn_local;
use termstart::services::auth::AuthService;
use termstart::config::Config;

pub fn handle_command(
    parts: Vec<&str>,
    handle_output: impl Fn(String) + 'static + Clone,
) -> String {
    let command = parts.join(" ");
    
    match parts.get(0) {
        Some(&"help") => {
            let is_logged_in = AuthService::get_current_user().is_some();
            let mut help_text = "Available commands:\n  help     - Show this help message\n  clear    - Clear the terminal\n  version  - Show version information\n".to_string();
            
            if !is_logged_in {
                help_text.push_str("  login    - Login to your account\n  register - Create a new account\n");
            } else {
                help_text.push_str("  logout   - Logout from your account\n  whoami   - Show current user information\n");
            }
            
            help_text
        },
        Some(&"version") => {
            "termstart v0.1.0".to_string()
        },
        Some(&"whoami") => {
            match AuthService::get_current_user() {
                Some(user) => format!("Logged in as: {}", user.email),
                None => "Not logged in".to_string(),
            }
        },
        Some(&"register") => {
            if AuthService::get_current_user().is_some() {
                "You are already logged in. Use 'logout' to sign out first.".to_string()
            } else if parts.len() < 3 {
                "Usage: register <email> <password>".to_string()
            } else {
                let email = parts[1].to_string();
                let password = parts[2].to_string();
                let handle_output = handle_output.clone();
                let command = command.clone();
                
                let config = Config::load();
                let auth_service = AuthService::new(
                    config.supabase_url.clone(),
                    config.supabase_key.clone(),
                );
                
                spawn_local(async move {
                    match auth_service.sign_up(&email, &password).await {
                        Ok(user) => {
                            handle_output(format!("{}", command));
                            handle_output(format!("Successfully registered and logged in as: {}", user.email));
                        }
                        Err(e) => {
                            handle_output(format!("{}", command));
                            handle_output(format!("Registration failed: {}", e));
                        }
                    }
                });
                
                "".to_string()
            }
        },
        Some(&"login") => {
            if AuthService::get_current_user().is_some() {
                "You are already logged in. Use 'logout' to sign out first.".to_string()
            } else if parts.len() < 3 {
                "Usage: login <email> <password>".to_string()
            } else {
                let email = parts[1].to_string();
                let password = parts[2].to_string();
                let handle_output = handle_output.clone();
                let command = command.clone();
                
                let config = Config::load();
                let auth_service = AuthService::new(
                    config.supabase_url.clone(),
                    config.supabase_key.clone(),
                );
                
                spawn_local(async move {
                    match auth_service.sign_in(&email, &password).await {
                        Ok(user) => {
                            handle_output(format!("{}", command));
                            handle_output(format!("Successfully logged in as: {}", user.email));
                        }
                        Err(e) => {
                            handle_output(format!("{}", command));
                            handle_output(format!("Login failed: {}", e));
                        }
                    }
                });
                
                "".to_string()
            }
        },
        Some(&"logout") => {
            if AuthService::get_current_user().is_none() {
                "You are not logged in.".to_string()
            } else {
                let handle_output = handle_output.clone();
                let command = command.clone();
                
                let config = Config::load();
                let auth_service = AuthService::new(
                    config.supabase_url.clone(),
                    config.supabase_key.clone(),
                );
                
                spawn_local(async move {
                    match auth_service.sign_out().await {
                        Ok(_) => {
                            handle_output(format!("{}", command));
                            handle_output("Successfully logged out".to_string());
                        }
                        Err(e) => {
                            handle_output(format!("{}", command));
                            handle_output(format!("Logout failed: {}", e));
                        }
                    }
                });
                
                "".to_string()
            }
        },
        Some(_) | None => {
            "Unknown command. Type 'help' for available commands.".to_string()
        }
    }
}
