use ratatui::widgets::ListState;
use crate::network::models::SearchResults;

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
    
    pub results: SearchResults,

    pub tracks_state: ListState,
    pub artists_state: ListState,
    pub albums_state: ListState,
    pub playlists_state: ListState,

    pub hovered_pane: SearchHoveredPane,
}

impl SearchState {
    pub fn new() -> Self {
        Self {
            input: String::new(),

            results: SearchResults::default(),

            tracks_state: ListState::default(),
            artists_state: ListState::default(),
            albums_state: ListState::default(),
            playlists_state: ListState::default(),
            hovered_pane: SearchHoveredPane::Input,
        }
    }
}

impl  Default for SearchState {
    fn default() -> Self {
        Self::new()
    }
}