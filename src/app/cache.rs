use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::LazyLock;
use serde::{Serialize, Deserialize};
use anyhow::Result;

static CACHE_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    dirs::cache_dir()
        .unwrap_or_else(|| PathBuf::from("./spotty_cache"))
        .join("spotty")
});
static AUDIO_DIR: LazyLock<PathBuf> = LazyLock::new(|| CACHE_DIR.join("audio"));
static LOG_DIR: LazyLock<PathBuf> = LazyLock::new(|| CACHE_DIR.join("log"));
static APP_CACHE_PATH: LazyLock<PathBuf> = LazyLock::new(|| CACHE_DIR.join("app_cache.json"));
static PLAYBACK_CACHE_PATH: LazyLock<PathBuf> = LazyLock::new(|| CACHE_DIR.join("playback.json"));
static TOKEN_CACHE_PATH: LazyLock<PathBuf> = LazyLock::new(|| CACHE_DIR.join(".spotify_token_cache.json"));

pub fn cache_dir() -> &'static Path           { &CACHE_DIR }
pub fn librespot_audio_dir() -> &'static Path { &AUDIO_DIR }
pub fn log_dir() -> &'static Path             { &LOG_DIR }
pub fn app_cache_path() -> &'static Path      { &APP_CACHE_PATH }
pub fn playback_cache_path() -> &'static Path { &PLAYBACK_CACHE_PATH }
pub fn token_cache_path() -> &'static Path    { &TOKEN_CACHE_PATH }

pub fn ensure_cache_dirs() -> Result<()> {
    fs::create_dir_all(cache_dir())?;
    fs::create_dir_all(librespot_audio_dir())?;
    fs::create_dir_all(log_dir())?;
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppCache {
    pub shuffle_state: HashMap<String, bool>,
    pub volume: u8,
}

impl AppCache {
    pub fn save(&self) -> Result<()> {
        crate::spotty_info!("cache", "Saving app cache...");
        let data = serde_json::to_string(self)?;
        fs::write(app_cache_path(), data)?;
        crate::spotty_info!("cache", "App cache saved successfully");
        Ok(())
    }

    pub fn load() -> Result<Self> {
        crate::spotty_info!("cache", "Loading app cache from disk...");
        let data = fs::read_to_string(app_cache_path())?;
        crate::spotty_info!("cache", "App cache loaded successfully");
        Ok(serde_json::from_str(&data)?)
    }

    pub fn shuffle(&self, context_uri: &str) -> bool {
        self.shuffle_state.get(context_uri).copied().unwrap_or(false)
    }
}

impl Default for AppCache {
    fn default() -> Self {
        Self {
            shuffle_state: HashMap::new(),
            volume: 50,
        }
    }
}
