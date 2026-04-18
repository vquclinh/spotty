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
    pub tracks: Option<Page<Track>>,
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
    pub description: Option<String>,
    pub items: Option<Page<PlayableItem>>,
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

#[derive(Debug, Clone)]
pub struct Page<T> {
    pub items: Vec<T>,
    pub total: u32,
    pub offset: u32,
    pub limit: u32,
    pub next: Option<String>,
}

impl<T> Default for Page<T> {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            total: 0,
            offset: 0,
            limit: 10,
            next: None,
        }
    }
}

impl<'de, T: DeserializeOwned> Deserialize<'de> for Page<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let val = Value::deserialize(deserializer)?;

        // Safely extract the array. filter_map silently discards any `null` elements
        let items = val.get("items")
            .and_then(Value::as_array)
            .map(|arr| {
                arr.iter()
                    .filter_map(|item| serde_json::from_value(item.clone()).ok())
                    .collect()
            })
            .unwrap_or_default();

        Ok(Page {
            items,
            total: val.get("total").and_then(Value::as_u64).unwrap_or(0) as u32,
            offset: val.get("offset").and_then(Value::as_u64).unwrap_or(0) as u32,
            limit: val.get("limit").and_then(Value::as_u64).unwrap_or(0) as u32,
            next: val.get("next").and_then(Value::as_str).map(String::from),
        })
    }
}
