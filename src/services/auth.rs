use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::JsFuture;
use log::{info, error};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub email: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthState {
    pub user: Option<User>,
    pub loading: bool,
}

pub struct AuthService {
    supabase_url: String,
    supabase_key: String,
}

impl AuthService {
    pub fn new(supabase_url: String, supabase_key: String) -> Self {
        info!("Creating AuthService with URL: {}", supabase_url);
        Self {
            supabase_url,
            supabase_key,
        }
    }

    pub async fn sign_up(&self, email: &str, password: &str) -> Result<User, String> {
        info!("Attempting to sign up with email: {}", email);
        let window = web_sys::window().unwrap();
        
        let supabase = js_sys::eval(&format!(
            "window.supabase.createClient('{}', '{}', {{
                auth: {{
                    persistSession: true,
                    autoRefreshToken: true,
                    detectSessionInUrl: true,
                    flowType: 'pkce',
                    storage: window.localStorage,
                    storageKey: 'supabase.auth.token',
                    debug: false,
                    tokenExpirationTime: 31536000000 // 1 year in milliseconds
                }}
            }})",
            self.supabase_url, self.supabase_key
        ))
        .map_err(|e| {
            error!("Failed to create Supabase client: {:?}", e);
            format!("Failed to create Supabase client: {:?}", e)
        })?;

        info!("Supabase client created successfully");

        let auth = js_sys::Reflect::get(&supabase, &JsValue::from_str("auth"))
            .map_err(|e| {
                error!("Failed to get auth object: {:?}", e);
                format!("Failed to get auth object: {:?}", e)
            })?;

        info!("Got auth object");

        let sign_up_fn = js_sys::Reflect::get(&auth, &JsValue::from_str("signUp"))
            .map_err(|e| {
                error!("Failed to get signUp function: {:?}", e);
                format!("Failed to get signUp function: {:?}", e)
            })?
            .dyn_into::<js_sys::Function>()
            .map_err(|e| {
                error!("Failed to convert to Function: {:?}", e);
                format!("Failed to convert to Function: {:?}", e)
            })?;

        info!("Got signUp function");

        let options = js_sys::Object::new();
        js_sys::Reflect::set(&options, &JsValue::from_str("email"), &JsValue::from_str(email))
            .map_err(|e| {
                error!("Failed to set email: {:?}", e);
                format!("Failed to set email: {:?}", e)
            })?;
        js_sys::Reflect::set(&options, &JsValue::from_str("password"), &JsValue::from_str(password))
            .map_err(|e| {
                error!("Failed to set password: {:?}", e);
                format!("Failed to set password: {:?}", e)
            })?;

        info!("Created sign up options");

        let sign_up_promise = sign_up_fn
            .call1(&auth, &options)
            .map_err(|e| {
                error!("Failed to call signUp: {:?}", e);
                format!("Failed to call signUp: {:?}", e)
            })?;

        info!("Called signUp");

        let sign_up_result = JsFuture::from(js_sys::Promise::from(sign_up_promise))
            .await
            .map_err(|e| {
                error!("Failed to await sign up: {:?}", e);
                format!("Failed to await sign up: {:?}", e)
            })?;

        info!("Got sign up result");
        info!("Sign up response: {:?}", sign_up_result);

        let error = js_sys::Reflect::get(&sign_up_result, &JsValue::from_str("error"))
            .ok()
            .and_then(|e| {
                if e.is_null() {
                    None
                } else {
                    let message = js_sys::Reflect::get(&e, &JsValue::from_str("message"))
                        .ok()
                        .and_then(|m| m.as_string())
                        .unwrap_or_else(|| "Unknown error".to_string());
                    
                    // Check if this is a rate limiting error
                    if message.contains("For security purposes, you can only request this after") {
                        Some("Please wait a moment before trying to register again. This is a security measure to protect against abuse.".to_string())
                    } else {
                        Some(message)
                    }
                }
            });
        
        if let Some(error_msg) = error {
            error!("Sign up failed: {}", error_msg);
            return Err(format!("Sign up failed: {}", error_msg));
        }

        let user_data = js_sys::Reflect::get(&sign_up_result, &JsValue::from_str("data"))
            .map_err(|e| {
                error!("Failed to get user data: {:?}", e);
                format!("Failed to get user data: {:?}", e)
            })?;

        info!("User data: {:?}", user_data);

        // Check if user_data is null
        if user_data.is_null() {
            error!("User data is null in response");
            return Err("Registration failed: Invalid response".to_string());
        }

        let user = js_sys::Reflect::get(&user_data, &JsValue::from_str("user"))
            .map_err(|e| {
                error!("Failed to get user: {:?}", e);
                format!("Failed to get user: {:?}", e)
            })?;

        if user.is_null() {
            error!("User is null in response");
            return Err("Registration failed: Invalid response".to_string());
        }

        let id = js_sys::Reflect::get(&user, &JsValue::from_str("id"))
            .map_err(|e| {
                error!("Failed to get user id: {:?}", e);
                format!("Failed to get user id: {:?}", e)
            })?
            .as_string()
            .ok_or_else(|| {
                error!("User id is not a string");
                "User id is not a string".to_string()
            })?;

        let email = js_sys::Reflect::get(&user, &JsValue::from_str("email"))
            .map_err(|e| {
                error!("Failed to get user email: {:?}", e);
                format!("Failed to get user email: {:?}", e)
            })?
            .as_string()
            .ok_or_else(|| {
                error!("User email is not a string");
                "User email is not a string".to_string()
            })?;

        let user = User { id, email };

        info!("Successfully parsed user data");

        let storage = window.local_storage().unwrap().unwrap();
        
        // Check if we have a session (user might need to confirm email)
        let session = js_sys::Reflect::get(&user_data, &JsValue::from_str("session"))
            .ok()
            .and_then(|s| {
                if s.is_null() {
                    info!("Session is null");
                    None
                } else {
                    info!("Session data: {:?}", s);
                    Some(s)
                }
            });

        if let Some(session) = session {
            let access_token = js_sys::Reflect::get(&session, &JsValue::from_str("access_token"))
                .map_err(|e| {
                    error!("Failed to get access token: {:?}", e);
                    format!("Failed to get access token: {:?}", e)
                })?
                .as_string()
                .ok_or_else(|| {
                    error!("Access token is not a string");
                    "Access token is not a string".to_string()
                })?;

            info!("Got access token: {}", access_token);

            // Store both the user object and the token
            storage
                .set_item("supabase.auth.user", &serde_json::to_string(&user).unwrap())
                .map_err(|e| {
                    error!("Failed to store user: {:?}", e);
                    format!("Failed to store user: {:?}", e)
                })?;

            storage
                .set_item("supabase.auth.token", &access_token)
                .map_err(|e| {
                    error!("Failed to store token: {:?}", e);
                    format!("Failed to store token: {:?}", e)
                })?;

            info!("Successfully stored session");
        } else {
            // No session means user needs to confirm email
            info!("No session in response - email confirmation required");
            return Err("Registration successful! Please check your email to confirm your account.".to_string());
        }

        Ok(user)
    }

    pub async fn sign_in(&self, email: &str, password: &str) -> Result<User, String> {
        info!("Attempting to sign in with email: {}", email);
        let window = web_sys::window().unwrap();
        
        let supabase = js_sys::eval(&format!(
            "window.supabase.createClient('{}', '{}', {{
                auth: {{
                    persistSession: true,
                    autoRefreshToken: true,
                    detectSessionInUrl: true,
                    flowType: 'pkce',
                    storage: window.localStorage,
                    storageKey: 'supabase.auth.token',
                    debug: false,
                    tokenExpirationTime: 31536000000 // 1 year in milliseconds
                }}
            }})",
            self.supabase_url, self.supabase_key
        ))
        .map_err(|e| {
            error!("Failed to create Supabase client: {:?}", e);
            format!("Failed to create Supabase client: {:?}", e)
        })?;

        info!("Supabase client created successfully");

        let auth = js_sys::Reflect::get(&supabase, &JsValue::from_str("auth"))
            .map_err(|e| {
                error!("Failed to get auth object: {:?}", e);
                format!("Failed to get auth object: {:?}", e)
            })?;

        info!("Got auth object");

        let sign_in_fn = js_sys::Reflect::get(&auth, &JsValue::from_str("signInWithPassword"))
            .map_err(|e| {
                error!("Failed to get signInWithPassword function: {:?}", e);
                format!("Failed to get signInWithPassword function: {:?}", e)
            })?
            .dyn_into::<js_sys::Function>()
            .map_err(|e| {
                error!("Failed to convert to Function: {:?}", e);
                format!("Failed to convert to Function: {:?}", e)
            })?;

        info!("Got signInWithPassword function");

        let options = js_sys::Object::new();
        js_sys::Reflect::set(&options, &JsValue::from_str("email"), &JsValue::from_str(email))
            .map_err(|e| {
                error!("Failed to set email: {:?}", e);
                format!("Failed to set email: {:?}", e)
            })?;
        js_sys::Reflect::set(&options, &JsValue::from_str("password"), &JsValue::from_str(password))
            .map_err(|e| {
                error!("Failed to set password: {:?}", e);
                format!("Failed to set password: {:?}", e)
            })?;

        info!("Created sign in options");

        let sign_in_promise = sign_in_fn
            .call1(&auth, &options)
            .map_err(|e| {
                error!("Failed to call signInWithPassword: {:?}", e);
                format!("Failed to call signInWithPassword: {:?}", e)
            })?;

        info!("Called signInWithPassword");

        let sign_in_result = JsFuture::from(js_sys::Promise::from(sign_in_promise))
            .await
            .map_err(|e| {
                error!("Failed to await sign in: {:?}", e);
                format!("Failed to await sign in: {:?}", e)
            })?;

        info!("Got sign in result");

        info!("Sign in response: {:?}", sign_in_result);

        let error = js_sys::Reflect::get(&sign_in_result, &JsValue::from_str("error"))
            .ok()
            .and_then(|e| {
                if e.is_null() {
                    None
                } else {
                    let message = js_sys::Reflect::get(&e, &JsValue::from_str("message"))
                        .ok()
                        .and_then(|m| m.as_string())
                        .unwrap_or_else(|| "Unknown error".to_string());
                    Some(message)
                }
            });
        
        if let Some(error_msg) = error {
            error!("Sign in failed: {}", error_msg);
            return Err(format!("Sign in failed: {}", error_msg));
        }

        let user_data = js_sys::Reflect::get(&sign_in_result, &JsValue::from_str("data"))
            .map_err(|e| {
                error!("Failed to get user data: {:?}", e);
                format!("Failed to get user data: {:?}", e)
            })?;

        info!("User data: {:?}", user_data);

        let user = js_sys::Reflect::get(&user_data, &JsValue::from_str("user"))
            .map_err(|e| {
                error!("Failed to get user: {:?}", e);
                format!("Failed to get user: {:?}", e)
            })?;

        if user.is_null() {
            error!("User is null in response");
            return Err("Authentication failed: Invalid credentials".to_string());
        }

        let id = js_sys::Reflect::get(&user, &JsValue::from_str("id"))
            .map_err(|e| {
                error!("Failed to get user id: {:?}", e);
                format!("Failed to get user id: {:?}", e)
            })?
            .as_string()
            .ok_or_else(|| {
                error!("User id is not a string");
                "User id is not a string".to_string()
            })?;

        let email = js_sys::Reflect::get(&user, &JsValue::from_str("email"))
            .map_err(|e| {
                error!("Failed to get user email: {:?}", e);
                format!("Failed to get user email: {:?}", e)
            })?
            .as_string()
            .ok_or_else(|| {
                error!("User email is not a string");
                "User email is not a string".to_string()
            })?;

        let user = User { id, email };

        info!("Successfully parsed user data");

        let storage = window.local_storage().unwrap().unwrap();
        let session = js_sys::Reflect::get(&user_data, &JsValue::from_str("session"))
            .map_err(|e| {
                error!("Failed to get session: {:?}", e);
                format!("Failed to get session: {:?}", e)
            })?;

        let access_token = js_sys::Reflect::get(&session, &JsValue::from_str("access_token"))
            .map_err(|e| {
                error!("Failed to get access token: {:?}", e);
                format!("Failed to get access token: {:?}", e)
            })?
            .as_string()
            .ok_or_else(|| {
                error!("Access token is not a string");
                "Access token is not a string".to_string()
            })?;

        // Store both the user object and the token
        storage
            .set_item("supabase.auth.user", &serde_json::to_string(&user).unwrap())
            .map_err(|e| {
                error!("Failed to store user: {:?}", e);
                format!("Failed to store user: {:?}", e)
            })?;

        storage
            .set_item("supabase.auth.token", &access_token)
            .map_err(|e| {
                error!("Failed to store token: {:?}", e);
                format!("Failed to store token: {:?}", e)
            })?;

        info!("Successfully stored session");
        Ok(user)
    }

    pub async fn sign_out(&self) -> Result<(), String> {
        info!("Attempting to sign out");
        let window = web_sys::window().unwrap();
        let storage = window.local_storage().unwrap().unwrap();
        
        // Clear all auth-related data from storage
        storage.remove_item("supabase.auth.token").ok();
        storage.remove_item("supabase.auth.user").ok();
        
        // Try to sign out from Supabase, but don't fail if it doesn't work
        let supabase = js_sys::eval(&format!(
            "window.supabase.createClient('{}', '{}')",
            self.supabase_url, self.supabase_key
        )).ok();

        if let Some(supabase) = supabase {
            if let Ok(auth) = js_sys::Reflect::get(&supabase, &JsValue::from_str("auth")) {
                if let Ok(sign_out_fn) = js_sys::Reflect::get(&auth, &JsValue::from_str("signOut"))
                    .and_then(|f| f.dyn_into::<js_sys::Function>())
                {
                    let _ = sign_out_fn.call0(&auth);
                }
            }
        }

        info!("Successfully cleared session");
        Ok(())
    }

    pub fn get_current_user() -> Option<User> {
        let window = web_sys::window().unwrap();
        let storage = window.local_storage().unwrap().unwrap();
        
        // First check if we have a token
        let token = storage.get_item("supabase.auth.token").unwrap();
        if token.is_none() {
            return None;
        }

        // Then get the user
        storage
            .get_item("supabase.auth.user")
            .unwrap()
            .and_then(|user| serde_json::from_str(&user).ok())
    }

    pub async fn refresh_token(&self) -> Result<(), String> {
        let window = web_sys::window().unwrap();
        let storage = window.local_storage().unwrap().unwrap();
        
        let supabase = js_sys::eval(&format!(
            "window.supabase.createClient('{}', '{}', {{
                auth: {{
                    persistSession: true,
                    autoRefreshToken: true,
                    detectSessionInUrl: true,
                    flowType: 'pkce',
                    storage: window.localStorage,
                    storageKey: 'supabase.auth.token',
                    debug: false,
                    tokenExpirationTime: 31536000000 // 1 year in milliseconds
                }}
            }})",
            self.supabase_url, self.supabase_key
        ))
        .map_err(|e| format!("Failed to create Supabase client: {:?}", e))?;

        let auth = js_sys::Reflect::get(&supabase, &JsValue::from_str("auth"))
            .map_err(|e| format!("Failed to get auth object: {:?}", e))?;

        let refresh_fn = js_sys::Reflect::get(&auth, &JsValue::from_str("refreshSession"))
            .map_err(|e| format!("Failed to get refreshSession function: {:?}", e))?
            .dyn_into::<js_sys::Function>()
            .map_err(|e| format!("Failed to convert to Function: {:?}", e))?;

        let refresh_promise = refresh_fn
            .call0(&auth)
            .map_err(|e| format!("Failed to call refreshSession: {:?}", e))?;

        let refresh_result = JsFuture::from(js_sys::Promise::from(refresh_promise))
            .await
            .map_err(|e| format!("Failed to refresh token: {:?}", e))?;

        let error = js_sys::Reflect::get(&refresh_result, &JsValue::from_str("error"))
            .ok()
            .and_then(|e| {
                if e.is_null() {
                    None
                } else {
                    let message = js_sys::Reflect::get(&e, &JsValue::from_str("message"))
                        .ok()
                        .and_then(|m| m.as_string())
                        .unwrap_or_else(|| "Unknown error".to_string());
                    Some(message)
                }
            });
        
        if let Some(error_msg) = error {
            return Err(format!("Token refresh failed: {}", error_msg));
        }

        let session = js_sys::Reflect::get(&refresh_result, &JsValue::from_str("data"))
            .map_err(|e| format!("Failed to get session data: {:?}", e))?
            .dyn_into::<js_sys::Object>()
            .map_err(|e| format!("Failed to convert to Object: {:?}", e))?;

        let access_token = js_sys::Reflect::get(&session, &JsValue::from_str("access_token"))
            .map_err(|e| format!("Failed to get access token: {:?}", e))?
            .as_string()
            .ok_or_else(|| "Access token is not a string".to_string())?;

        storage
            .set_item("supabase.auth.token", &access_token)
            .map_err(|e| format!("Failed to store token: {:?}", e))?;

        Ok(())
    }
} 