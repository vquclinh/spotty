use crate::network::models::*;
use crate::app::types::StatefulTable;

use ratatui::layout::Rect;

#[derive(Clone)]
pub struct PlaylistState {
    pub playlist: Playlist,
    pub tracks: StatefulTable<PlayableItem>,

    pub is_loading: bool,
    pub is_end: bool,
    pub last_area: Rect,
}

impl PlaylistState {
    pub fn new(playlist: Playlist) -> Self {
        Self {
            playlist,
            tracks: StatefulTable::new(),

            is_loading: false,
            is_end: false,
            last_area: Rect::default(),
        }
    }
}
