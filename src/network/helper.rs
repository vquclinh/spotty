use rspotify::{prelude::*, AuthCodePkceSpotify};
use serde::{Serialize, de::DeserializeOwned};
use anyhow::{Result, Context};
use std::collections::HashMap;

// GET: For fetching data (Tracks, Playlists, Search results)
pub async fn get<T>(client: &AuthCodePkceSpotify, endpoint: &str, params: &HashMap<&str, &str>) -> Result<T>
where
    T: DeserializeOwned,
{
    let json_str = client.api_get(endpoint, params)
        .await
        .context(format!("GET request failed at: {}", endpoint))?;

    let data: T = serde_json::from_str(&json_str)
        .context("Failed to deserialize GET response")?;

    Ok(data)
}

// POST: For creating resources or adding tracks
pub async fn post<T, B>(client: &AuthCodePkceSpotify, endpoint: &str, body: &B) -> Result<T>
where
    T: DeserializeOwned,
    B: Serialize,
{
    let payload = serde_json::to_value(body)?;
    
    let json_str = client.api_post(endpoint, &payload)
        .await
        .context(format!("POST request failed at: {}", endpoint))?;

    let data: T = serde_json::from_str(&json_str)
        .context("Failed to deserialize POST response")?;

    Ok(data)
}

// PUT: For state changes (Playback control, Volume, Shuffle)
pub async fn put<T, B>(client: &AuthCodePkceSpotify, endpoint: &str, body: &B) -> Result<T>
where
    T: DeserializeOwned,
    B: Serialize,
{
    let payload = serde_json::to_value(body)?;
    
    let json_str = client.api_put(endpoint, &payload)
        .await
        .context(format!("PUT request failed at: {}", endpoint))?;

    let data: T = serde_json::from_str(&json_str)
        .context("Failed to deserialize PUT response")?;

    Ok(data)
}

// DELETE: For removing tracks from playlists
pub async fn delete<T, B>(client: &AuthCodePkceSpotify, endpoint: &str, body: &B) -> Result<T>
where
    T: DeserializeOwned,
    B: Serialize,
{
    let payload = serde_json::to_value(body)?;
    
    let json_str = client.api_delete(endpoint, &payload)
        .await
        .context(format!("DELETE request failed at: {}", endpoint))?;

    let data: T = serde_json::from_str(&json_str)
        .context("Failed to deserialize DELETE response")?;

    Ok(data)
}
