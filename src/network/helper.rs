use rspotify::{prelude::*, AuthCodePkceSpotify};
use serde::{Serialize, de::DeserializeOwned};
use anyhow::Result;
use std::collections::HashMap;

// The current implementation simply ignores if the reponse deserialization
// fails because there are quite a few cases to handle. We could probably
// improve this later.
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
    T: Default
{
    pub fn data(self) -> T {
        self.data.unwrap_or_default()
    }
}

// Returns the deserialized value
pub async fn get<T>(client: &AuthCodePkceSpotify, endpoint: &str, params: &HashMap<&str, &str>)
-> Result<HttpResponse<T>>
where
    T: DeserializeOwned,
{
    let response = client.api_get(endpoint, params)
        .await
        .map_err(|e| anyhow::anyhow!("GET request failed at {}: {}", endpoint, e))?;

    match serde_json::from_str::<T>(&response) {
        Ok(data) => Ok(HttpResponse { response, data: Some(data) }),
        Err(e) => {
            if response.trim().is_empty() {
                Ok(HttpResponse { response, data: None })
            } else {
                Err(anyhow::anyhow!("Parse error: {}\nRaw response: {}", e, response))
            }
        }
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
        .map_err(|e| anyhow::anyhow!("POST request failed at {}: {}", endpoint, e))?;

    match serde_json::from_str::<T>(&response) {
        Ok(data) => Ok(HttpResponse { response, data: Some(data) }),
        Err(e) => {
            if response.trim().is_empty() {
                Ok(HttpResponse { response, data: None })
            } else {
                Err(anyhow::anyhow!("Parse error: {}\nRaw response: {}", e, response))
            }
        }
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
        .map_err(|e| anyhow::anyhow!("PUT request failed at {}: {}", endpoint, e))?;

    match serde_json::from_str::<T>(&response) {
        Ok(data) => Ok(HttpResponse { response, data: Some(data) }),
        Err(e) => {
            if response.trim().is_empty() {
                Ok(HttpResponse { response, data: None })
            } else {
                Err(anyhow::anyhow!("Parse error: {}\nRaw response: {}", e, response))
            }
        }
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
        .map_err(|e| anyhow::anyhow!("DELETE request failed at {}: {}", endpoint, e))?;

    match serde_json::from_str::<T>(&response) {
        Ok(data) => Ok(HttpResponse { response, data: Some(data) }),
        Err(e) => {
            if response.trim().is_empty() {
                Ok(HttpResponse { response, data: None })
            } else {
                Err(anyhow::anyhow!("Parse error: {}\nRaw response: {}", e, response))
            }
        }
    }
}
