use serde::{Deserialize, Deserializer, de::DeserializeOwned};
use serde_json::Value;
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

// ------------------------------------------- User -------------------------------------
#[derive(Debug, Deserialize, Clone)]
pub struct User {
    pub id: String,
    pub uri: String,
    pub display_name: String,

}

impl Default for User {
    fn default() -> Self {
        Self { id: String::new(), uri: String::new(), display_name: String::from("User") }
    }
}

// ---------------------------------- Simplified Album -------------------------------------
#[derive(Debug, Deserialize, Clone, Default)]
pub struct SimplifiedAlbum {
    pub id: String,
    pub name: String,
    pub album_type: String,
}

// ----------------------------------------- Item ------------------------------------------
#[derive(Debug, Deserialize, Clone, Default)]
pub struct Artist {
    pub id: String,
    pub uri: String,
    pub name: String,
    pub genres: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct Album {
    pub id: String,
    pub uri: String,
    pub name: String,
    pub artists: Vec<Artist>,
    pub release_date: Option<String>,
    pub tracks: Option<Page<Track>>,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct Track {
    pub id: String,
    pub uri: String,
    pub name: String,
    pub artists: Vec<Artist>,
    #[serde(default)]
    pub album: Option<SimplifiedAlbum>,
    #[serde(with = "duration_ms", rename = "duration_ms")]
    pub duration: Duration,
    pub explicit: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Episode {
    pub id: String,
    pub uri: String,
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

#[derive(Debug, Deserialize, Clone, Default)]
pub struct Playlist {
    pub id: String,
    pub uri: String,
    pub name: String,
    pub collaborative: bool,
    #[serde(default)]
    pub owner: User,
    pub description: Option<String>,
    pub items: Option<Page<PlayableItem>>,
}

// ------------------------------------- Playback -----------------------------------
#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum RepeatState {
    #[default]
    Off,
    Track,
    Context
}

impl RepeatState {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Off => "off",
            Self::Track => "track",
            Self::Context => "context"
        }
    }
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct Device {
    pub is_active: bool,
    pub name: String,
    #[serde(rename = "volume_percent")]
    pub volume: u8
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct Playback {
    pub item: Option<PlayableItem>,
    pub is_playing: bool,
    #[serde(with = "duration_ms", rename = "progress_ms")]
    pub progress: Duration,
    pub device: Device,
    pub repeat_state: RepeatState,
    pub shuffle_state: bool,
}

// -------------------------------------- Playable Item ------------------------------
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

// ----------------------------------------- Search Item --------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SearchType {
    Track,
    Album,
    Artist,
    Playlist,
    Episode
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct SearchResult {
    pub playlists: Option<Page<Playlist>>,
    pub albums: Option<Page<Album>>,
    pub artists: Option<Page<Artist>>,
    pub tracks: Option<Page<Track>>,
    pub episodes: Option<Page<Episode>>,
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

// ------------------------------------------- Queue Response -----------------------------
#[derive(Debug, Deserialize, Clone)]
pub struct QueueResponse {
    pub currently_playing: Option<PlayableItem>,
    pub queue: Vec<PlayableItem>,
}

// ------------------------------------- Action Menu Target ------------------------------
#[derive(Clone, Debug)]
pub enum MenuTarget {
    Track(Track),
    Artist(Artist),
    Album(Album),
    Playlist(Playlist),
    Episode(Episode),
}

// -------------------------------------------- Page --------------------------------------
// Supports both offset-based and cursor-based pagination
#[derive(Debug, Clone)]
pub struct Page<T> {
    pub items: Vec<T>,
    pub total: Option<u32>,
    pub offset: Option<u32>,
    pub limit: u32,
    pub next: Option<String>,
    pub after: Option<String>,
}

impl<T> Default for Page<T> {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            total: None,
            offset: None,
            limit: 20,
            next: None,
            after: None,
        }
    }
}

// Custom deserialization logic to handle spotify's inconsistency where the page
// is nested inside different layers
impl<'de, T: DeserializeOwned> Deserialize<'de> for Page<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let val = Value::deserialize(deserializer)?;

        let items = val.get("items")
            .and_then(Value::as_array)
            .map(|arr| {
                arr.iter()
                    .filter_map(|item| serde_json::from_value(item.clone()).ok())
                    .collect()
            })
            .unwrap_or_default();

        let after = val.get("cursors")
            .and_then(|c| c.get("after"))
            .and_then(Value::as_str)
            .map(String::from);

        Ok(Page {
            items,
            total: val.get("total").and_then(Value::as_u64).map(|v| v as u32),
            offset: val.get("offset").and_then(Value::as_u64).map(|v| v as u32),
            limit: val.get("limit").and_then(Value::as_u64).unwrap_or(20) as u32,
            next: val.get("next").and_then(Value::as_str).map(String::from),
            after,
        })
    }
}