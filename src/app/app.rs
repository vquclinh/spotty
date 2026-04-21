use ratatui::widgets::ListState;
use tokio::sync::mpsc;

use crate::app::home_state::HomeTab;
use crate::app::splash_state::SplashState;
use crate::app::types::{ActionMenu, ActiveBlock, StatefulTable};
use crate::app::route::Route;
use crate::app::state::SharedState;
use crate::app::playbar_state::PlaybarState;

use crate::network::models::*;
use crate::network::request::ClientRequest;

pub struct App {
    pub route: Route,
    pub active_block: ActiveBlock,
    pub history: Vec<(Route, ActiveBlock)>, // store history about Route and ActiveBlock

    pub network_tx: mpsc::UnboundedSender<ClientRequest>, // the bridge between UI and Network
    pub shared_state: SharedState,

    pub playback: Option<Playback>,
    pub liked_songs: usize,
    pub playlists: StatefulTable<Playlist>,
    pub playbar_state: PlaybarState,

    // Tracks selection and scroll offset
    pub library_state: ListState,
    pub playlists_state: ListState,

    pub should_quit: bool, // Signal to quit main loop
    pub show_help: bool, // Signal to turn on pop-up help
    pub action_menu: ActionMenu,
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
            playlists: StatefulTable::new(),

            library_state: ListState::default(),
            playlists_state: ListState::default(),
            playbar_state: PlaybarState::default(),

            action_menu: ActionMenu::new(),
        }
    }

    pub fn set_current_route(&mut self, route: Route) {
        self.history.push((self.route.clone(), self.active_block.clone()));

        #[allow(clippy::single_match)]
        match &route {
            Route::Home(state) => {
                match state.active_tab {
                    HomeTab::RecentlyPlayed => {
                        let _ = self.network_tx.send(ClientRequest::GetRecentlyPlayed { limit: 15, offset: 0 });
                    }
                    HomeTab::TopTracks => {
                        let _ = self.network_tx.send(ClientRequest::GetUserTopTracks { time_range: TimeRange::ShortTerm, limit: 15, offset: 0 });
                    }
                    HomeTab::TopArtists => {
                        let _ = self.network_tx.send(ClientRequest::GetUserTopArtists { time_range: TimeRange::ShortTerm, limit: 15, offset: 0 });
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

            if !shared_state.playlists.is_empty() {
                self.playlists.items = shared_state.playlists.drain(..).collect();
            }

            match &mut self.route {
                Route::Home(home_state) => {

                    if !shared_state.recent_tracks.is_empty() {
                        home_state.recent_tracks.items = shared_state.recent_tracks.drain(..).collect();
                    }

                    if !shared_state.top_tracks.is_empty() {
                        home_state.top_tracks.items = shared_state.top_tracks.drain(..).collect();
                    }

                    if !shared_state.top_artists.is_empty() {
                        home_state.top_artists.items = shared_state.top_artists.drain(..).collect();
                    }
                }

                Route::PlaylistDetail(playlist_state) => {
                    if !shared_state.playlist_items.is_empty() {
                        playlist_state.tracks.items = shared_state.playlist_items.drain(..).collect();
                    }
                }

                Route::Search(search_state) => {
                    let has_tracks = shared_state.search_results.tracks.as_ref().is_some_and(|t| !t.items.is_empty());
                    let has_artists = shared_state.search_results.artists.as_ref().is_some_and(|a| !a.items.is_empty());

                    if has_tracks || has_artists {
                        search_state.results = std::mem::take(&mut shared_state.search_results);
                    }
                }

                _ => {}
            }
        }
    }
}
