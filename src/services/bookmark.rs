use serde::{Deserialize, Serialize};
use log::{info, debug, error};
use crate::services::auth::{AuthService, User};
use supabase_rs::SupabaseClient;
use serde_json::{json, Value};
use gloo_net::http::Request;
use crate::config::Config;
use url::Url;
use std::collections::HashSet;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Bookmark {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub user_id: String,
    pub name: String,
    pub url: String,
    #[serde(default)]
    pub tags: HashSet<String>,
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub updated_at: String,
}

#[derive(Debug)]
pub enum BookmarkError {
    NotAuthenticated,
    InvalidUrl(String),
    DuplicateName(String),
    DatabaseError(String),
    NetworkError(String),
    ValidationError(String),
}

impl std::fmt::Display for BookmarkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BookmarkError::NotAuthenticated => write!(f, "Not authenticated"),
            BookmarkError::InvalidUrl(url) => write!(f, "Invalid URL: {}", url),
            BookmarkError::DuplicateName(name) => write!(f, "Bookmark with name '{}' already exists", name),
            BookmarkError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            BookmarkError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            BookmarkError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
        }
    }
}

pub struct BookmarkService {
    client: SupabaseClient,
    config: Config,
}

impl BookmarkService {
    pub fn new(supabase_url: String, supabase_key: String) -> Self {
        info!("Creating BookmarkService with URL: {}", supabase_url);
        let client = SupabaseClient::new(
            supabase_url.clone(),
            supabase_key.clone()
        ).expect("Failed to create Supabase client");
        Self { 
            client,
            config: Config {
                supabase_url,
                supabase_key,
            }
        }
    }

    fn get_current_user_id(&self) -> Result<String, BookmarkError> {
        AuthService::get_current_user()
            .map(|user| user.id)
            .ok_or(BookmarkError::NotAuthenticated)
    }

    fn get_auth_token(&self) -> Result<String, BookmarkError> {
        let window = web_sys::window().unwrap();
        let storage = window.local_storage().unwrap().unwrap();
        storage
            .get_item("supabase.auth.token")
            .unwrap()
            .ok_or(BookmarkError::NotAuthenticated)
    }

    fn normalize_url(url: &str) -> Result<String, BookmarkError> {
        let url = if !url.starts_with("http://") && !url.starts_with("https://") {
            format!("https://{}", url)
        } else {
            url.to_string()
        };

        Url::parse(&url)
            .map_err(|e| BookmarkError::InvalidUrl(e.to_string()))
            .map(|_| url)
    }

