use anyhow::{Context, Result};
use librespot_core::{
    authentication::Credentials,
    cache::Cache,
    config::SessionConfig,
    session::Session,
};
use librespot_oauth::OAuthClientBuilder;
use std::env;
use std::path::Path;

pub fn get_audio_session() -> Result<(Session, Credentials)> {
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

    let credentials = match cache.credentials() {
        Some(creds) => creds,
        None => {
            let client_id = env::var("RSPOTIFY_CLIENT_ID")
                .context("RSPOTIFY_CLIENT_ID not found in .env")?;
            let redirect_uri = env::var("RSPOTIFY_REDIRECT_URI")
                .context("RSPOTIFY_REDIRECT_URI not found in .env")?;

            let oauth_client = OAuthClientBuilder::new(
                &client_id,
                &redirect_uri,
                vec![
                    "streaming",
                    "user-read-playback-state",
                    "user-modify-playback-state",
                    "user-read-currently-playing",
                    "app-remote-control",
                ],
            )
            .open_in_browser()
            .build()
            .context("Failed to build OAuth client")?;

            let token = oauth_client
                .get_access_token()
                .context("Failed to get access token")?;

            Credentials::with_access_token(token.access_token)
        }
    };

    // initialize session
    let session = Session::new(SessionConfig::default(), Some(cache));

    Ok((session, credentials))
}