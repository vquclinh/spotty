use crate::{app::types::StatefulList, network::models::*};

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
    
    pub results: SearchResult,

    pub tracks_state: StatefulList,
    pub artists_state: StatefulList,
    pub albums_state: StatefulList,
    pub playlists_state: StatefulList,

    pub hovered_pane: SearchHoveredPane,
}

impl SearchState {
    pub fn new() -> Self {
        Self {
            input: String::new(),

            results: SearchResult::default(),

            tracks_state: StatefulList::default(),
            artists_state: StatefulList::default(),
            albums_state: StatefulList::default(),
            playlists_state: StatefulList::default(),
            hovered_pane: SearchHoveredPane::Input,
        }
    }
}

impl  Default for SearchState {
    fn default() -> Self {
        Self::new()
    }
}
