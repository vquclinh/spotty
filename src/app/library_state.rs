use crate::network::models::*;
use crate::app::types::StatefulTable;
use ratatui::layout::Rect;

#[derive(Clone)]
pub enum LibraryMenuItem {
    LikedSongs(LikedSongsState),
    Artists,
    Albums(SavedAlbumsState),
    Podcasts
}

#[derive(Clone)]
pub struct LikedSongsState {
    pub tracks: StatefulTable<Track>,

    pub last_area: Rect,
}

impl LikedSongsState {
    pub fn new(tracks: Vec<Track>) -> Self {
        Self {
            tracks: StatefulTable::with_items(tracks),

            last_area: Rect::default(),
        }
    }
}

#[derive(Clone)]
pub struct SavedAlbumsState {
    pub albums: StatefulTable<Album>,

    pub last_area: Rect,
}

impl SavedAlbumsState {
    pub fn new(albums: Vec<Album>) -> Self {
        Self {
            albums: StatefulTable::with_items(albums),

            last_area: Rect::default(),
        }
    }
}
