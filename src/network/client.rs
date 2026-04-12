use rspotify::{
    prelude::*,
    AuthCodePkceSpotify,
    model::CurrentPlaybackContext,
    model::PlayableItem,
    model::enums::misc::RepeatState,
};
use super::auth;
use anyhow::{Result, anyhow, bail};
use std::collections::HashMap;
use std::time::{Instant, Duration};
use serde_json::Value;

fn normalize_duration(duration: chrono::Duration) -> Duration {
    // Make negative duration zero
    duration.to_std().unwrap_or(Duration::ZERO)
}

pub struct Artist {
    pub id: String,
    pub name: String,
    pub genres: Option<Vec<String>>,
}

pub struct Album {
    pub id: String,
    pub name: String,
    pub artists: Vec<Artist>,
    pub release_date: String,
    pub tracks: Vec<Track>,
}

pub struct Track {
    pub id: String,
    pub name: String,
    pub artists: Vec<Artist>,
    pub album_name: String,
    pub duration: Duration,
    pub explicit: bool,
}

impl Track {
    fn parse(v: &Value) -> Result<Self> {
        if v["type"].as_str() != Some("track") {
            bail!("Item is not a track");
        }

        let name = v["name"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing track name"))?
            .to_string();

        let id = v["id"]
            .as_str()
            .unwrap_or_default()
            .to_string();

        let album_name = v["album"]["name"]
            .as_str()
            .unwrap_or("Unknown Album")
            .to_string();

        let duration_ms = v["duration_ms"]
            .as_u64()
            .ok_or_else(|| anyhow::anyhow!("Missing duration"))?;

        let explicit = v["explicit"].as_bool().unwrap_or(false);

        let artists = v["artists"]
            .as_array()
            .ok_or_else(|| anyhow::anyhow!("Missing artists array"))?
            .iter()
            .map(|a| Artist {
                id: a["id"].as_str().unwrap_or_default().to_string(),
                name: a["name"].as_str().unwrap_or("Unknown Artist").to_string(),
                genres: None,
            })
            .collect();

        Ok(Self {
            id,
            name,
            album_name,
            artists,
            duration: Duration::from_millis(duration_ms),
            explicit,
        })
    }
}

impl From<rspotify::model::FullTrack> for Track {
    fn from(t: rspotify::model::FullTrack) -> Self {
        Self {
            id: t.id.map(|id| id.to_string()).unwrap_or_default(),
            name: t.name,
            album_name: t.album.name,
            artists: t.artists.into_iter().map(|a| Artist {
                id: a.id.map(|id| id.to_string()).unwrap_or_default(),
                name: a.name,
                genres: None,
            }).collect(),
            duration: normalize_duration(t.duration),
            explicit: t.explicit,
        }
    }
}

pub struct Episode {
    pub id: String,
    pub name: String,
    pub description: String,
    pub show_name: String,
    pub release_date: String,
    pub duration: Duration,
    pub resume_point: Duration, // where the user left off
    pub explicit: bool,
    pub is_externally_hosted: bool,
}

impl Episode {
    pub fn parse(v: &Value) -> Result<Self> {
        if v["type"].as_str() != Some("episode") {
            return Err(anyhow::anyhow!("Item is not an episode"));
        }

        let id = v["id"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing episode id"))?
            .to_string();

        let name = v["name"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing episode name"))?
            .to_string();

        let description = v["description"]
            .as_str()
            .unwrap_or_default()
            .to_string();

        let show_name = v["show"]["name"]
            .as_str()
            .unwrap_or("Unknown Show")
            .to_string();

        let release_date = v["release_date"]
            .as_str()
            .unwrap_or_default()
            .to_string();

        let duration_ms = v["duration_ms"]
            .as_u64()
            .ok_or_else(|| anyhow::anyhow!("Missing episode duration"))?;

        // Extract resume position if it exists, otherwise default to zero
        let resume_ms = v["resume_point"]["resume_position_ms"]
            .as_u64()
            .unwrap_or(0);

        let explicit = v["explicit"].as_bool().unwrap_or(false);
        let is_externally_hosted = v["is_externally_hosted"].as_bool().unwrap_or(false);

        Ok(Self {
            id,
            name,
            description,
            show_name,
            release_date,
            duration: Duration::from_millis(duration_ms),
            resume_point: Duration::from_millis(resume_ms),
            explicit,
            is_externally_hosted,
        })
    }
}

impl From<rspotify::model::FullEpisode> for Episode {
    fn from(e: rspotify::model::FullEpisode) -> Self {
        Self {
            id: e.id.to_string(),
            name: e.name,
            description: e.description,
            show_name: e.show.name,
            release_date: e.release_date,
            duration: normalize_duration(e.duration),
            resume_point: e.resume_point.map_or(Duration::ZERO, |rp| {
                if rp.fully_played { Duration::ZERO } else { normalize_duration(rp.resume_position) }
            }),
            explicit: e.explicit,
            is_externally_hosted: e.is_externally_hosted,
        }
    }
}

pub struct Playlist {
    pub id: String,
    pub name: String,
    pub owner: String,
    pub description: String,
    pub tracks: Vec<Track>,
}

pub struct PlaybackState {
    pub item: Option<Playable>,
    pub is_playing: bool,
    pub progress: Duration,
    pub device_name: String,
    pub repeat_state: RepeatState, // Off, Track, Context
    pub shuffle_state: bool,
}

impl PlaybackState {
    fn parse(v: &Value) -> Result<Self> {
        let item_json = &v["item"];
        let item = if item_json.is_null() {
            None
        } else {
            match item_json["type"].as_str() {
                Some("track") => Some(Playable::Track(Track::parse(item_json)?)),
                Some("episode") => Some(Playable::Episode(Episode::parse(item_json)?)),
                _ => None,
            }
        };

        Ok(Self {
            item,
            is_playing: v["is_playing"].as_bool().unwrap_or(false),
            progress: Duration::from_millis(v["progress_ms"].as_u64().unwrap_or(0)),
            device_name: v["device"]["name"].as_str().unwrap_or("Unknown").to_string(),
            repeat_state: match v["repeat_state"].as_str().unwrap_or("off") {
                "track" => RepeatState::Track,
                "context" => RepeatState::Context,
                _ => RepeatState::Off,
            },
            shuffle_state: v["shuffle_state"].as_bool().unwrap_or(false),
        })
    }
}

impl From<CurrentPlaybackContext> for PlaybackState {
    fn from(ctx: CurrentPlaybackContext) -> Self {
        Self {
            item: match ctx.item {
                Some(PlayableItem::Track(t)) => Some(Playable::Track(t.into())),
                Some(PlayableItem::Episode(e)) => Some(Playable::Episode(e.into())),
                Some(PlayableItem::Unknown(_)) => {
                    println!("Unknown item detected");
                    None
                },
                None => None
            },
            is_playing: ctx.is_playing,
            progress: ctx.progress.map_or(Duration::ZERO, |d| normalize_duration(d)),
            device_name: ctx.device.name,
            repeat_state: match ctx.repeat_state {
                RepeatState::Off => RepeatState::Off,
                RepeatState::Track => RepeatState::Track,
                RepeatState::Context => RepeatState::Context,
            },
            shuffle_state: ctx.shuffle_state,
        }
    }
}

pub struct UserProfile {
    pub id: String,
    pub display_name: String,

}

impl Default for UserProfile {
    fn default() -> Self {
        Self { id: String::new(), display_name: String::from("User") }
    }
}

pub enum Playable {
    Track(Track),
    Episode(Episode)
}

pub struct Searchable {
    pub tracks: Vec<Track>,
    pub albums: Vec<Album>,
    pub artists: Vec<Artist>,
    pub playlists: Vec<Playlist>,
    pub episodes: Vec<Episode>,
}

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
            cache: cache_ttl_sec.map_or(Cache::default(), |s| Cache::new(s))
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

    // pub async fn get_recently_played(&self, limit: u32) -> Result<Vec<Track>> {
    //     let history = self.client.current_user_recently_played(Some(limit), None).await?;
    //     let tracks = history.items
    //         .into_iter()
    //         .map(|h| Track::from(h.track))
    //         .collect();
    //     Ok(tracks)
    // }

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
