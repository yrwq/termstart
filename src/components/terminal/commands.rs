use termstart::services::auth::AuthService;
use termstart::services::bookmark::{BookmarkService};
use termstart::config::Config;
use web_sys::window;
use futures::Future;

pub fn handle_command(
    parts: Vec<&str>,
) -> std::pin::Pin<Box<dyn Future<Output = Result<String, String>> + 'static>> {
    let command = parts.join(" ");
    
    let parts_owned: Vec<String> = parts.iter().map(|s| s.to_string()).collect();
    let command_owned = command.clone();

    match parts.get(0) {
        Some(&"help") => Box::pin(async move { Ok(handle_sync_command(parts_owned.iter().map(|s| s.to_string()).collect(), &command_owned)) }),
        Some(&"fetch") => Box::pin(async move { Ok(handle_sync_command(parts_owned.iter().map(|s| s.to_string()).collect(), &command_owned)) }),
        Some(&"theme") => Box::pin(async move { Ok(handle_sync_command(parts_owned.iter().map(|s| s.to_string()).collect(), &command_owned)) }),
        Some(&"version") => Box::pin(async move { Ok(handle_sync_command(parts_owned.iter().map(|s| s.to_string()).collect(), &command_owned)) }),
        Some(&"whoami") => Box::pin(async move { Ok(handle_sync_command(parts_owned.iter().map(|s| s.to_string()).collect(), &command_owned)) }),
        Some(&"clear") => Box::pin(async move { Ok("".to_string()) }),
        Some(&"cd") => Box::pin(async move { Ok("".to_string()) }),
        Some(&"register") => Box::pin(handle_async_register(parts.into_iter().map(|s| s.to_string()).collect(), command)),
        Some(&"login") => Box::pin(handle_async_login(parts.into_iter().map(|s| s.to_string()).collect(), command)),
        Some(&"logout") => Box::pin(handle_async_logout(parts.into_iter().map(|s| s.to_string()).collect(), command)),
        Some(&"ls") => Box::pin(handle_async_ls(parts.into_iter().map(|s| s.to_string()).collect(), command)),
        Some(&"cat") => Box::pin(handle_async_cat(parts.into_iter().map(|s| s.to_string()).collect(), command)),
        Some(&"touch") => Box::pin(handle_async_touch(parts.into_iter().map(|s| s.to_string()).collect(), command)),
        Some(&"open") => Box::pin(handle_async_open(parts.into_iter().map(|s| s.to_string()).collect(), command)),
        Some(&"rm") => Box::pin(handle_async_rm(parts.into_iter().map(|s| s.to_string()).collect(), command)),
        Some(&"tag") => Box::pin(handle_async_tag(parts.into_iter().map(|s| s.to_string()).collect(), command)),
        Some(&"search") => Box::pin(handle_async_search(parts.into_iter().map(|s| s.to_string()).collect(), command)),
        Some(&"tree") => Box::pin(handle_async_tree(parts_owned, "".to_string())),
        Some(_) | None => Box::pin(async move { Ok("Unknown command. Type 'help' for available commands.".to_string()) }),
    }
}

