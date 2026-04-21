use rspotify::{prelude::*, AuthCodePkceSpotify};
use serde::{Serialize, de::DeserializeOwned};
use anyhow::{Result, Context};
use std::collections::HashMap;

pub struct HttpResponse<T> {
    pub response: String,
    pub data: Option<T>
}

impl<T> Default for HttpResponse<T> {
    fn default() -> Self {
        Self { response: String::new(), data: None }
    }
}

impl<T> HttpResponse<T> 
where 
    T: Default + Clone 
{
    pub fn data(&self) -> T {
        self.data.clone().unwrap_or_default()
    }
}

// Returns the deserialized value
pub async fn get<T>(client: &AuthCodePkceSpotify, endpoint: &str, params: &HashMap<&str, &str>) -> Result<HttpResponse<T>>
where
    T: DeserializeOwned,
{
    let response = client.api_get(endpoint, params)
        .await
        .context(format!("GET request failed at: {}", endpoint))?;

    if let Ok(data) = serde_json::from_str(&response) {
        Ok(HttpResponse { response, data: Some(data) })
    } else {
        Ok(HttpResponse { response, data: None })
    }

}

pub async fn post<T, B>(client: &AuthCodePkceSpotify, endpoint: &str, body: &B) -> Result<HttpResponse<T>>
where
    T: DeserializeOwned,
    B: Serialize,
{
    let payload = serde_json::to_value(body)?;

    let response = client.api_post(endpoint, &payload)
        .await
        .context(format!("POST request failed at: {}", endpoint))?;

    if let Ok(data) = serde_json::from_str(&response) {
        Ok(HttpResponse { response, data: Some(data) })
    } else {
        Ok(HttpResponse { response, data: None })
    }
}

// PUT always returns 204
pub async fn put<T, B>(client: &AuthCodePkceSpotify, endpoint: &str, body: &B) -> Result<HttpResponse<T>>
where
    T: DeserializeOwned,
    B: Serialize
{
    let payload = serde_json::to_value(body)?;

    let response = client.api_put(endpoint, &payload)
        .await
        .context(format!("PUT request failed at: {}", endpoint))?;

    if let Ok(data) = serde_json::from_str(&response) {
        Ok(HttpResponse { response, data: Some(data) })
    } else {
        Ok(HttpResponse { response, data: None })
    }
}

pub async fn delete<T, B>(client: &AuthCodePkceSpotify, endpoint: &str, body: &B) -> Result<HttpResponse<T>>
where
    T: DeserializeOwned,
    B: Serialize,
{
    let payload = serde_json::to_value(body)?;

    let response = client.api_delete(endpoint, &payload)
        .await
        .context(format!("DELETE request failed at: {}", endpoint))?;

    if let Ok(data) = serde_json::from_str(&response) {
        Ok(HttpResponse { response, data: Some(data) })
    } else {
        Ok(HttpResponse { response, data: None })
    }
}
