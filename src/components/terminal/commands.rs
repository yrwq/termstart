use wasm_bindgen_futures::spawn_local;
use termstart::services::auth::AuthService;
use termstart::config::Config;
use web_sys::{window, Navigator, Location};

pub fn handle_command(
    parts: Vec<&str>,
    handle_output: impl Fn(String) + 'static + Clone,
) -> String {
    let command = parts.join(" ");
    
    match parts.get(0) {
        Some(&"help") => {
            let is_logged_in = AuthService::get_current_user().is_some();
            let mut help_text = "Available commands:\n  help     - Show this help message\n  clear    - Clear the terminal\n  version  - Show version information\n  theme    - Toggle between light and dark theme\n  fetch    - Display system information\n".to_string();
            
            if !is_logged_in {
                help_text.push_str("  login    - Login to your account\n  register - Create a new account\n");
            } else {
                help_text.push_str("  logout   - Logout from your account\n  whoami   - Show current user information\n");
            }
            
            help_text       
        },
        Some(&"fetch") => {
            let window = window().unwrap();
            let navigator = window.navigator();
            let user_agent = navigator.user_agent().unwrap_or_default();
            
            // Get browser info
            let browser = if user_agent.contains("Firefox") {
                "Firefox"
            } else if user_agent.contains("Chrome") {
                "Chrome"
            } else if user_agent.contains("Safari") {
                "Safari"
            } else {
                "Unknown Browser"
            };

            // Get OS info
            let os = if user_agent.contains("Mac") {
                "macOS"
            } else if user_agent.contains("Windows") {
                "Windows"
            } else if user_agent.contains("Linux") {
                "Linux"
            } else {
                "Unknown OS"
            };

            // Get screen resolution
            let screen_width = window.inner_width().unwrap().as_f64().unwrap_or(0.0) as i32;
            let screen_height = window.inner_height().unwrap().as_f64().unwrap_or(0.0) as i32;

            // Get color scheme
            let is_dark = window.document().unwrap()
                .document_element().unwrap()
                .class_list().contains("dark");
            let theme = if is_dark { "Dark" } else { "Light" };

            // Get host
            let host = window.location()
                .host()
                .unwrap_or_else(|_| "localhost".to_string());

            // Format the output
            format!(
                "\n{}:{} in {}\n{}\n\n\
                OS: {}\n\
                Browser: {}\n\
                Resolution: {}x{}\n\
                Theme: {}\n\
                User Agent: {}\n",
                if let Some(user) = AuthService::get_current_user() {
                    user.email
                } else {
                    "guest".to_string()
                },
                "termstart",
                host,
                "─".repeat(50),
                os,
                browser,
                screen_width,
                screen_height,
                theme,
                browser
            )
        },
        Some(&"theme") => {
            let window = window().unwrap();
            let document = window.document().unwrap();
            let html = document.document_element().unwrap();
            
            if html.class_list().contains("dark") {
                html.class_list().remove_1("dark").unwrap();
                "Switched to light theme".to_string()
            } else {
                html.class_list().add_1("dark").unwrap();
                "Switched to dark theme".to_string()
            }
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
