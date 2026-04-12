use ratatui::widgets::ListState;
use crate::app::types::Track;

#[derive(Clone)]
pub struct SearchState {
    pub input: String,
    pub results_artists: Vec<String>,
    pub results_tracks: Vec<Track>,
    pub results_state: ListState,
}

impl SearchState {
    pub fn new() -> Self {
        Self {
            input: String::new(),
            results_artists: vec![],
            results_tracks: vec![],
            results_state: ListState::default(),
        }
    }
}

impl Default for SearchState {
    fn default() -> Self {
        Self::new()
    }
}