    fn validate_bookmark_name(name: &str) -> Result<(), BookmarkError> {
        if name.is_empty() {
            return Err(BookmarkError::ValidationError("Bookmark name cannot be empty".into()));
        }
        if name.len() > 100 {
            return Err(BookmarkError::ValidationError("Bookmark name is too long".into()));
        }
        if !name.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') {
            return Err(BookmarkError::ValidationError("Bookmark name can only contain alphanumeric characters, hyphens, and underscores".into()));
        }
        Ok(())
    }

    pub async fn create_bookmark(&self, name: &str, url: &str, tags: Option<Vec<String>>) -> Result<Bookmark, BookmarkError> {
        let user_id = self.get_current_user_id()?;
        let auth_token = self.get_auth_token()?;
        
        // Validate inputs
        Self::validate_bookmark_name(name)?;
        let normalized_url = Self::normalize_url(url)?;
        
        // Check for duplicate name
        if let Ok(Some(_)) = self.get_bookmark_by_name(name).await {
            return Err(BookmarkError::DuplicateName(name.to_string()));
        }

        let tags = tags.unwrap_or_default().into_iter().collect::<HashSet<_>>();
        let now = js_sys::Date::new_0().to_iso_string().as_string().unwrap_or_default();

        let data = json!({
            "user_id": user_id,
            "name": name,
            "url": normalized_url,
            "tags": tags,
            "created_at": now,
            "updated_at": now
        });

        let request = Request::post(&format!("{}/rest/v1/bookmarks", self.config.supabase_url))
            .header("apikey", &self.config.supabase_key)
            .header("Authorization", &format!("Bearer {}", auth_token))
            .header("Content-Type", "application/json")
            .header("Prefer", "return=representation")
            .body(data.to_string())
            .map_err(|e| BookmarkError::NetworkError(e.to_string()))?;

        let response = request.send()
            .await
            .map_err(|e| BookmarkError::NetworkError(e.to_string()))?;

        if !response.ok() {
            let error_text = response.text().await
                .unwrap_or_else(|_| "Failed to get error message".to_string());
            return Err(BookmarkError::DatabaseError(error_text));
        }

        let response_text = response.text().await
            .map_err(|e| BookmarkError::NetworkError(e.to_string()))?;

        // Supabase returns an array with a single item
        let bookmarks: Vec<Bookmark> = serde_json::from_str(&response_text)
            .map_err(|e| BookmarkError::DatabaseError(e.to_string()))?;

        bookmarks.into_iter().next()
            .ok_or_else(|| BookmarkError::DatabaseError("No bookmark returned from database".into()))
    }

    pub async fn get_bookmarks(&self, tag: Option<&str>) -> Result<Vec<Bookmark>, BookmarkError> {
        let user_id = self.get_current_user_id()?;
        let auth_token = self.get_auth_token()?;
        
        let mut url = format!("{}/rest/v1/bookmarks?user_id=eq.{}", self.config.supabase_url, user_id);
        if let Some(tag) = tag {
            url.push_str(&format!("&tags=cs.{{{}", tag));
        }

        let request = Request::get(&url)
            .header("apikey", &self.config.supabase_key)
            .header("Authorization", &format!("Bearer {}", auth_token))
            .header("Content-Type", "application/json");

        let response = request.send()
            .await
            .map_err(|e| BookmarkError::NetworkError(e.to_string()))?;

        if !response.ok() {
            let error_text = response.text().await
                .unwrap_or_else(|_| "Failed to get error message".to_string());
            return Err(BookmarkError::DatabaseError(error_text));
        }

        let response_text = response.text().await
            .map_err(|e| BookmarkError::NetworkError(e.to_string()))?;

        serde_json::from_str(&response_text)
            .map_err(|e| BookmarkError::DatabaseError(e.to_string()))
    }

    pub async fn get_bookmark_by_name(&self, name: &str) -> Result<Option<Bookmark>, BookmarkError> {
        let user_id = self.get_current_user_id()?;
        let auth_token = self.get_auth_token()?;

        let request = Request::get(&format!(
            "{}/rest/v1/bookmarks?user_id=eq.{}&name=eq.{}",
            self.config.supabase_url, user_id, name
        ))
        .header("apikey", &self.config.supabase_key)
        .header("Authorization", &format!("Bearer {}", auth_token))
        .header("Content-Type", "application/json");

        let response = request.send()
            .await
            .map_err(|e| BookmarkError::NetworkError(e.to_string()))?;

        if !response.ok() {
            let error_text = response.text().await
                .unwrap_or_else(|_| "Failed to get error message".to_string());
            return Err(BookmarkError::DatabaseError(error_text));
        }

        let response_text = response.text().await
            .map_err(|e| BookmarkError::NetworkError(e.to_string()))?;

        let bookmarks: Vec<Bookmark> = serde_json::from_str(&response_text)
            .map_err(|e| BookmarkError::DatabaseError(e.to_string()))?;

        Ok(bookmarks.into_iter().next())
    }

    pub async fn update_bookmark(&self, name: &str, url: Option<&str>, tags: Option<Vec<String>>) -> Result<Bookmark, BookmarkError> {
        let user_id = self.get_current_user_id()?;
        let auth_token = self.get_auth_token()?;
        
        // Get existing bookmark
        let existing = self.get_bookmark_by_name(name).await?
            .ok_or_else(|| BookmarkError::ValidationError("Bookmark not found".into()))?;

        // Prepare update data
        let mut update_data = json!({});
        
        if let Some(url) = url {
            let normalized_url = Self::normalize_url(url)?;
            update_data["url"] = json!(normalized_url);
        }
        
        if let Some(tags) = tags {
            update_data["tags"] = json!(tags.into_iter().collect::<HashSet<_>>());
        }
        
        update_data["updated_at"] = json!(js_sys::Date::new_0().to_iso_string().as_string().unwrap_or_default());

        let request = Request::patch(&format!(
            "{}/rest/v1/bookmarks?user_id=eq.{}&name=eq.{}",
            self.config.supabase_url, user_id, name
        ))
        .header("apikey", &self.config.supabase_key)
        .header("Authorization", &format!("Bearer {}", auth_token))
        .header("Content-Type", "application/json")
        .header("Prefer", "return=representation")
        .body(update_data.to_string())
        .map_err(|e| BookmarkError::NetworkError(e.to_string()))?;

        let response = request.send()
            .await
            .map_err(|e| BookmarkError::NetworkError(e.to_string()))?;

        if !response.ok() {
            let error_text = response.text().await
                .unwrap_or_else(|_| "Failed to get error message".to_string());
            return Err(BookmarkError::DatabaseError(error_text));
        }

        let response_text = response.text().await
            .map_err(|e| BookmarkError::NetworkError(e.to_string()))?;

        let bookmarks: Vec<Bookmark> = serde_json::from_str(&response_text)
            .map_err(|e| BookmarkError::DatabaseError(e.to_string()))?;

        bookmarks.into_iter().next()
            .ok_or_else(|| BookmarkError::DatabaseError("No bookmark returned from database".into()))
    }

    pub async fn delete_bookmark(&self, name: &str) -> Result<(), BookmarkError> {
        let user_id = self.get_current_user_id()?;
        let auth_token = self.get_auth_token()?;

        let request = Request::delete(&format!(
            "{}/rest/v1/bookmarks?user_id=eq.{}&name=eq.{}",
            self.config.supabase_url, user_id, name
        ))
        .header("apikey", &self.config.supabase_key)
        .header("Authorization", &format!("Bearer {}", auth_token));

        let response = request.send()
            .await
            .map_err(|e| BookmarkError::NetworkError(e.to_string()))?;

        if !response.ok() {
            let error_text = response.text().await
                .unwrap_or_else(|_| "Failed to get error message".to_string());
            return Err(BookmarkError::DatabaseError(error_text));
        }

        Ok(())
    }

    pub async fn search_bookmarks(&self, query: &str) -> Result<Vec<Bookmark>, BookmarkError> {
        let user_id = self.get_current_user_id()?;
        let auth_token = self.get_auth_token()?;

        let request = Request::get(&format!(
            "{}/rest/v1/bookmarks?user_id=eq.{}&or=(name.ilike.*{}*,url.ilike.*{}*)",
            self.config.supabase_url, user_id, query, query
        ))
        .header("apikey", &self.config.supabase_key)
        .header("Authorization", &format!("Bearer {}", auth_token))
        .header("Content-Type", "application/json");

        let response = request.send()
            .await
            .map_err(|e| BookmarkError::NetworkError(e.to_string()))?;

        if !response.ok() {
            let error_text = response.text().await
                .unwrap_or_else(|_| "Failed to get error message".to_string());
            return Err(BookmarkError::DatabaseError(error_text));
        }

        let response_text = response.text().await
            .map_err(|e| BookmarkError::NetworkError(e.to_string()))?;

        serde_json::from_str(&response_text)
            .map_err(|e| BookmarkError::DatabaseError(e.to_string()))
    }
} 