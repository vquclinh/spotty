use rspotify::{
    prelude::*,
    AuthCodePkceSpotify,
    model::CurrentPlaybackContext,
    model::PlayableItem,
    model::enums::misc::RepeatState,
    model::AdditionalType
};
use super::auth;
use anyhow::Result;
use std::collections::HashMap;
use std::time::{Instant, Duration};

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

    pub async fn get_playback_state(&self) -> Result<Option<PlaybackState>> {
        let ctx = self.client.current_playback(None, None::<Vec<_>>).await?;

        Ok(ctx.map(PlaybackState::from))
    }

}
