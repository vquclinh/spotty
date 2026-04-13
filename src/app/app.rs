use ratatui::widgets::ListState;
use tokio::sync::mpsc;

use crate::app::splash_state::SplashState;
use crate::app::types::ActiveBlock;
use crate::app::route::Route;

use crate::network::models::{Playlist, PlaybackState};
use crate::network::request::ClientRequest;

pub struct App {
    pub route: Route,
    pub active_block: ActiveBlock,
    pub history: Vec<(Route, ActiveBlock)>,

    pub network_tx: mpsc::UnboundedSender<ClientRequest>,

    pub playback: Option<PlaybackState>,
    pub liked_songs: usize,
    pub playlists: Vec<Playlist>,

    pub library_state: ListState,
    pub playlists_state: ListState,

    pub should_quit: bool,
    pub show_help: bool,
}

impl App {
    pub fn new(network_tx: mpsc::UnboundedSender<ClientRequest>) -> Self {
        let _ = network_tx.send(ClientRequest::GetCurrentPlayback);
        let _ = network_tx.send(ClientRequest::GetUserPlaylists);
        
        Self {
            route: Route::Splash(SplashState::new()),
            active_block: ActiveBlock::PlaylistsMenu,
            history: vec![],
            show_help: false,
            should_quit: false,

            network_tx,

            playback: None,
            liked_songs: 0,
            playlists: vec![],

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