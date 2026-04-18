use rspotify::{
    model::CurrentPlaybackContext,
    model::PlayableItem,
    model::enums::misc::RepeatState,
};

use serde::Deserialize;

use anyhow::{Result, bail};
use std::time::Duration;
use serde_json::Value;

fn normalize_duration(duration: chrono::Duration) -> Duration {
    // Make negative duration zero
    duration.to_std().unwrap_or(Duration::ZERO)
}

#[derive(Debug, Deserialize, Clone)]
pub struct Artist {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Album {
    pub id: String,
    pub name: String,
    pub artists: Vec<Artist>,
    pub release_date: String,
    pub tracks: Vec<Track>,
}

#[derive(Debug, Deserialize, Clone)]
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

#[derive(Debug, Deserialize, Clone)]
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
    pub fn parse(v: &Value) -> Result<Self> {
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
            progress: ctx.progress.map_or(Duration::ZERO, normalize_duration),
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

#[derive(Default, Debug, Clone)]
pub struct SearchResults {
    pub tracks: Vec<Track>,
    pub artists: Vec<Artist>,
    pub albums: Vec<Album>,
    pub playlists: Vec<Playlist>,
}