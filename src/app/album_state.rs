use crate::app::types::*;
use crate::network::models::*;
use ratatui::layout::Rect;

#[derive(Clone)]
pub struct AlbumState {
    pub album_id: String,
    pub album: Option<Album>,
    pub tracks: StatefulTable<Track>,
    pub last_area: Rect,
}

impl Default for AlbumState {
    fn default() -> Self {
        Self {
            album_id: String::default(),
            album: None,
            tracks: StatefulTable::new(),
            last_area: Rect::default(),
        }
    }
}

impl AlbumState {
    pub fn new(id: String) -> Self {
        Self {
            album_id: id,
            album: None,
            tracks: StatefulTable::default(),
            last_area: Rect::default(),
        }
    }
}