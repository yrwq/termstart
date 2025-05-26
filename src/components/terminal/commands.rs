use termstart::services::auth::AuthService;
use termstart::services::bookmark::{BookmarkService};
use termstart::config::Config;
use web_sys::window;
use futures::Future;
use wasm_bindgen::JsValue;
use serde_wasm_bindgen;

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
            let mut help_text = String::new();
            
            // Header
            help_text.push_str("╭──────────────────────────╮\n");
            help_text.push_str("│      termstart v0.1.0    │\n");
            help_text.push_str("├──────────────────────────┤\n");
            
            // Basic commands
            help_text.push_str("│  Basic:                  │\n");
            help_text.push_str("│  • help    - help menu   │\n");
            help_text.push_str("│  • clear   - clear       │\n");
            help_text.push_str("│  • version - version     │\n");
            help_text.push_str("│  • theme   - theme       │\n");
            help_text.push_str("│  • fetch   - system info │\n");
            
            if !is_logged_in {
                // Auth commands
                help_text.push_str("├──────────────────────────┤\n");
                help_text.push_str("│  Auth:                   │\n");
                help_text.push_str("│  • login   - sign in     │\n");
                help_text.push_str("│  • register - sign up    │\n");
            } else {
                // Bookmark commands
                help_text.push_str("├──────────────────────────┤\n");
                help_text.push_str("│  Bookmarks:              │\n");
                help_text.push_str("│  • ls     - list         │\n");
                help_text.push_str("│  • cat    - show url     │\n");
                help_text.push_str("│  • touch  - create       │\n");
                help_text.push_str("│  • open   - open tab     │\n");
                help_text.push_str("│  • rm     - remove       │\n");
                help_text.push_str("│  • tag    - tags         │\n");
                help_text.push_str("│  • search - search       │\n");
                help_text.push_str("│  • tree   - tree view    │\n");
                help_text.push_str("│  • logout - sign out     │\n");
                help_text.push_str("│  • whoami - user info    │\n");
            }
            
            help_text.push_str("╰──────────────────────────╯\n");
            
            help_text
        },
        Some("fetch") => {
            let window = window().unwrap();
            let navigator = window.navigator();
            let user_agent = navigator.user_agent().unwrap_or_default();
            
            let browser = if user_agent.contains("Firefox") {
                "firefox"
            } else if user_agent.contains("Chrome") {
                "chrome"
            } else if user_agent.contains("Safari") {
                "safari"
            } else {
                "unknown"
            };

            let os = if user_agent.contains("Mac") {
                "macos"
            } else if user_agent.contains("Windows") {
                "windows"
            } else if user_agent.contains("Linux") {
                "linux"
            } else {
                "unknown"
            };

            let screen_width = window.inner_width().unwrap().as_f64().unwrap_or(0.0) as i32;
            let screen_height = window.inner_height().unwrap().as_f64().unwrap_or(0.0) as i32;

            let is_dark = window.document().unwrap()
                .document_element().unwrap()
                .class_list().contains("dark");
            let theme = if is_dark { "dark" } else { "light" };

            format!(
                "{}\n\
                os: {}\n\
                browser: {}\n\
                resolution: {}x{}\n\
                theme: {}\n\
                user agent: {}\n",
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
                "switched to light theme".to_string()
            } else {
                html.class_list().add_1("dark").unwrap();
                "switched to dark theme".to_string()
            }
        },
        Some("version") => {
            "termstart v0.1.0".to_string()
        },
        Some("whoami") => {
            match AuthService::get_current_user() {
                Some(user) => format!("logged in as: {}", user.email),
                None => "not logged in".to_string(),
            }
        },
        Some(_) | None => {
             "".to_string()
        }
    }
}

