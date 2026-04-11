use ratatui::widgets::ListState;

use crate::app::splash_state::SplashState;
use crate::app::types::{Track, PlayerState, ActiveBlock};
use crate::app::route::Route;
use crate::app::home_state::HomeState;

pub struct App {
    pub route: Route,
    pub active_block: ActiveBlock,
    pub history: Vec<(Route, ActiveBlock)>,

    pub should_quit: bool,
    pub show_help: bool,

    pub player: PlayerState,

    pub liked_songs: usize,
    pub playlists: Vec<String>,
    pub library_state: ListState,
    pub playlists_state: ListState,
}

impl App {
    pub fn new() -> Self {
        let dummy_track = Track {
            title: "Making My Way".to_string(),
            artist: "Son Tung MTP".to_string(),
            album: "Single".to_string(),
        };

        Self {
            route: Route::Splash(SplashState::new()),
            active_block: ActiveBlock::PlaylistsMenu,
            history: vec![],
            show_help: false,
            should_quit: false,

            player: PlayerState {
                is_playing: true,
                current_track: Some(dummy_track.clone()),
                queue: vec![],
            },
            
            liked_songs: 152,
            playlists: vec![
                "Lofi Chill".to_string(), 
                "Gym".to_string(), 
                "Top Hits 2024".to_string()
            ],

            library_state: ListState::default(),
            playlists_state: ListState::default(),
        }
    }

    pub fn on_tick(&mut self) {
        if let Some(next_route) = self.route.update() {
            self.route = next_route;
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}