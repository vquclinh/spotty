use ratatui::widgets::ListState;
use tokio::sync::mpsc;

use crate::app::home_state::HomeTab;
use crate::app::splash_state::SplashState;
use crate::app::types::ActiveBlock;
use crate::app::route::Route;
use crate::app::state::SharedState;

use crate::network::models::{Playlist, PlaybackState};
use crate::network::request::ClientRequest;
use crate::ui::home;

pub struct App {
    pub route: Route,
    pub active_block: ActiveBlock, 
    pub history: Vec<(Route, ActiveBlock)>, // store history about Route and ActiveBlock

    pub network_tx: mpsc::UnboundedSender<ClientRequest>, // the bridge between UI and Network
    pub shared_state: SharedState,

    pub playback: Option<PlaybackState>,
    pub liked_songs: usize,
    pub playlists: Vec<Playlist>,

    // Tracks selection and scroll offset
    pub library_state: ListState,
    pub playlists_state: ListState,

    pub should_quit: bool, // Signal to quit main loop
    pub show_help: bool, // Signal to turn on pop-up help
}

impl App {
    pub fn new(
        network_tx: mpsc::UnboundedSender<ClientRequest>,
        shared_state: SharedState,
    ) -> Self {
        // At initialization, send a request to get current playback and playlists
        let _ = network_tx.send(ClientRequest::GetCurrentPlayback);
        let _ = network_tx.send(ClientRequest::GetUserPlaylists);
        
        Self {
            route: Route::Splash(SplashState::new()),
            active_block: ActiveBlock::PlaylistsMenu,
            history: vec![],
            show_help: false,
            should_quit: false,

            network_tx,
            shared_state,

            playback: None,
            liked_songs: 0,
            playlists: vec![],

            library_state: ListState::default(),
            playlists_state: ListState::default(),
        }
    }

    pub fn set_current_route(&mut self, route: Route) {
        self.history.push((self.route.clone(), self.active_block.clone()));

        #[allow(clippy::single_match)]
        match &route {
            Route::Home(state) => {
                match state.active_tab {
                    HomeTab::RecentlyPlayed => {
                        let _ = self.network_tx.send(ClientRequest::GetRecentlyPlayed { limit: 30 });
                    }
                    HomeTab::TopTracks => {
                        let _ = self.network_tx.send(ClientRequest::GetTopTracks { limit: 20 });
                    }
                    HomeTab::TopArtists => {
                        // TODO
                    }
                }
            }
            _ => {}
        }

        self.route = route;
    }

    // tick in main loop
    pub fn on_tick(&mut self) {
        // update route
        if let Some(next_route) = self.route.update() {
            self.set_current_route(next_route);
        }

        self.sync_data();
    }

    fn sync_data(&mut self) {
        if let Ok(mut shared_state) = self.shared_state.lock() {
            
            if shared_state.playback.is_some() {
                self.playback = shared_state.playback.take();
            }

            if let Route::Home(home_state) = &mut self.route {
                
                if !shared_state.recent_tracks.is_empty() {
                    home_state.recent_tracks.items = shared_state.recent_tracks.drain(..).collect();
                }

                if !shared_state.playlists.is_empty() {
                    self.playlists = shared_state.playlists.drain(..).collect();
                }

                if !shared_state.top_tracks.is_empty() {
                    home_state.top_tracks.items = shared_state.top_tracks.drain(..).collect();
                }
            }
        }
    }
}
