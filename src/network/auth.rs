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

    const CLIENT_ID: &str = "2c51a156a0a649b88bf852b12feedf7b";
    const REDIRECT_URI: &str = "http://127.0.0.1:8888/callback";

    let creds = Credentials::from_env()
        .unwrap_or_else(|| Credentials::new(CLIENT_ID, ""));

    let scopes = rspotify::scopes!(
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
    );

    let oauth = OAuth::from_env(scopes.clone()).unwrap_or(OAuth {
        redirect_uri: REDIRECT_URI.to_string(),
        scopes,
        ..Default::default()
    });

    Ok(AuthCodePkceSpotify::with_config(creds, oauth, conf))
}

pub async fn authenticate(client: &mut AuthCodePkceSpotify) -> Result<()> {
    let url = client.get_authorize_url(None)?;

    // Automatically refresh token if it has expired
    client.prompt_for_token(&url).await?;

    Ok(())
}
