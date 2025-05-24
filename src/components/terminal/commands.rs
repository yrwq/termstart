use wasm_bindgen_futures::spawn_local;
use termstart::services::auth::AuthService;
use termstart::services::bookmark::{BookmarkService};
use termstart::config::Config;
use web_sys::window;

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
                help_text.push_str("  logout   - Logout from your account\n  whoami   - Show current user information\n  ls       - List your bookmarks\n  cat      - Show bookmark URL (usage: cat <bookmark_name>)\n  touch    - Create a bookmark (usage: touch <name> <url> [tags])\n  open     - Open bookmark in new tab (usage: open <bookmark_name>)\n  rm       - Remove a bookmark (usage: rm <bookmark_name>)\n  tag      - Add/remove tags (usage: tag <bookmark_name> <add|remove> <tag1> [tag2...])\n  search   - Search bookmarks (usage: search <query>)\n  tree     - Show a hierarchical view of bookmarks organized by tags\n");
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

            // Format the output
            format!(
                "{}\n\
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
        Some(&"ls") => {
            if AuthService::get_current_user().is_none() {
                "You must be logged in to use this command.".to_string()
            } else {
                let handle_output = handle_output.clone();
                let command = command.clone();
                let tag_filter = parts.get(1).map(|&s| s.to_string());
                
                let config = Config::load();
                let bookmark_service = BookmarkService::new(
                    config.supabase_url.clone(),
                    config.supabase_key.clone(),
                );
                
                spawn_local(async move {
                    match bookmark_service.get_bookmarks(None).await {
                        Ok(bookmarks) => {
                            if bookmarks.is_empty() {
                                handle_output(format!("{}\nNo bookmarks found.", command));
                            } else {
                                let mut output = String::new();
                                
                                if let Some(tag) = tag_filter {
                                    // Show bookmarks in the specified tag
                                    let filtered_bookmarks: Vec<_> = bookmarks.iter()
                                        .filter(|b| b.tags.contains(&tag))
                                        .collect();
                                    
                                    if filtered_bookmarks.is_empty() {
                                        handle_output(format!("{}\nNo bookmarks found in tag '{}'", command, tag));
                                    } else {
                                        for bookmark in filtered_bookmarks {
                                            let tags = if bookmark.tags.is_empty() {
                                                String::new()
                                            } else {
                                                format!(" [{}]", bookmark.tags.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(", "))
                                            };
                                            output.push_str(&format!("BOOKMARK_ITEM:{}{}\n", bookmark.name, tags));
                                        }
                                        handle_output(output);
                                    }
                                } else {
                                    // Show all tags as folders and bookmarks without tags
                                    let mut tags: std::collections::HashSet<String> = std::collections::HashSet::new();
                                    let mut untagged_bookmarks = Vec::new();
                                    
                                    for bookmark in &bookmarks {
                                        if bookmark.tags.is_empty() {
                                            untagged_bookmarks.push(bookmark);
                                        } else {
                                            tags.extend(bookmark.tags.iter().cloned());
                                        }
                                    }
                                    
                                    // Sort tags alphabetically
                                    let mut sorted_tags: Vec<_> = tags.into_iter().collect();
                                    sorted_tags.sort();
                                    
                                    // Display tags as folders
                                    if !sorted_tags.is_empty() {
                                        for tag in sorted_tags {
                                            output.push_str(&format!("TAG_ITEM:{}\n", tag));
                                        }
                                    }
                                    
                                    // Display untagged bookmarks
                                    if !untagged_bookmarks.is_empty() {
                                        for bookmark in untagged_bookmarks {
                                            let tags = if bookmark.tags.is_empty() {
                                                String::new()
                                            } else {
                                                format!(" [{}]", bookmark.tags.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(", "))
                                            };
                                            output.push_str(&format!("BOOKMARK_ITEM:{}{}\n", bookmark.name, tags));
                                        }
                                    }
                                    
                                    handle_output(output);
                                }
                            }
                        }
                        Err(e) => {
                            handle_output(format!("{}\nFailed to list bookmarks: {}", command, e));
                        }
                    }
                });
                
                "".to_string()
            }
        },
        Some(&"cat") => {
            if AuthService::get_current_user().is_none() {
                "You must be logged in to use this command.".to_string()
            } else if parts.len() < 2 {
                "Usage: cat <bookmark_name>".to_string()
            } else {
                let name = parts[1].to_string();
                let handle_output = handle_output.clone();
                let command = command.clone();
                
                let config = Config::load();
                let bookmark_service = BookmarkService::new(
                    config.supabase_url.clone(),
                    config.supabase_key.clone(),
                );
                
                spawn_local(async move {
                    match bookmark_service.get_bookmark_by_name(&name).await {
                        Ok(Some(bookmark)) => {
                            let tags = if bookmark.tags.is_empty() {
                                String::new()
                            } else {
                                format!("\nTags: {}", bookmark.tags.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(", "))
                            };
                            handle_output(format!("{}\nURL: {}{}", command, bookmark.url, tags));
                        }
                        Ok(None) => {
                            handle_output(format!("{}\nBookmark '{}' not found.", command, name));
                        }
                        Err(e) => {
                            handle_output(format!("{}\nFailed to get bookmark: {}", command, e));
                        }
                    }
                });
                
                "".to_string()
            }
        },
        Some(&"touch") => {
            if AuthService::get_current_user().is_none() {
                "You must be logged in to use this command.".to_string()
            } else if parts.len() < 3 {
                "Usage: touch <name> <url> [tags]".to_string()
            } else {
                let name = parts[1].to_string();
                let url = parts[2].to_string();
                let tags = if parts.len() > 3 {
                    Some(parts[3..].iter().map(|&s| s.to_string()).collect())
                } else {
                    None
                };
                
                let handle_output = handle_output.clone();
                let command = command.clone();
                
                let config = Config::load();
                let bookmark_service = BookmarkService::new(
                    config.supabase_url.clone(),
                    config.supabase_key.clone(),
                );
                
                spawn_local(async move {
                    match bookmark_service.create_bookmark(&name, &url, tags).await {
                        Ok(bookmark) => {
                            let tags = if bookmark.tags.is_empty() {
                                String::new()
                            } else {
                                format!(" with tags: {}", bookmark.tags.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(", "))
                            };
                            handle_output(format!("{}\nCreated bookmark '{}'{}", command, bookmark.name, tags));
                        }
                        Err(e) => {
                            handle_output(format!("{}\nFailed to create bookmark: {}", command, e));
                        }
                    }
                });
                
                "".to_string()
            }
        },
        Some(&"open") => {
            if AuthService::get_current_user().is_none() {
                "You must be logged in to use this command.".to_string()
            } else if parts.len() < 2 {
                "Usage: open <bookmark_name>".to_string()
            } else {
                let name = parts[1].to_string();
                let handle_output = handle_output.clone();
                let command = command.clone();
                
                let config = Config::load();
                let bookmark_service = BookmarkService::new(
                    config.supabase_url.clone(),
                    config.supabase_key.clone(),
                );
                
                spawn_local(async move {
                    match bookmark_service.get_bookmark_by_name(&name).await {
                        Ok(Some(bookmark)) => {
                            if let Some(window) = web_sys::window() {
                                if let Err(e) = window.open_with_url_and_target(&bookmark.url, "_blank") {
                                    handle_output(format!("{}\nFailed to open URL: {:?}", command, e));
                                } else {
                                    handle_output(format!("{}\nOpening {} in new tab...", command, bookmark.url));
                                }
                            } else {
                                handle_output(format!("{}\nFailed to open URL: Could not access window", command));
                            }
                        }
                        Ok(None) => {
                            handle_output(format!("{}\nBookmark '{}' not found.", command, name));
                        }
                        Err(e) => {
                            handle_output(format!("{}\nFailed to get bookmark: {}", command, e));
                        }
                    }
                });
                
                "".to_string()
            }
        },
        Some(&"rm") => {
            if AuthService::get_current_user().is_none() {
                "You must be logged in to use this command.".to_string()
            } else if parts.len() < 2 {
                "Usage: rm <bookmark_name>".to_string()
            } else {
                let name = parts[1].to_string();
                let handle_output = handle_output.clone();
                let command = command.clone();
                
                let config = Config::load();
                let bookmark_service = BookmarkService::new(
                    config.supabase_url.clone(),
                    config.supabase_key.clone(),
                );
                
                spawn_local(async move {
                    match bookmark_service.delete_bookmark(&name).await {
                        Ok(_) => {
                            handle_output(format!("{}\nDeleted bookmark '{}'", command, name));
                        }
                        Err(e) => {
                            handle_output(format!("{}\nFailed to delete bookmark: {}", command, e));
                        }
                    }
                });
                
                "".to_string()
            }
        },
        Some(&"tag") => {
            if AuthService::get_current_user().is_none() {
                "You must be logged in to use this command.".to_string()
            } else if parts.len() < 4 {
                "Usage: tag <bookmark_name> <add|remove> <tag1> [tag2...]".to_string()
            } else {
                let name = parts[1].to_string();
                let action = parts[2].to_string();
                let tags: Vec<String> = parts[3..].iter().map(|&s| s.to_string()).collect();
                
                let handle_output = handle_output.clone();
                let command = command.clone();
                
                let config = Config::load();
                let bookmark_service = BookmarkService::new(
                    config.supabase_url.clone(),
                    config.supabase_key.clone(),
                );
                
                spawn_local(async move {
                    // First get the existing bookmark
                    match bookmark_service.get_bookmark_by_name(&name).await {
                        Ok(Some(existing)) => {
                            let mut current_tags: Vec<String> = existing.tags.into_iter().collect();
                            
                            match action.as_str() {
                                "add" => {
                                    current_tags.extend(tags);
                                    current_tags.sort();
                                    current_tags.dedup();
                                }
                                "remove" => {
                                    current_tags.retain(|t| !tags.contains(t));
                                }
                                _ => {
                                    handle_output(format!("{}\nInvalid action. Use 'add' or 'remove'.", command));
                                    return;
                                }
                            }
                            
                            match bookmark_service.update_bookmark(&name, None, Some(current_tags)).await {
                                Ok(bookmark) => {
                                    let tags = if bookmark.tags.is_empty() {
                                        String::new()
                                    } else {
                                        format!(" with tags: {}", bookmark.tags.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(", "))
                                    };
                                    handle_output(format!("{}\nUpdated bookmark '{}'{}", command, bookmark.name, tags));
                                }
                                Err(e) => {
                                    handle_output(format!("{}\nFailed to update bookmark: {}", command, e));
                                }
                            }
                        }
                        Ok(None) => {
                            handle_output(format!("{}\nBookmark '{}' not found.", command, name));
                        }
                        Err(e) => {
                            handle_output(format!("{}\nFailed to get bookmark: {}", command, e));
                        }
                    }
                });
                
                "".to_string()
            }
        },
        Some(&"search") => {
            if AuthService::get_current_user().is_none() {
                "You must be logged in to use this command.".to_string()
            } else if parts.len() < 2 {
                "Usage: search <query>".to_string()
            } else {
                let query = parts[1..].join(" ");
                let handle_output = handle_output.clone();
                let command = command.clone();
                
                let config = Config::load();
                let bookmark_service = BookmarkService::new(
                    config.supabase_url.clone(),
                    config.supabase_key.clone(),
                );
                
                spawn_local(async move {
                    match bookmark_service.search_bookmarks(&query).await {
                        Ok(bookmarks) => {
                            if bookmarks.is_empty() {
                                handle_output(format!("{}\nNo bookmarks found matching '{}'", command, query));
                            } else {
                                let mut output = String::new();
                                for bookmark in bookmarks {
                                    let tags = if bookmark.tags.is_empty() {
                                        String::new()
                                    } else {
                                        format!(" [{}]", bookmark.tags.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(", "))
                                    };
                                    output.push_str(&format!("{} - {}{}\n", bookmark.name, bookmark.url, tags));
                                }
                                handle_output(format!("{}\n{}", command, output));
                            }
                        }
                        Err(e) => {
                            handle_output(format!("{}\nFailed to search bookmarks: {}", command, e));
                        }
                    }
                });
                
                "".to_string()
            }
        },
        Some(&"tree") => {
            if AuthService::get_current_user().is_none() {
                "You must be logged in to use this command.".to_string()
            } else {
                let handle_output = handle_output.clone();
                
                let config = Config::load();
                let bookmark_service = BookmarkService::new(
                    config.supabase_url.clone(),
                    config.supabase_key.clone(),
                );
                
                spawn_local(async move {
                    match bookmark_service.get_bookmarks(None).await {
                        Ok(bookmarks) => {
                            if bookmarks.is_empty() {
                                handle_output("No bookmarks found.".to_string());
                            } else {
                                let mut output = String::new();
                                
                                // Group bookmarks by tags
                                let mut tag_groups: std::collections::HashMap<String, Vec<&termstart::services::bookmark::Bookmark>> = std::collections::HashMap::new();
                                let mut untagged_bookmarks = Vec::new();
                                
                                for bookmark in &bookmarks {
                                    if bookmark.tags.is_empty() {
                                        untagged_bookmarks.push(bookmark);
                                    } else {
                                        for tag in &bookmark.tags {
                                            tag_groups.entry(tag.clone())
                                                .or_insert_with(Vec::new)
                                                .push(bookmark);
                                        }
                                    }
                                }
                                
                                // Sort tags alphabetically
                                let mut sorted_tags: Vec<_> = tag_groups.keys().cloned().collect();
                                sorted_tags.sort();
                                
                                // Build tree structure
                                output.push_str(".\n");
                                
                                // Add untagged bookmarks at root level
                                if !untagged_bookmarks.is_empty() {
                                    for bookmark in untagged_bookmarks {
                                        output.push_str(&format!("├── BOOKMARK_ITEM:{}\n", bookmark.name));
                                    }
                                }
                                
                                // Add tagged bookmarks under their tags
                                for (i, tag) in sorted_tags.iter().enumerate() {
                                    let is_last = i == sorted_tags.len() - 1;
                                    let prefix = if is_last { "└── " } else { "├── " };
                                    output.push_str(&format!("{}TAG_ITEM:{}\n", prefix, tag));
                                    
                                    let bookmarks = &tag_groups[tag];
                                    for (j, bookmark) in bookmarks.iter().enumerate() {
                                        let is_last_bookmark = j == bookmarks.len() - 1;
                                        let sub_prefix = if is_last { "    " } else { "│   " };
                                        let bookmark_prefix = if is_last_bookmark { "└── " } else { "├── " };
                                        output.push_str(&format!("{}{}BOOKMARK_ITEM:{}\n", sub_prefix, bookmark_prefix, bookmark.name));
                                    }
                                }
                                
                                handle_output(output);
                            }
                        }
                        Err(e) => {
                            handle_output(format!("Failed to list bookmarks: {}", e));
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