fn handle_async_ls(
    parts: Vec<String>,
    _command_line: String,
) -> std::pin::Pin<Box<dyn Future<Output = Result<String, String>> + 'static>> {
    Box::pin(async move {
        if AuthService::get_current_user().is_none() {
            return Ok("you must be logged in to use this command.".to_string());
        }

        let tag_filter = parts.get(1).map(|s| s.to_string());

        // Get the cached bookmarks from the window object
        let window = web_sys::window().unwrap();
        let cache = js_sys::Reflect::get(&window, &JsValue::from_str("__bookmark_cache"))
            .ok()
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0);

        let current_time = js_sys::Date::now();
        let cache_age = current_time - cache;

        let bookmarks = if cache_age > 300000.0 { // 5 minutes in milliseconds
            // Cache is old or missing, fetch fresh data
            let config = Config::load();
            let bookmark_service = BookmarkService::new(
                config.supabase_url.clone(),
                config.supabase_key.clone(),
            );

            match bookmark_service.get_bookmarks(None).await {
                Ok(fetched_bookmarks) => {
                    // Update cache timestamp
                    js_sys::Reflect::set(&window, &JsValue::from_str("__bookmark_cache"), &JsValue::from_f64(current_time))
                        .unwrap_or_default();

                    // Store bookmarks in cache
                    js_sys::Reflect::set(&window, &JsValue::from_str("__bookmarks"), &serde_wasm_bindgen::to_value(&fetched_bookmarks).unwrap())
                        .unwrap_or_default();

                    fetched_bookmarks
                }
                Err(e) => return Err(format!("failed to get bookmarks: {}", e)),
            }
        } else {
            // Use cached bookmarks
            js_sys::Reflect::get(&window, &JsValue::from_str("__bookmarks"))
                .ok()
                .and_then(|v| serde_wasm_bindgen::from_value(v).ok())
                .unwrap_or_default()
        };

        if bookmarks.is_empty() {
            return Ok("no bookmarks found.".to_string());
        }

        let mut output = String::new();

        if let Some(tag) = tag_filter {
            let filtered_bookmarks: Vec<_> = bookmarks.iter()
                .filter(|b| b.tags.contains(&tag))
                .collect();

            if filtered_bookmarks.is_empty() {
                 return Ok(format!("no bookmarks found in tag '{}'", tag));
            } else {
                for bookmark in filtered_bookmarks {
                    let tags = if bookmark.tags.is_empty() {
                        String::new()
                    } else {
                        format!(" [{}]", bookmark.tags.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(", "))
                    };
                    output.push_str(&format!("BOOKMARK_ITEM:{}{}
", bookmark.name.trim(), tags.trim()));
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
                output.push_str(&format!("TAG_ITEM:{}
", tag.trim()));
            }
        }

        if !untagged_bookmarks.is_empty() {
            for bookmark in untagged_bookmarks {
                 let tags = if bookmark.tags.is_empty() {
                    String::new()
                } else {
                    format!(" [{}]", bookmark.tags.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(", "))
                };
                output.push_str(&format!("BOOKMARK_ITEM:{}{}
", bookmark.name.trim(), tags.trim()));
            }
        }

         Ok(output)
    })
}

fn handle_async_register(
    parts: Vec<String>,
    _command_line: String,
) -> std::pin::Pin<Box<dyn Future<Output = Result<String, String>> + 'static>> {
    Box::pin(async move {
        if AuthService::get_current_user().is_some() {
            return Ok("you are already logged in".to_string());
        } else if parts.len() < 3 {
            return Ok("usage: register <email> <password>".to_string());
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
                    Ok(format!("successfully registered and logged in as: {}", user.email))
                }
                Err(e) => {
                    Err(format!("registration failed: {}", e))
                }
            }
        }
    })
}

fn handle_async_login(
    parts: Vec<String>,
    _command_line: String,
) -> std::pin::Pin<Box<dyn Future<Output = Result<String, String>> + 'static>> {
    Box::pin(async move {
        if AuthService::get_current_user().is_some() {
            return Ok("you are already logged in.".to_string());
        } else if parts.len() < 3 {
            return Ok("usage: login <email> <password>".to_string());
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
                    Ok(format!("successfully logged in as: {}", user.email))
                }
                Err(e) => {
                    Err(format!("login failed: {}", e))
                }
            }
        }
    })
}

