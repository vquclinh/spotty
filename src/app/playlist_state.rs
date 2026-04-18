use crate::network::models::*;
use crate::app::types::StatefulTable;

#[derive(Clone)]
pub struct PlaylistState {
    pub playlist: Playlist,
    pub tracks: StatefulTable<PlayableItem>,
}

impl PlaylistState {
    pub fn new(playlist: Playlist) -> Self {
        Self {
            playlist,
            tracks: StatefulTable::new(),
        }
    }
}
