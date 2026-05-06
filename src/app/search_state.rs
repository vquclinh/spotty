use crate::{app::types::StatefulList, network::models::*};
use ratatui::layout::Rect;

#[derive(Clone, PartialEq, Default)]
pub enum SearchHoveredPane {
    #[default]
    Input,
    Tracks,
    Artists,
    Albums,
    Playlists,
}

#[derive(Clone)]
pub struct SearchState {
    pub input: String,

    pub tracks_state: StatefulList<Track>,
    pub artists_state: StatefulList<Artist>,
    pub albums_state: StatefulList<Album>,
    pub playlists_state: StatefulList<Playlist>,

    pub hovered_pane: SearchHoveredPane,

    pub last_area: Rect,
}

impl SearchState {
    pub fn new() -> Self {
        Self {
            input: String::new(),

            tracks_state: StatefulList::default(),
            artists_state: StatefulList::default(),
            albums_state: StatefulList::default(),
            playlists_state: StatefulList::default(),
            hovered_pane: SearchHoveredPane::Input,

            last_area: Rect::default(),
        }
    }
}

impl  Default for SearchState {
    fn default() -> Self {
        Self::new()
    }
}
