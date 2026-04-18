use super::auth;
use super::models::*;
use super::helper;
use rspotify::{
    prelude::*,
    AuthCodePkceSpotify,
};
use anyhow::{Result, Context};
use std::collections::HashMap;
use std::time::{Instant, Duration};
use serde_json::{Value, json};
use core::iter::IntoIterator;

// -------------------------------------- CACHE ---------------------------------------
#[allow(dead_code)]
pub struct CacheItem<T> {
    item: T,
    fetched_at: Instant
}

// Cache data queries for some time to reduce API calls
pub struct Cache {
    pub tracks: HashMap<String, CacheItem<Track>>,
    pub albums: HashMap<String, CacheItem<Album>>,
    pub artists: HashMap<String, CacheItem<Artist>>,
    pub ttl: Duration, // time to live
}

impl Cache {
    pub fn new(ttl_seconds: u64) -> Self {
        Self {
            tracks: HashMap::new(),
            albums: HashMap::new(),
            artists: HashMap::new(),
            ttl: Duration::from_secs(ttl_seconds),
        }
    }

    pub fn is_expired<T>(&self, item: &CacheItem<T>) -> bool {
        item.fetched_at.elapsed() > self.ttl
    }
}

impl Default for Cache {
    fn default() -> Self {
        // Default tll: 30 mins
        Self::new(30 * 60)
    }
}

// ---------------------------------------- WEB API CLIENT ----------------------------
#[allow(dead_code)]
pub struct WebApiClient {
    client: AuthCodePkceSpotify,
    cache: Cache
}

impl WebApiClient {
    pub async fn new(cache_ttl_sec: Option<u64>) -> Result<Self> {
        let mut client = auth::create_auth_client().await?;
        auth::authenticate(&mut client).await?;

        Ok(Self {
            client,
            cache: cache_ttl_sec.map_or(Cache::default(), Cache::new)
        })
    }

    pub async fn get_current_user(&self) -> Result<User> {
        helper::get(&self.client, "me", &HashMap::new()).await
    }

    pub async fn get_current_playback(&self) -> Result<Option<Playback>> {
        let mut params = HashMap::new();
        params.insert("additional_types", "track,episode");

        // get<T> will fail on 204 No Content (empty string). 
        // We handle this by checking the raw response 
        match self.client.api_get("me/player", &params).await {
            Ok(json_str) if !json_str.is_empty() => {
                let state: Playback = serde_json::from_str(&json_str)?;
                Ok(Some(state))
            }
            _ => Ok(None),
        }
    }

    pub async fn get_user_playlists(&self) -> Result<Vec<Playlist>> {
        // Spotify returns a paging object with an "items" field
        let res: Value = helper::get(&self.client, "me/playlists", &HashMap::new()).await?;

        let playlists = serde_json::from_value(res["items"].clone())
            .context("Failed to parse playlists items")?;

        Ok(playlists)
    }

    pub async fn get_queue(&self) -> Result<Vec<Track>> {
        let res: Value = helper::get(&self.client, "me/player/queue", &HashMap::new()).await?;

        let queue = serde_json::from_value(res["queue"].clone())
            .context("Failed to parse queue items")?;

        Ok(queue)
    }

    pub async fn get_playlist_items(&self, id: &str, limit: u32, offset: u32) -> Result<Vec<PlayableItem>> {
        let endpoint = format!("playlists/{}/tracks", id);
        let mut params = HashMap::new();
        let limit_str = limit.to_string();
        let offset_str = offset.to_string();
        params.insert("limit", limit_str.as_str());
        params.insert("offset", offset_str.as_str());

        let res: Value = helper::get(&self.client, &endpoint, &params).await?;

        // Playlist items are nested under item: { track: { ... } }
        let items: Vec<PlayableItem> = res["items"]
            .as_array()
            .unwrap_or(&vec![])
            .iter()
            .filter_map(|item| {
                let track = item.get("track").filter(|v| !v.is_null());
                let episode = item.get("episode").filter(|v| !v.is_null());

                let target = track.or(episode)?;
                serde_json::from_value(target.clone()).ok()
            })
        .collect();

        Ok(items)
    }

    pub async fn get_user_top_tracks(&self, time_range: TimeRange, limit: u32, offset: u32) -> Result<Vec<Track>> {
        let mut params = HashMap::new();
        let limit_str = limit.to_string();
        let offset_str = offset.to_string();
        params.insert("limit", limit_str.as_str());
        params.insert("offset", offset_str.as_str());
        params.insert("time_range", time_range.as_str());

        let res: Value = helper::get(&self.client, "me/top/tracks", &params).await?;
        let tracks = serde_json::from_value(res["items"].clone())?;
        Ok(tracks)
    }

    // TODO: clamp the arguments
    pub async fn get_user_top_artists(&self, time_range: TimeRange, limit: u32, offset: u32) -> Result<Vec<Artist>> {
        let mut params = HashMap::new();
        let limit_str = limit.to_string();
        let offset_str = offset.to_string();
        params.insert("limit", limit_str.as_str());
        params.insert("offset", offset_str.as_str());
        params.insert("time_range", time_range.as_str());

        let res: Value = helper::get(&self.client, "me/top/artists", &params).await?;
        let artists = serde_json::from_value(res["items"].clone())?;
        Ok(artists)
    }

    pub async fn get_recently_played_tracks(&self, limit: u32) -> Result<Vec<Track>> {
        let mut params = HashMap::new();
        let limit_str = limit.to_string();
        params.insert("limit", limit_str.as_str());

        let res: Value = helper::get(&self.client, "me/player/recently-played", &params).await?;

        // Recently played items are nested under { track: { ... } }
        let tracks = res["items"]
            .as_array()
            .unwrap_or(&vec![])
            .iter()
            .filter_map(|item| serde_json::from_value(item["track"].clone()).ok())
            .collect();

        Ok(tracks)
    }

    pub async fn toggle_playback(&self, playing: bool) -> Result<()> {
        let endpoint = if playing { "me/player/pause" } else { "me/player/play" };
        // Use empty json! object for PUT requests with no body
        helper::put::<Value, _>(&self.client, endpoint, &json!({})).await?;

        Ok(())
    }

    pub async fn next_track(&self) -> Result<()> {
        helper::post::<Value, _>(&self.client, "me/player/next", &json!({})).await?;
        Ok(())
    }

    pub async fn prev_track(&self) -> Result<()> {
        helper::post::<Value, _>(&self.client, "me/player/previous", &json!({})).await?;
        Ok(())
    }

    pub async fn search_items(
        &self,
        query: &str,
        search_types: impl IntoIterator<Item = SearchType>,
        limit: u32
    ) -> Result<SearchResult> {
        let mut params = HashMap::new();
        let limit_str = limit.to_string();

        // Join search_types into a comma-separated string (e.g., "track,artist")
        let type_str = search_types.into_iter()
            .map(|t| format!("{:?}", t).to_lowercase())
            .collect::<Vec<_>>()
            .join(",");

        params.insert("q", query);
        params.insert("type", type_str.as_str());
        params.insert("limit", limit_str.as_str());

        helper::get(&self.client, "search", &params).await
    }
}
