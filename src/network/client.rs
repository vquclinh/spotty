use rspotify::{
    prelude::*,
    AuthCodePkceSpotify,
    model::idtypes::PlaylistId,
    model::PlayableItem,
    model::TimeRange
};
use super::auth;
use anyhow::{Result};
use std::collections::HashMap;
use std::time::{Instant, Duration};
use serde_json::Value;

use super::models::{Track, Artist, Album, Playlist, PlaybackState, UserProfile};

// -------------------------------------- CACHE ---------------------------------------
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

    pub async fn get_user_profile(&self) -> Result<UserProfile> {
        let profile = self.client.me().await?;

        Ok(UserProfile {
            id: profile.id.to_string(),
            display_name: profile.display_name.unwrap_or(String::from("User"))
        })

    }

    // pub async fn get_playback_state(&self) -> Result<Option<PlaybackState>> {
    //     let ctx = self.client.current_playback(None, None::<Vec<_>>).await?;

    //     Ok(ctx.map(PlaybackState::from))
    // }

    // Parse manually because rspotify cannot detect track
    pub async fn get_playback_state(&self) -> Result<Option<PlaybackState>> {
        let endpoint = "me/player";
        let params = HashMap::<&str, &str>::new();

        let json = self.client.api_get(endpoint, &params).await?;

        if json.is_empty() {
            return Ok(None);
        }

        let v: Value = serde_json::from_str(&json)?;
        Ok(PlaybackState::parse(&v).ok())
    }

    pub async fn get_user_playlists(&self) -> Result<Vec<Playlist>> {
        // Collects first page of playlists
        let page = self.client.current_user_playlists_manual(None, None).await?;
        let playlists = page.items
            .into_iter()
            .map(|p| Playlist {
                id: p.id.to_string(),
                name: p.name,
                owner: p.owner.display_name.unwrap_or_else(|| "Unknown".to_string()),
                description: String::new(),
                tracks: Vec::new(),
            })
            .collect();
        Ok(playlists)
    }

    pub async fn get_queue(&self) -> Result<Vec<Track>> {
        let rspotify_queue = self.client.current_user_queue().await?;

        let tracks = rspotify_queue
            .queue
            .into_iter()
            .filter_map(|item| match item {
                PlayableItem::Track(t) => Some(Track::from(t)),
                PlayableItem::Episode(_) | PlayableItem::Unknown(_) => None,
            })
            .collect();

        Ok(tracks)
    }

    pub async fn get_playlist_tracks(&self, id: &str, limit: Option<u32>, offset: Option<u32>) 
    -> Result<Vec<Track>> {
        let playlist_id = PlaylistId::from_id_or_uri(id)
            .map_err(|e| anyhow::anyhow!("Invalid Playlist ID or URI: {}", e))?;

        let page = self.client
            .playlist_items_manual(playlist_id, None, None, limit, offset)
            .await?;

        let tracks = page.items
            .into_iter()
            .filter_map(|item| {
                item.item.and_then(|playable| match playable {
                    PlayableItem::Track(t) => Some(Track::from(t)),
                    _ => None, // Episodes and Unknown types are ignored
                })
            })
            .collect();

        Ok(tracks)
    }

    // get user top tracks in 4 months (ShortTerm)
    pub async fn get_user_top_tracks (&self, limit: u32) -> Result<Vec<Track>> {
        let page = self.client.current_user_top_tracks_manual(
            Some(TimeRange::ShortTerm),
            Some(limit),
            None
        ).await?;
        
        let tracks = page.items
            .into_iter()
            .map(|t| {
                let artists = t.artists.into_iter().map(|a| Artist {
                    id: a.id.map(|id| id.to_string()).unwrap_or_default(),
                    name: a.name,
                    genres: None,
                }).collect();

                Track {
                    id: t.id.map(|id| id.to_string()).unwrap_or_default(),
                    name: t.name,
                    artists,
                    album_name: t.album.name,
                    duration: Duration::from_millis(t.duration.num_milliseconds() as u64),
                    explicit: t.explicit,
                }
            }).collect();

        
        Ok(tracks)
    }

    // using rspotify to get raw data and then we handle this data
    // especially handle the "external_ids"
    pub async fn get_recently_played(&self, limit: u32) -> Result<Vec<Track>> {
        let endpoint = "me/player/recently-played";
        let mut params = HashMap::new();
        let limit_str = limit.to_string();
        params.insert("limit", limit_str.as_str());

        // get json
        let json_str = self.client.api_get(endpoint, &params).await?;
        let v: Value = serde_json::from_str(&json_str)?;

        let mut tracks = Vec::new();

        // go through each items
        if let Some(items) = v["items"].as_array() {
            for item in items {
                if let Some(track_val) = item.get("track") {
                    
                    // get id, name, explicit, album name, duration
                    // safe because we have a case None for each info
                    let id = track_val["id"].as_str().unwrap_or("").to_string();
                    let name = track_val["name"].as_str().unwrap_or("Unknown Track").to_string();
                    let explicit = track_val["explicit"].as_bool().unwrap_or(false);

                    let album_name = track_val["album"]["name"]
                        .as_str()
                        .unwrap_or("Unknown Album")
                        .to_string();

                    let duration_ms = track_val["duration_ms"].as_u64().unwrap_or(0);
                    let duration = Duration::from_millis(duration_ms);
                    
                    let mut artists = Vec::new();
                    if let Some(artists_array) = track_val["artists"].as_array() {
                        for artist_val in artists_array {
                            let artist_name = artist_val["name"].as_str().unwrap_or("Unknown Artist").to_string();
                            let artist_id = artist_val["id"].as_str().unwrap_or("").to_string();
                            
                            artists.push(Artist {
                                id: artist_id,
                                name: artist_name,
                                genres: None,
                            });
                        }
                    }

                    tracks.push(Track {
                        id,
                        name,
                        artists,
                        album_name,
                        duration,
                        explicit,
                    });
                }
            }
        }

        Ok(tracks)
    }

    pub async fn toggle_playback(&mut self, playing: bool) -> Result<()> {
        if playing {
            self.client.pause_playback(None).await?
        } else {
            self.client.resume_playback(None, None).await?
        }
        Ok(())
    }

    pub async fn next_track(&mut self) -> Result<()> {
        self.client.next_track(None).await?;
        Ok(())
    }

    pub async fn prev_track(&mut self) -> Result<()> {
        self.client.previous_track(None).await?;
        Ok(())
    }


}
