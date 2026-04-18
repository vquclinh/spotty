use serde::Deserialize;
use std::time::Duration;

mod duration_ms {
    use serde::{Deserialize, Deserializer};
    use std::time::Duration;

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
    where
        D: Deserializer<'de>,
    {
        let ms = u64::deserialize(deserializer)?;
        Ok(Duration::from_millis(ms))
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Artist {
    pub id: String,
    pub name: String,
    pub genres: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Album {
    pub id: String,
    pub name: String,
    pub artists: Vec<Artist>,
    pub release_date: Option<String>,
    #[serde(default)]
    pub tracks: Vec<Track>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Track {
    pub id: String,
    pub name: String,
    pub artists: Vec<Artist>,
    #[serde(default)]
    pub album_name: String, 
    #[serde(with = "duration_ms", rename = "duration_ms")]
    pub duration: Duration,
    pub explicit: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Episode {
    pub id: String,
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub show_name: String,
    pub release_date: String,
    #[serde(with = "duration_ms", rename = "duration_ms")]
    pub duration: Duration,
    #[serde(default, with = "duration_ms", rename = "resume_position_ms")]
    pub resume_point: Duration,
    pub explicit: bool,
    pub is_externally_hosted: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Playlist {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub owner: User,
    pub description: String,
    #[serde(default)]
    pub items: Vec<PlayableItem>,
}

#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum RepeatState {
    Off,
    Track,
    Context
}

#[derive(Debug, Deserialize, Clone)]
pub struct Playback {
    pub item: Option<PlayableItem>,
    pub is_playing: bool,
    #[serde(with = "duration_ms", rename = "progress_ms")]
    pub progress: Duration,
    #[serde(default)]
    pub device_name: String,
    pub repeat_state: RepeatState,
    pub shuffle_state: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct User {
    pub id: String,
    pub display_name: String,

}

impl Default for User {
    fn default() -> Self {
        Self { id: String::new(), display_name: String::from("User") }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayableType {
    Track,
    Episode
}

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum PlayableItem {
    Track(Track),
    Episode(Episode),
}

impl PlayableItem {
    pub fn name(&self) -> &str {
        match self {
            PlayableItem::Track(i) => i.name.as_str(),
            PlayableItem::Episode(i) => i.name.as_str()
        }
    }

    pub fn artists(&self) -> String {
        match self {
            PlayableItem::Track(i) => {
                if i.artists.is_empty() {
                    return "Unknown".to_string();
                }

                i.artists
                    .iter()
                    .map(|a| a.name.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            }
            PlayableItem::Episode(i) => {
                if i.name.is_empty() {
                    "Unknown".to_string()
                } else {
                    i.name.clone()
                }
            }
        }
}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SearchType {
    Track,
    Album,
    Artist,
    Playlist,
    Episode
}

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum SearchItem {
    Track(Track),
    Album(Album),
    Artist(Artist),
    Playlist(Playlist),
    Episode(Episode),
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct SearchResult {
    pub playlists: Option<Vec<Playlist>>,
    pub albums: Option<Vec<Album>>,
    pub artists: Option<Vec<Artist>>,
    pub tracks: Option<Vec<Track>>,
    pub episodes: Option<Vec<Episode>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeRange {
    ShortTerm,
    MediumTerm,
    LongTerm
}

impl TimeRange {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ShortTerm => "short_term",
            Self::MediumTerm => "medium_term",
            Self::LongTerm => "long_term"
        }
    }
}
