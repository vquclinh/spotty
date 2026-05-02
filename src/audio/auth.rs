use anyhow::{Context, Result};
use librespot_core::{
    authentication::Credentials,
    cache::Cache,
    config::SessionConfig,
    session::Session,
};
use std::path::Path;

pub fn get_audio_session(access_token: String) -> Result<(Session, Credentials)> {
    let cache_dir = Path::new(".spotty_cache");
    if !cache_dir.exists() {
        std::fs::create_dir_all(cache_dir)?;
    }

    let cache = Cache::new(
        Some(cache_dir),
        Some(cache_dir),
        Some(cache_dir),
        None,
    ).context("Failed to create librespot cache")?;

    // Use the token provided by rspotify
    let credentials = Credentials::with_access_token(access_token);

    let session = Session::new(SessionConfig::default(), Some(cache));

    Ok((session, credentials))
}
