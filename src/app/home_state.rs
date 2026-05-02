use crate::app::types::{StatefulTable};
use crate::network::models::{Artist, Track};

use ratatui::layout::Rect;

#[derive(Clone, PartialEq, Default)]
pub enum HomeTab {
    TopTracks,
    TopArtists,
    #[default]
    RecentlyPlayed,
}

#[derive(Clone)]
pub struct HomePane<T> {
    pub list: StatefulTable<T>,
    pub is_loading: bool,
    pub is_end: bool,
}

impl<T> Default for HomePane<T> {
    fn default() -> Self {
        Self {
            list: StatefulTable::new(),
            is_loading: false,
            is_end: false,
        }
    }
}

#[derive(Clone)]
pub struct HomeState {
    pub greeting: String,
    pub active_tab: HomeTab,

    pub top_tracks: HomePane<Track>,
    pub top_artists: HomePane<Artist>,
    pub recent_tracks: HomePane<Track>,

    pub last_area: Rect,
}

impl Default for HomeState {
    fn default() -> Self {
        Self {
            greeting: "Good Morning".to_string(),
            active_tab: HomeTab::TopTracks,

            top_tracks: HomePane::default(),
            top_artists: HomePane::default(),
            recent_tracks: HomePane::default(),

            last_area: Rect::default(),
        }
    }
}

impl HomeState {
    pub fn new() -> Self {
        Self::default()
    }
}
