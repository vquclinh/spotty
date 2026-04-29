use super::auth;
use super::models::*;
use super::helper;
use rspotify::{
    AuthCodePkceSpotify,
};
use anyhow::{Result, Context};
use std::collections::HashMap;
use std::time::{Instant, Duration};
use serde_json::{Value, json};
use serde::Deserialize;
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
    pub client: AuthCodePkceSpotify,
    pub cache: Cache
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
        helper::get(&self.client, "me", &HashMap::new())
            .await.map(|r| r.data())
    }

    pub async fn get_current_playback(&self) -> Result<Option<Playback>> {
        let params = HashMap::from([
            ("additional_types", "track,episode")
        ]);

        helper::get(&self.client, "me/player", &params)
            .await.map(|r| r.data())
    }

    pub async fn get_user_playlists(&self) -> Result<Vec<Playlist>> {
        // Spotify returns a paging object with an "items" field
        let data: Value = helper::get(&self.client, "me/playlists", &HashMap::new())
            .await.map(|r| r.data())?;

        let playlists = serde_json::from_value(data["items"].clone())
            .context("Failed to parse playlists items")?;

        Ok(playlists)
    }

    pub async fn get_queue(&self) -> Result<QueueResponse> {
        let data: Value = helper::get(&self.client, "me/player/queue", &HashMap::new())
            .await.map(|r| r.data())?;

        let queue_res: QueueResponse = serde_json::from_value(data)
            .context("Failed to parse queue response from Spotify")?;

        Ok(queue_res)
    }

    pub async fn get_playlist_items(&self, id: &str, limit: u32, offset: u32) -> Result<Vec<PlayableItem>> {
        let endpoint = format!("playlists/{}/items", id);
        let limit = limit.clamp(1, 50).to_string();
        let offset = offset.to_string();
        let params = HashMap::from([
            ("limit", limit.as_str()),
            ("offset", offset.as_str())
        ]);

        let data: Value = helper::get(&self.client, &endpoint, &params).await?.data();

        let items: Vec<PlayableItem> = data["items"]
            .as_array()
            .unwrap_or(&vec![])
            .iter()
            .filter_map(|wrapper| {
                let target = wrapper.get("track")
                    .or_else(|| wrapper.get("item"))
                    .filter(|v| !v.is_null())?;

                serde_json::from_value::<PlayableItem>(target.clone()).ok()
            })
            .collect();

        Ok(items)
    }

    pub async fn get_user_top_tracks(&self, time_range: TimeRange, limit: u32, offset: u32) -> Result<Vec<Track>> {
        let limit = limit.clamp(1, 50).to_string();
        let offset = offset.to_string();
        let params = HashMap::<&str, &str>::from([
            ("limit", limit.as_str()),
            ("offset", offset.as_str()),
            ("time_range", time_range.as_str())
        ]);

        let data: Value = helper::get(&self.client, "me/top/tracks", &params)
            .await?.data();
        let tracks = serde_json::from_value(data["items"].clone())?;
        Ok(tracks)
    }

    pub async fn get_user_top_artists(&self, time_range: TimeRange, limit: u32, offset: u32) -> Result<Vec<Artist>> {
        let limit = limit.clamp(1, 50).to_string();
        let offset = offset.to_string();
        let params = HashMap::<&str, &str>::from([
            ("limit", limit.as_str()),
            ("offset", offset.as_str()),
            ("time_range", time_range.as_str())
        ]);

        let data: Value = helper::get(&self.client, "me/top/artists", &params)
            .await?.data();
        let artists = serde_json::from_value(data["items"].clone())?;
        Ok(artists)
    }

    pub async fn get_recently_played_tracks(&self, limit: u32, offset: u32) -> Result<Vec<Track>> {
        let limit = limit.clamp(1, 50).to_string();
        let offset = offset.to_string();
        let params = HashMap::<&str, &str>::from([
            ("limit", limit.as_str()),
            ("offset", offset.as_str())
        ]);

        let data: Value = helper::get(&self.client, "me/player/recently-played", &params)
            .await?.data();

        // Recently played items are nested under { track: { ... } }
        let tracks = data["items"]
            .as_array()
            .unwrap_or(&vec![])
            .iter()
            .filter_map(|item| serde_json::from_value(item["track"].clone()).ok())
            .collect();

        Ok(tracks)
    }

    pub async fn toggle_playback(&self, playing: bool) -> Result<()> {
        let endpoint = if playing { "me/player/pause" } else { "me/player/play" };
        helper::put(&self.client, endpoint, &HashMap::<&str, &str>::new())
            .await.map(|r| r.data())
    }

    pub async fn next_track(&self) -> Result<()> {
        helper::post(&self.client, "me/player/next", &HashMap::<&str, &str>::new())
            .await.map(|r| r.data())
    }

    pub async fn prev_track(&self) -> Result<()> {
        helper::post(&self.client, "me/player/previous", &HashMap::<&str, &str>::new())
            .await.map(|r| r.data())
    }

    pub async fn search_items(
        &self,
        query: &str,
        search_types: impl IntoIterator<Item = SearchType>,
        limit: u32,
        offset: u32
    ) -> Result<SearchResult> {
        let limit = limit.clamp(1, 10).to_string();
        let offset = offset.to_string();
        // Join search_types into a comma-separated string (e.g., "track,artist")
        let kind = search_types.into_iter()
            .map(|t| match t {
                SearchType::Track => "track",
                SearchType::Artist => "artist",
                SearchType::Album => "album",
                SearchType::Playlist => "playlist",
                SearchType::Episode => "episode",
            })
        .collect::<Vec<_>>()
        .join(",");

        let params = HashMap::<&str, &str>::from([
            ("q", query),
            ("type", kind.as_str()),
            ("limit", limit.as_str()),
            ("offset", offset.as_str()),
        ]);

        helper::get(&self.client, "search", &params)
            .await.map(|r| r.data())
    }

    pub async fn set_repeat_mode(&self, state: RepeatState) -> Result<()> {
        let url = format!("me/player/repeat?state={}", state.as_str());

        helper::put(&self.client, &url, &HashMap::<&str, &str>::new())
            .await.map(|r| r.data())
    }

    pub async fn seek_to_position(&self, position_ms: u32) -> Result<()> {
        let url = format!("me/player/seek?position_ms={}", position_ms);

        helper::put(&self.client, &url, &HashMap::<&str, &str>::new())
            .await.map(|r| r.data())
    }

    pub async fn set_volume(&self, volume_percent: u8) -> Result<()> {
        let url = format!("me/player/volume?volume_percent={}", volume_percent);

        helper::put(&self.client, &url, &HashMap::<&str, &str>::new())
            .await.map(|r| r.data())
    }

    pub async fn toggle_shuffle(&self, shuffling: bool) -> Result<()> {
        let url = format!("me/player/shuffle?state={}", !shuffling);

        helper::put(&self.client, &url, &HashMap::<&str, &str>::new())
            .await.map(|r| r.data())
    }

    pub async fn add_item_to_queue(&self, uri: &str) -> Result<()> {
        let url = format!("me/player/queue?uri={}", uri);

        helper::post(&self.client, &url, &HashMap::<&str, &str>::new())
            .await.map(|r| r.data())
    }

    pub async fn add_items_to_playlist(&self, playlist_id: &str, uris: impl IntoIterator<Item = &str>) -> Result<()> {
        let endpoint = format!("playlists/{}/items", playlist_id);
        let uris: Vec<&str> = uris.into_iter().collect();
        let params = HashMap::from([
            ("uris", uris)
        ]);

        helper::post(&self.client, &endpoint, &params)
            .await.map(|r| r.data())
    }

    pub async fn remove_items_from_playlist(&self, playlist_id: &str, uris: impl IntoIterator<Item = &str>) -> Result<()> {
        let endpoint = format!("playlists/{}/items", playlist_id);

        // Map each &str to a json object
        let items: Vec<Value> = uris
            .into_iter()
            .map(|uri| json!({ "uri": uri }))
            .collect();

        // Let json! convert the array for us
        helper::delete(&self.client, &endpoint, &json!({ "items": items }))
            .await.map(|r| r.data())
    }

    pub async fn save_items_to_library(&self, uris: impl IntoIterator<Item = &str>) -> Result<()> {
        let uris = uris.into_iter().collect::<Vec<_>>().join(",");
        let url = format!("me/library?uris={}", uris);

        helper::put(&self.client, &url , &HashMap::<&str, &str>::new())
            .await.map(|r| r.data())
    }

    pub async fn remove_items_from_library(&self, uris: impl IntoIterator<Item = &str>) -> Result<()> {
        let uris = uris.into_iter().collect::<Vec<_>>().join(",");
        let url = format!("me/library?uris={}", uris);

        helper::delete(&self.client, &url , &HashMap::<&str, &str>::new())
            .await.map(|r| r.data())
    }

    pub async fn get_album(&self, id: &str) -> Result<Album> {
        let endpoint = format!("albums/{}", id);

        helper::get(&self.client, &endpoint, &HashMap::new())
            .await.map(|r| r.data())
            .context(format!("Failed to fetch album with id: {}", id))
    }

    pub async fn get_user_liked_songs(&self, limit: u32, offset: u32) -> Result<Page<Track>> {
        let limit = limit.clamp(1, 50).to_string();
        let offset = offset.to_string();
        let params = HashMap::<&str, &str>::from([
            ("limit", limit.as_str()),
            ("offset", offset.as_str())
        ]);

        // We will only get the item for now and ignore the added datetime
        #[derive(Deserialize)]
        struct SavedTrack {
            track: Track,
        }

        let res: Page<SavedTrack> = helper::get(&self.client, "me/tracks", &params).await?.data();

        // Destructure to get items and metadata
        let Page { items, total, offset, limit, next, after } = res;

        Ok(Page {
            items: items.into_iter().map(|st| st.track).collect(),
            total,
            offset,
            limit,
            next,
            after,
        })
    }

    pub async fn get_user_saved_albums(&self, limit: u32, offset: u32) -> Result<Page<Album>> {
        let limit = limit.clamp(1, 50).to_string();
        let offset = offset.to_string();
        let params = HashMap::<&str, &str>::from([
            ("limit", limit.as_str()),
            ("offset", offset.as_str())
        ]);

        #[derive(Deserialize)]
        struct SavedAlbum {
            album: Album,
        }

        let res: Page<SavedAlbum> = helper::get(&self.client, "me/albums", &params).await?.data();

        let Page { items, total, offset, limit, next, after } = res;

        Ok(Page {
            items: items.into_iter().map(|st| st.album).collect(),
            total,
            offset,
            limit,
            next,
            after,
        })
    }

    pub async fn get_user_saved_artists(&self, limit: u32, after: Option<&str>) -> Result<Page<Artist>> {
        let limit = limit.clamp(1, 50).to_string();
        let mut params = HashMap::<&str, &str>::from([
            ("type", "artist"),
            ("limit", limit.as_str()),
        ]);
        if let Some(ref cursor) = after {
            params.insert("after", cursor);
        }
        
        #[derive(Deserialize, Default)]
        struct FollowedArtists {
            artists: Page<Artist>,
        }

        let res: FollowedArtists = helper::get(&self.client, "me/following", &params).await?.data();
        
        Ok(res.artists)
    }

    pub async fn get_user_saved_podcasts(&self, limit: u32, offset: u32) -> Result<Page<Episode>> {
        let limit = limit.clamp(1, 50).to_string();
        let offset = offset.to_string();
        let params = HashMap::<&str, &str>::from([
            ("limit", limit.as_str()),
            ("offset", offset.as_str())
        ]);

        #[derive(Deserialize)]
        struct SavedEpisode {
            episode: Episode,
        }

        let res: Page<SavedEpisode> = helper::get(&self.client, "me/episodes", &params).await?.data();

        let Page { items, total, offset, limit, next, after } = res;

        Ok(Page {
            items: items.into_iter().map(|st| st.episode).collect(),
            total,
            offset,
            limit,
            next,
            after,
        })
    }
}