fn handle_async_logout(
    _parts: Vec<String>,
    _command_line: String,
) -> std::pin::Pin<Box<dyn Future<Output = Result<String, String>> + 'static>> {
    Box::pin(async move {
        if AuthService::get_current_user().is_none() {
            return Ok("you are not logged in.".to_string());
        } else {
            let config = Config::load();
            let auth_service = AuthService::new(
                config.supabase_url.clone(),
                config.supabase_key.clone(),
            );

            match auth_service.sign_out().await {
                Ok(_) => {
                    Ok("successfully logged out".to_string())
                }
                Err(e) => {
                    Err(format!("logout failed: {}", e))
                }
            }
        }
    })
}

fn handle_async_cat(
    parts: Vec<String>,
    _command_line: String,
) -> std::pin::Pin<Box<dyn Future<Output = Result<String, String>> + 'static>> {
    Box::pin(async move {
        if AuthService::get_current_user().is_none() {
            return Ok("you must be logged in to use this command.".to_string());
        } else if parts.len() < 2 {
            return Ok("usage: cat <bookmark_name>".to_string());
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
                        format!("\ntags: {}", bookmark.tags.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(", "))
                    };
                    Ok(format!("url: {}{}", bookmark.url, tags))
                }
                Ok(None) => {
                    Ok(format!("bookmark '{}' not found.", name))
                }
                Err(e) => {
                    Err(format!("failed to get bookmark: {}", e))
                }
            }
        }
    })
}

fn handle_async_touch(
    parts: Vec<String>,
    _command_line: String,
) -> std::pin::Pin<Box<dyn Future<Output = Result<String, String>> + 'static>> {
    Box::pin(async move {
        if AuthService::get_current_user().is_none() {
            return Ok("you must be logged in to use this command.".to_string());
        } else if parts.len() < 3 {
            return Ok("usage: touch <name> <url> [tags]".to_string());
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
                    Ok(format!("created bookmark '{}'{}", bookmark.name, tags))
                }
                Err(e) => {
                    Err(format!("failed to create bookmark: {}", e))
                }
            }
        }
    })
}

fn handle_async_open(
    parts: Vec<String>,
    _command_line: String,
) -> std::pin::Pin<Box<dyn Future<Output = Result<String, String>> + 'static>> {
    Box::pin(async move {
        if AuthService::get_current_user().is_none() {
            return Ok("you must be logged in to use this command.".to_string());
        } else if parts.len() < 2 {
            return Ok("usage: open <bookmark_name>".to_string());
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
                            Err(format!("failed to open URL: {:?}", e))
                        } else {
                            Ok(format!("opening {} in new tab...", bookmark.url))
                        }
                    } else {
                        Err("failed to open URL: Could not access window".to_string())
                    }
                }
                Ok(None) => {
                    Ok(format!("bookmark '{}' not found.", name))
                }
                Err(e) => {
                    Err(format!("failed to get bookmark: {}", e))
                }
            }
        }
    })
}

fn handle_async_rm(
    parts: Vec<String>,
    _command_line: String,
) -> std::pin::Pin<Box<dyn Future<Output = Result<String, String>> + 'static>> {
    Box::pin(async move {
        if AuthService::get_current_user().is_none() {
            return Ok("you must be logged in to use this command.".to_string());
        } else if parts.len() < 2 {
            return Ok("usage: rm <bookmark_name>".to_string());
        } else {
            let name = parts[1].to_string();

            let config = Config::load();
            let bookmark_service = BookmarkService::new(
                config.supabase_url.clone(),
                config.supabase_key.clone(),
            );

            match bookmark_service.delete_bookmark(&name).await {
                Ok(_) => {
                    Ok(format!("deleted bookmark '{}'", name))
                }
                Err(e) => {
                    Err(format!("failed to delete bookmark: {}", e))
                }
            }
        }
    })
}

