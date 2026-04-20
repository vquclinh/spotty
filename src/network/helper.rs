use rspotify::{prelude::*, AuthCodePkceSpotify};
use serde::{Serialize, de::DeserializeOwned};
use anyhow::{Result, Context, bail};
use std::collections::HashMap;

// Returns the deserialized value
pub async fn get<T>(client: &AuthCodePkceSpotify, endpoint: &str, params: &HashMap<&str, &str>) -> Result<T>
where
    T: DeserializeOwned,
{
    let response = client.api_get(endpoint, params)
        .await
        .context(format!("GET request failed at: {}", endpoint))?;

    if response.trim().is_empty() {
        bail!("GET request returned empty response at: {}", endpoint)
    }

    let data: T = serde_json::from_str(&response)
        .context("Failed to deserialize GET response")?;

    Ok(data)
}

// Used when we want to allow empty response
pub async fn get_opt<T>(client: &AuthCodePkceSpotify, endpoint: &str, params: &HashMap<&str, &str>) -> Result<Option<T>>
where
    T: DeserializeOwned,
{
    let response = client.api_get(endpoint, params)
        .await
        .context(format!("GET request failed at: {}", endpoint))?;

    if response.trim().is_empty() {
        return Ok(None);
    }

    let data: T = serde_json::from_str(&response)
        .context("Failed to deserialize GET response")?;

    Ok(Some(data))
}

pub async fn post<T, B>(client: &AuthCodePkceSpotify, endpoint: &str, body: &B) -> Result<Option<T>>
where
    T: DeserializeOwned,
    B: Serialize,
{
    let payload = serde_json::to_value(body)?;

    let response = client.api_post(endpoint, &payload)
        .await
        .context(format!("POST request failed at: {}", endpoint))?;

    if response.trim().is_empty() {
        return Ok(None);
    }

    let data: T = serde_json::from_str(&response)
        .context("Failed to deserialize POST response")?;

    Ok(Some(data))
}

// PUT always returns 204
pub async fn put<B>(client: &AuthCodePkceSpotify, endpoint: &str, body: &B) -> Result<()>
where
    B: Serialize,
{
    let payload = serde_json::to_value(body)?;

    client.api_put(endpoint, &payload)
        .await
        .context(format!("PUT request failed at: {}", endpoint))?;
    Ok(())
}

pub async fn delete<T, B>(client: &AuthCodePkceSpotify, endpoint: &str, body: &B) -> Result<Option<T>>
where
    T: DeserializeOwned,
    B: Serialize,
{
    let payload = serde_json::to_value(body)?;

    let response = client.api_delete(endpoint, &payload)
        .await
        .context(format!("DELETE request failed at: {}", endpoint))?;

    if response.trim().is_empty() {
        return Ok(None);
    }

    let data: T = serde_json::from_str(&response)
        .context("Failed to deserialize DELETE response")?;
    Ok(Some(data))
}
