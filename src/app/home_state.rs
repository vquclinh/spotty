use crate::app::types::{StatefulTable};
use crate::network::models::{Artist, Track};

use ratatui::layout::Rect;

#[derive(Clone, PartialEq)]
pub enum HomeTab {
    TopTracks,
    TopArtists,
    RecentlyPlayed,
}

#[derive(Clone)]
pub struct HomeState {
    pub greeting: String,
    pub active_tab: HomeTab,

    pub top_tracks: StatefulTable<Track>,
    pub top_artists: StatefulTable<Artist>,
    pub recent_tracks: StatefulTable<Track>,

    pub last_area: Rect,
}

impl HomeState {
    pub fn new() -> Self {
        Self {
            greeting: "Good Morning".to_string(),
            active_tab: HomeTab::TopTracks,
            top_tracks: StatefulTable::new(),
            top_artists: StatefulTable::new(),
            recent_tracks: StatefulTable::new(),

            last_area: Rect::default(),
        }
    }
}

impl Default for HomeState {
    fn default() -> Self {
        Self::new()
    }
}