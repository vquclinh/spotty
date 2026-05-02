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
pub struct SearchPane<T> {
    pub list: StatefulList<T>,
    pub is_loading: bool,
    pub is_end: bool,
}

impl<T> Default for SearchPane<T> {
    fn default() -> Self {
        Self {
            list: StatefulList::new(),
            is_loading: false,
            is_end: false,
        }
    }
}

#[derive(Clone)]
pub struct SearchState {
    pub input: String,

    pub tracks_state: SearchPane<Track>,
    pub artists_state: SearchPane<Artist>,
    pub albums_state: SearchPane<Album>,
    pub playlists_state: SearchPane<Playlist>,

    pub hovered_pane: SearchHoveredPane,

    pub last_area: Rect,
}

impl SearchState {
    pub fn new() -> Self {
        Self {
            input: String::new(),

            tracks_state: SearchPane::default(),
            artists_state: SearchPane::default(),
            albums_state: SearchPane::default(),
            playlists_state: SearchPane::default(),
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
