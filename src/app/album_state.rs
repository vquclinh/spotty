use crate::app::types::*;
use crate::network::models::*;
use ratatui::layout::Rect;

#[derive(Clone)]
pub struct AlbumState {
    pub album_id: String,
    pub album: Album,
    pub tracks: StatefulTable<Track>,

    pub is_loading: bool,
    pub is_end: bool,
    pub last_area: Rect,
}

impl Default for AlbumState {
    fn default() -> Self {
        Self {
            album_id: String::default(),
            album: Album::default(),
            tracks: StatefulTable::new(),

            is_loading: false,
            is_end: false,
            last_area: Rect::default(),
        }
    }
}

impl AlbumState {
    pub fn new(id: String) -> Self {
        Self {
            album_id: id,
            ..Self::default()
        }
    }
}
