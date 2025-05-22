use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct User {
    pub username: String,
    pub email: String,
    pub token: String,
    pub is_authenticated: bool,
    pub is_admin: bool,
}

pub const STORAGE_KEY: &str = "termstart_user";