use anyhow::{Context, Result};
use librespot_core::{
    authentication::Credentials,
    cache::Cache,
    config::SessionConfig,
    session::Session,
};
use crate::app::cache;

pub fn get_audio_session(access_token: String) -> Result<(Session, Credentials)> {
    let librespot_cache = Cache::new(
        Some(cache::cache_dir()),
        Some(cache::cache_dir()),
        Some(cache::librespot_audio_dir()),
        None,
    ).context("Failed to create librespot cache")?;

    // Use the token provided by rspotify
    let credentials = Credentials::with_access_token(access_token);

    let session = Session::new(SessionConfig::default(), Some(librespot_cache));

    Ok((session, credentials))
}
