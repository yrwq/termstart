use std::sync::OnceLock;
use log::info;

static CONFIG: OnceLock<Config> = OnceLock::new();

#[derive(Clone)]
pub struct Config {
    pub supabase_url: String,
    pub supabase_key: String,
}

impl Config {
    pub fn load() -> &'static Config {
        CONFIG.get_or_init(|| {
            let supabase_url = "https://cusiuzrjqouormzafbou.supabase.co";
            let supabase_key = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6ImN1c2l1enJqcW91b3JtemFmYm91Iiwicm9sZSI6ImFub24iLCJpYXQiOjE3NDc5MTQ5MTMsImV4cCI6MjA2MzQ5MDkxM30.ac64y9_B630_vsibcNwUcWZ59hDyFDsp6i3wP_ebOFI";

            info!("Loading config with URL: {}", supabase_url);
            
            Config {
                supabase_url: supabase_url.to_string(),
                supabase_key: supabase_key.to_string(),
            }
        })
    }
} 