fn handle_sync_command(
    parts: Vec<String>,
    _command_line: &str,
) -> String {
    match parts.get(0).map(|s| s.as_str()) {
        Some("help") => {
            let is_logged_in = AuthService::get_current_user().is_some();
            let mut help_text = "Available commands:\n  help     - Show this help message\n  clear    - Clear the terminal\n  version  - Show version information\n  theme    - Toggle between light and dark theme\n  fetch    - Display system information\n".to_string();
            
            if !is_logged_in {
                help_text.push_str("  login    - Login to your account\n  register - Create a new account\n");
            } else {
                help_text.push_str("  logout   - Logout from your account\n  whoami   - Show current user information\n  ls       - List your bookmarks\n  cat      - Show bookmark URL (usage: cat <bookmark_name>)\n  touch    - Create a bookmark (usage: touch <name> <url> [tags])\n  open     - Open bookmark in new tab (usage: open <bookmark_name>)\n  rm       - Remove a bookmark (usage: rm <bookmark_name>)\n  tag      - Add/remove tags (usage: tag <bookmark_name> <add|remove> <tag1> [tag2...])\n  search   - Search bookmarks (usage: search <query>)\n  tree     - Show a hierarchical view of bookmarks organized by tags\n");
            }
            
            help_text       
        },
        Some("fetch") => {
            let window = window().unwrap();
            let navigator = window.navigator();
            let user_agent = navigator.user_agent().unwrap_or_default();
            
            let browser = if user_agent.contains("Firefox") {
                "Firefox"
            } else if user_agent.contains("Chrome") {
                "Chrome"
            } else if user_agent.contains("Safari") {
                "Safari"
            } else {
                "Unknown Browser"
            };

            let os = if user_agent.contains("Mac") {
                "macOS"
            } else if user_agent.contains("Windows") {
                "Windows"
            } else if user_agent.contains("Linux") {
                "Linux"
            } else {
                "Unknown OS"
            };

            let screen_width = window.inner_width().unwrap().as_f64().unwrap_or(0.0) as i32;
            let screen_height = window.inner_height().unwrap().as_f64().unwrap_or(0.0) as i32;

            let is_dark = window.document().unwrap()
                .document_element().unwrap()
                .class_list().contains("dark");
            let theme = if is_dark { "Dark" } else { "Light" };

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
        Some("theme") => {
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
        Some("version") => {
            "termstart v0.1.0".to_string()
        },
        Some("whoami") => {
            match AuthService::get_current_user() {
                Some(user) => format!("Logged in as: {}", user.email),
                None => "Not logged in".to_string(),
            }
        },
        Some(_) | None => {
             "".to_string()
        }
    }
}

fn handle_async_ls(
    parts: Vec<String>,
    command_line: String,
) -> std::pin::Pin<Box<dyn Future<Output = Result<String, String>> + 'static>> {
    Box::pin(async move {
        if AuthService::get_current_user().is_none() {
            return Ok("You must be logged in to use this command.".to_string());
        }

        let tag_filter = parts.get(1).map(|s| s.to_string());
        let config = Config::load();
        let bookmark_service = BookmarkService::new(
            config.supabase_url.clone(),
            config.supabase_key.clone(),
        );
        
        match bookmark_service.get_bookmarks(None).await {
            Ok(bookmarks) => {
                if bookmarks.is_empty() {
                    return Ok("No bookmarks found.".to_string());
                }
                
                let mut output = String::new();
                
                if let Some(tag) = tag_filter {
                    let filtered_bookmarks: Vec<_> = bookmarks.iter()
                        .filter(|b| b.tags.contains(&tag))
                        .collect();
                    
                    if filtered_bookmarks.is_empty() {
                         return Ok(format!("No bookmarks found in tag '{}'", tag));
                    } else {
                        for bookmark in filtered_bookmarks {
                            let tags = if bookmark.tags.is_empty() {
                                String::new()
                            } else {
                                format!(" [{}]", bookmark.tags.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(", "))
                            };
                            output.push_str(&format!("BOOKMARK_ITEM:{}{}\n", bookmark.name.trim(), tags.trim()));
                        }
                         return Ok(output);
                    }
                }

                let mut tags: std::collections::HashSet<String> = std::collections::HashSet::new();
                let mut untagged_bookmarks = Vec::new();
                
                for bookmark in &bookmarks {
                    if bookmark.tags.is_empty() {
                        untagged_bookmarks.push(bookmark);
                    } else {
                        tags.extend(bookmark.tags.iter().cloned());
                    }
                }
                
                let mut sorted_tags: Vec<_> = tags.into_iter().collect();
                sorted_tags.sort();
                
                let mut output = String::new();
                
                if !sorted_tags.is_empty() {
                    for tag in sorted_tags {
                        output.push_str(&format!("TAG_ITEM:{}\n", tag.trim()));
                    }
                }
                
                if !untagged_bookmarks.is_empty() {
                    for bookmark in untagged_bookmarks {
                         let tags = if bookmark.tags.is_empty() {
                            String::new()
                        } else {
                            format!(" [{}]", bookmark.tags.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(", "))
                        };
                        output.push_str(&format!("BOOKMARK_ITEM:{}{}\n", bookmark.name.trim(), tags.trim()));
                    }
                }

                 Ok(output)
            }
            Err(e) => {
                Err(format!("Failed to list bookmarks: {}", e))
            }
        }
    })
}

fn handle_async_register(
    parts: Vec<String>,
    command_line: String,
) -> std::pin::Pin<Box<dyn Future<Output = Result<String, String>> + 'static>> {
    Box::pin(async move {
        if AuthService::get_current_user().is_some() {
            return Ok("You are already logged in. Use 'logout' to sign out first.".to_string());
        } else if parts.len() < 3 {
            return Ok("Usage: register <email> <password>".to_string());
        } else {
            let email = parts[1].to_string();
            let password = parts[2].to_string();
            
            let config = Config::load();
            let auth_service = AuthService::new(
                config.supabase_url.clone(),
                config.supabase_key.clone(),
            );
            
            match auth_service.sign_up(&email, &password).await {
                Ok(user) => {
                    Ok(format!("Successfully registered and logged in as: {}", user.email))
                }
                Err(e) => {
                    Err(format!("Registration failed: {}", e))
                }
            }
        }
    })
}