fn handle_async_search(
    parts: Vec<String>,
    _command_line: String,
) -> std::pin::Pin<Box<dyn Future<Output = Result<String, String>> + 'static>> {
    Box::pin(async move {
        if AuthService::get_current_user().is_none() {
            return Ok("you must be logged in to use this command.".to_string());
        } else if parts.len() < 2 {
            return Ok("usage: search <query>".to_string());
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
                        Ok(format!("no bookmarks found matching '{}'", query))
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
                    Err(format!("failed to search bookmarks: {}", e))
                }
            }
        }
    })
}

fn handle_async_tag(
    parts: Vec<String>,
    _command_line: String,
) -> std::pin::Pin<Box<dyn Future<Output = Result<String, String>> + 'static>> {
    Box::pin(async move {
        if AuthService::get_current_user().is_none() {
            return Ok("you must be logged in to use this command.".to_string());
        }

        if parts.len() < 4 {
             return Ok("usage: tag <bookmark_name> <add|remove> <tag1> [tag2...]".to_string());
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
                         Ok(format!("updated bookmark '{}'{}", bookmark.name, tags))
                    }
                    Err(e) => {
                        Err(format!("failed to update bookmark: {}", e))
                    }
                }
            }
            Ok(None) => {
                 Ok(format!("bookmark '{}' not found.", name))
            }
            Err(e) => {
                Err(format!("failed to get bookmark: {}", e))
            }
        }
    })
}

async fn handle_async_tree(_parts: Vec<String>, _command_line: String) -> Result<String, String> {
    if AuthService::get_current_user().is_none() {
        return Ok("you must be logged in to use this command.".to_string());
    }

    // Get the cached bookmarks from the window object
    let window = web_sys::window().unwrap();
    let cache = js_sys::Reflect::get(&window, &JsValue::from_str("__bookmark_cache"))
        .ok()
        .and_then(|v| v.as_f64())
        .unwrap_or(0.0);

    let current_time = js_sys::Date::now();
    let cache_age = current_time - cache;

    // If cache is older than 5 minutes, refresh it
    if cache_age > 300000.0 { // 5 minutes in milliseconds
        let config = Config::load();
        let bookmark_service = BookmarkService::new(
            config.supabase_url.clone(),
            config.supabase_key.clone(),
        );
        
        match bookmark_service.get_bookmarks(None).await {
            Ok(bookmarks) => {
                // Update cache timestamp
                js_sys::Reflect::set(&window, &JsValue::from_str("__bookmark_cache"), &JsValue::from_f64(current_time))
                    .unwrap_or_default();
                
                // Store bookmarks in cache
                js_sys::Reflect::set(&window, &JsValue::from_str("__bookmarks"), &serde_wasm_bindgen::to_value(&bookmarks).unwrap())
                    .unwrap_or_default();
                
                generate_tree_output(&bookmarks)
            }
            Err(e) => Err(format!("failed to get bookmarks: {}", e))
        }
    } else {
        // Use cached bookmarks
        let cached_bookmarks: Vec<termstart::services::bookmark::Bookmark> = js_sys::Reflect::get(&window, &JsValue::from_str("__bookmarks"))
            .ok()
            .and_then(|v| serde_wasm_bindgen::from_value(v).ok())
            .unwrap_or_default();
        
        generate_tree_output(&cached_bookmarks)
    }
}

fn generate_tree_output(bookmarks: &[termstart::services::bookmark::Bookmark]) -> Result<String, String> {
    use std::collections::{BTreeMap, BTreeSet};
    let mut tag_map: BTreeMap<String, Vec<&termstart::services::bookmark::Bookmark>> = BTreeMap::new();
    let mut untagged = Vec::new();
    let mut all_tags = BTreeSet::new();
    
    for bookmark in bookmarks {
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