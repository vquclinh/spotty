use rspotify::{
    prelude::*,
    AuthCodePkceSpotify,
    Credentials,
    OAuth,
    Config
};
use anyhow::Result;

pub async fn create_auth_client() -> Result<AuthCodePkceSpotify> {
    let conf = Config {
        // Enable this so that if the token is already in cache
        // then user does not need to authorize again
        token_cached : true,
        ..Default::default()
    };

    let creds = Credentials::from_env().ok_or(anyhow::anyhow!("Spotify credentials not found"))?;
    let oauth = OAuth::from_env(
        rspotify::scopes!(
            // Spotify Connect
            "user-read-playback-state",
            "user-modify-playback-state",
            "user-read-currently-playing",
            "user-read-private",
            "user-read-email",
            // Playback
            "app-remote-control",
            "streaming",
            // Playlists
            "playlist-read-private",
            "playlist-read-collaborative",
            "playlist-modify-private",
            "playlist-modify-public",
            // Listening history
            "user-read-playback-position",
            "user-top-read",
            "user-read-recently-played",
            // Library
            "user-library-modify",
            "user-library-read",
            // Users
            "user-follow-modify",
            "user-follow-read"
        )
    ).ok_or(anyhow::anyhow!("Spotify OAuth config not found"))?;

    Ok(AuthCodePkceSpotify::with_config(creds, oauth, conf))
}

pub async fn authenticate(client: &mut AuthCodePkceSpotify) -> Result<()> {
    let url = client.get_authorize_url(None)?;

    // Automatically refresh token if it has expired
    client.prompt_for_token(&url).await?;

    Ok(())
}