fn handle_async_login(
    parts: Vec<String>,
    command_line: String,
) -> std::pin::Pin<Box<dyn Future<Output = Result<String, String>> + 'static>> {
    Box::pin(async move {
        if AuthService::get_current_user().is_some() {
            return Ok("You are already logged in. Use 'logout' to sign out first.".to_string());
        } else if parts.len() < 3 {
            return Ok("Usage: login <email> <password>".to_string());
        } else {
            let email = parts[1].to_string();
            let password = parts[2].to_string();
            
            let config = Config::load();
            let auth_service = AuthService::new(
                config.supabase_url.clone(),
                config.supabase_key.clone(),
            );
            
            match auth_service.sign_in(&email, &password).await {
                Ok(user) => {
                    Ok(format!("Successfully logged in as: {}", user.email))
                }
                Err(e) => {
                    Err(format!("Login failed: {}", e))
                }
            }
        }
    })
}

fn handle_async_logout(
    parts: Vec<String>,
    command_line: String,
) -> std::pin::Pin<Box<dyn Future<Output = Result<String, String>> + 'static>> {
    Box::pin(async move {
        if AuthService::get_current_user().is_none() {
            return Ok("You are not logged in.".to_string());
        } else {
            let config = Config::load();
            let auth_service = AuthService::new(
                config.supabase_url.clone(),
                config.supabase_key.clone(),
            );

            match auth_service.sign_out().await {
                Ok(_) => {
                    Ok("Successfully logged out".to_string())
                }
                Err(e) => {
                    Err(format!("Logout failed: {}", e))
                }
            }
        }
    })
}

fn handle_async_cat(
    parts: Vec<String>,
    command_line: String,
) -> std::pin::Pin<Box<dyn Future<Output = Result<String, String>> + 'static>> {
    Box::pin(async move {
        if AuthService::get_current_user().is_none() {
            return Ok("You must be logged in to use this command.".to_string());
        } else if parts.len() < 2 {
            return Ok("Usage: cat <bookmark_name>".to_string());
        } else {
            let name = parts[1].to_string();

            let config = Config::load();
            let bookmark_service = BookmarkService::new(
                config.supabase_url.clone(),
                config.supabase_key.clone(),
            );

            match bookmark_service.get_bookmark_by_name(&name).await {
                Ok(Some(bookmark)) => {
                    let tags = if bookmark.tags.is_empty() {
                        String::new()
                    } else {
                        format!("\nTags: {}", bookmark.tags.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(", "))
                    };
                    Ok(format!("URL: {}{}", bookmark.url, tags))
                }
                Ok(None) => {
                    Ok(format!("Bookmark '{}' not found.", name))
                }
                Err(e) => {
                    Err(format!("Failed to get bookmark: {}", e))
                }
            }
        }
    })
}

fn handle_async_touch(
    parts: Vec<String>,
    command_line: String,
) -> std::pin::Pin<Box<dyn Future<Output = Result<String, String>> + 'static>> {
    Box::pin(async move {
        if AuthService::get_current_user().is_none() {
            return Ok("You must be logged in to use this command.".to_string());
        } else if parts.len() < 3 {
            return Ok("Usage: touch <name> <url> [tags]".to_string());
        } else {
            let name = parts[1].to_string();
            let url = parts[2].to_string();
            let tags = if parts.len() > 3 {
                Some(parts[3..].iter().map(|s| s.to_string()).collect())
            } else {
                None
            };

            let config = Config::load();
            let bookmark_service = BookmarkService::new(
                config.supabase_url.clone(),
                config.supabase_key.clone(),
            );

            match bookmark_service.create_bookmark(&name, &url, tags).await {
                Ok(bookmark) => {
                    let tags = if bookmark.tags.is_empty() {
                        String::new()
                    } else {
                        format!(" with tags: {}", bookmark.tags.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(", "))
                    };
                    Ok(format!("Created bookmark '{}'{}", bookmark.name, tags))
                }
                Err(e) => {
                    Err(format!("Failed to create bookmark: {}", e))
                }
            }
        }
    })
}

fn handle_async_open(
    parts: Vec<String>,
    command_line: String,
) -> std::pin::Pin<Box<dyn Future<Output = Result<String, String>> + 'static>> {
    Box::pin(async move {
        if AuthService::get_current_user().is_none() {
            return Ok("You must be logged in to use this command.".to_string());
        } else if parts.len() < 2 {
            return Ok("Usage: open <bookmark_name>".to_string());
        } else {
            let name = parts[1].to_string();

            let config = Config::load();
            let bookmark_service = BookmarkService::new(
                config.supabase_url.clone(),
                config.supabase_key.clone(),
            );

            match bookmark_service.get_bookmark_by_name(&name).await {
                Ok(Some(bookmark)) => {
                    if let Some(window) = web_sys::window() {
                        if let Err(e) = window.open_with_url_and_target(&bookmark.url, "_blank") {
                            Err(format!("Failed to open URL: {:?}", e))
                        } else {
                            Ok(format!("Opening {} in new tab...", bookmark.url))
                        }
                    } else {
                        Err("Failed to open URL: Could not access window".to_string())
                    }
                }
                Ok(None) => {
                    Ok(format!("Bookmark '{}' not found.", name))
                }
                Err(e) => {
                    Err(format!("Failed to get bookmark: {}", e))
                }
            }
        }
    })
}

fn handle_async_rm(
    parts: Vec<String>,
    command_line: String,
) -> std::pin::Pin<Box<dyn Future<Output = Result<String, String>> + 'static>> {
    Box::pin(async move {
        if AuthService::get_current_user().is_none() {
            return Ok("You must be logged in to use this command.".to_string());
        } else if parts.len() < 2 {
            return Ok("Usage: rm <bookmark_name>".to_string());
        } else {
            let name = parts[1].to_string();

            let config = Config::load();
            let bookmark_service = BookmarkService::new(
                config.supabase_url.clone(),
                config.supabase_key.clone(),
            );

            match bookmark_service.delete_bookmark(&name).await {
                Ok(_) => {
                    Ok(format!("Deleted bookmark '{}'", name))
                }
                Err(e) => {
                    Err(format!("Failed to delete bookmark: {}", e))
                }
            }
        }
    })
}

fn handle_async_search(
    parts: Vec<String>,
    command_line: String,
) -> std::pin::Pin<Box<dyn Future<Output = Result<String, String>> + 'static>> {
    Box::pin(async move {
        if AuthService::get_current_user().is_none() {
            return Ok("You must be logged in to use this command.".to_string());
        } else if parts.len() < 2 {
            return Ok("Usage: search <query>".to_string());
        } else {
            let query = parts[1..].join(" ");

            let config = Config::load();
            let bookmark_service = BookmarkService::new(
                config.supabase_url.clone(),
                config.supabase_key.clone(),
            );

            match bookmark_service.search_bookmarks(&query).await {
                Ok(bookmarks) => {
                    if bookmarks.is_empty() {
                        Ok(format!("No bookmarks found matching '{}'", query))
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
                        Ok(output)
                    }
                }
                Err(e) => {
                    Err(format!("Failed to search bookmarks: {}", e))
                }
            }
        }
    })
}

fn handle_async_tag(
    parts: Vec<String>,
    command_line: String,
) -> std::pin::Pin<Box<dyn Future<Output = Result<String, String>> + 'static>> {
    Box::pin(async move {
        if AuthService::get_current_user().is_none() {
            return Ok("You must be logged in to use this command.".to_string());
        }

        if parts.len() < 4 {
             return Ok("Usage: tag <bookmark_name> <add|remove> <tag1> [tag2...]".to_string());
        }

        let name = parts[1].to_string();
        let action = parts[2].to_string();
        let tags: Vec<String> = parts[3..].iter().map(|s| s.to_string()).collect();

        let config = Config::load();
        let bookmark_service = BookmarkService::new(
            config.supabase_url.clone(),
            config.supabase_key.clone(),
        );

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
                         return Ok(format!("Invalid action. Use 'add' or 'remove'."));
                    }
                }

                match bookmark_service.update_bookmark(&name, None, Some(current_tags)).await {
                    Ok(bookmark) => {
                        let tags = if bookmark.tags.is_empty() {
                            String::new()
                        } else {
                            format!(" with tags: {}", bookmark.tags.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(", "))
                        };
                         Ok(format!("Updated bookmark '{}'{}", bookmark.name, tags))
                    }
                    Err(e) => {
                        Err(format!("Failed to update bookmark: {}", e))
                    }
                }
            }
            Ok(None) => {
                 Ok(format!("Bookmark '{}' not found.", name))
            }
            Err(e) => {
                Err(format!("Failed to get bookmark: {}", e))
            }
        }
    })
}

async fn handle_async_tree(parts: Vec<String>, _command_line: String) -> Result<String, String> {
    if AuthService::get_current_user().is_none() {
        return Ok("You must be logged in to use this command.".to_string());
    }
    let config = Config::load();
    let bookmark_service = BookmarkService::new(
        config.supabase_url.clone(),
        config.supabase_key.clone(),
    );
    match bookmark_service.get_bookmarks(None).await {
        Ok(bookmarks) => {
            use std::collections::{BTreeMap, BTreeSet};
            let mut tag_map: BTreeMap<String, Vec<&termstart::services::bookmark::Bookmark>> = BTreeMap::new();
            let mut untagged = Vec::new();
            let mut all_tags = BTreeSet::new();
            for bookmark in &bookmarks {
                if bookmark.tags.is_empty() {
                    untagged.push(bookmark);
                } else {
                    for tag in &bookmark.tags {
                        tag_map.entry(tag.clone()).or_default().push(bookmark);
                        all_tags.insert(tag.clone());
                    }
                }
            }
            let mut output = String::new();
            let mut tag_list: Vec<_> = all_tags.into_iter().collect();
            tag_list.sort();
            for tag in &tag_list {
                output.push_str(&format!("TAG_ITEM:{}\n", tag));
                if let Some(bookmarks) = tag_map.get(tag) {
                    for (i, bookmark) in bookmarks.iter().enumerate() {
                        let prefix = if i == bookmarks.len() - 1 { "└──" } else { "├──" };
                        output.push_str(&format!("BOOKMARK_ITEM: {} {}\n", prefix, bookmark.name));
                    }
                }
            }
            if !untagged.is_empty() {
                output.push_str("TAG_ITEM:untagged\n");
                for (i, bookmark) in untagged.iter().enumerate() {
                    let prefix = if i == untagged.len() - 1 { "└──" } else { "├──" };
                    output.push_str(&format!("BOOKMARK_ITEM: {} {}\n", prefix, bookmark.name));
                }
            }
            Ok(output)
        }
        Err(e) => Err(format!("Failed to get bookmarks: {}", e))
    }
}