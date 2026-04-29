use tokio::sync::mpsc;
use std::time::Duration;

use crate::app::home_state::HomeTab;
use crate::app::splash_state::SplashState;
use crate::app::types::{ActionMenu, ActiveBlock, PlaylistSelector, StatefulList, StatefulTable};
use crate::app::route::Route;
use crate::app::state::SharedState;
use crate::app::library_state::*;

use crate::network::models::*;
use crate::network::request::ClientRequest;

// Global/non route-specific data will be stored in app
pub struct App {
    pub route: Route,
    pub active_block: ActiveBlock,
    pub history: Vec<(Route, ActiveBlock)>, // store history about Route and ActiveBlock

    pub network_tx: mpsc::UnboundedSender<ClientRequest>, // the bridge between UI and Network
    pub shared_state: SharedState,

    pub user: User,
    pub playback: Option<Playback>,
    pub library_menu: StatefulList<LibraryMenuItem>,
    pub playlists_menu: StatefulTable<Playlist>,

    pub should_quit: bool, // Signal to quit main loop
    pub show_help: bool, // Signal to turn on pop-up help
    pub action_menu: ActionMenu,
    pub playlist_selector: PlaylistSelector,
}

impl App {
    pub fn new(
        network_tx: mpsc::UnboundedSender<ClientRequest>,
        shared_state: SharedState,
    ) -> Self {
        // At initialization, send a request to get current playback and playlists
        let _ = network_tx.send(ClientRequest::GetCurrentUser);
        let _ = network_tx.send(ClientRequest::GetCurrentPlayback);
        let _ = network_tx.send(ClientRequest::GetUserPlaylists { limit: 50, offset: 0 });

        Self {
            route: Route::Splash(SplashState::new()),
            active_block: ActiveBlock::LibraryMenu,
            history: vec![],
            show_help: false,
            should_quit: false,

            network_tx,
            shared_state,

            user: User::default(),

            playback: None,
            // Initialize the items we want to have in the library menu
            library_menu: StatefulList::with_items(vec![
                LibraryMenuItem::LikedSongs(LikedSongsState::new(vec![])),
                LibraryMenuItem::SavedArtists(SavedArtistsState::new(vec![])),
                LibraryMenuItem::SavedAlbums(SavedAlbumsState::new(vec![])),
                LibraryMenuItem::SavedPodcasts(SavedPodcastsState::new(vec![]))
            ]),
            playlists_menu: StatefulTable::new(),

            action_menu: ActionMenu::new(),
            playlist_selector: PlaylistSelector::new(),
        }
    }

    pub fn set_current_route(&mut self, route: Route) {
        self.history.push((self.route.clone(), self.active_block.clone()));

        #[allow(clippy::single_match)]
        match &route {
            Route::Home(state) => {
                match state.active_tab {
                    HomeTab::RecentlyPlayed => {
                        let _ = self.network_tx.send(ClientRequest::GetRecentlyPlayed { limit: 15, after: None });
                    }
                    HomeTab::TopTracks => {
                        let _ = self.network_tx.send(ClientRequest::GetUserTopTracks { time_range: TimeRange::ShortTerm, limit: 15, offset: 0 });
                    }
                    HomeTab::TopArtists => {
                        let _ = self.network_tx.send(ClientRequest::GetUserTopArtists { time_range: TimeRange::ShortTerm, limit: 15, offset: 0 });
                    }
                }
            }
            Route::Queue(_) => {
                let _ = self.network_tx.send(ClientRequest::GetQueue);
            }
            Route::AlbumDetail(state) => {
                let id = state.album_id.clone();
                let _ = self.network_tx.send(ClientRequest::GetAlbum { id });
            }
            Route::LikedSongs(_) => {
                let _ = self.network_tx.send(ClientRequest::GetUserLikedSongs { limit: 50, offset: 0 });
            }
            Route::SavedAlbums(_) => {
                let _ = self.network_tx.send(ClientRequest::GetUserSavedAlbums { limit: 50, offset: 0 });
            }
            Route::SavedArtists(_) => {
                let _ = self.network_tx.send(ClientRequest::GetUserSavedArtists { limit: 50, after: None });
            }
            Route::SavedPodcasts(_) => {
                let _ = self.network_tx.send(ClientRequest::GetUserSavedPodcasts { limit: 50, offset: 0 });
            }
            _ => {}
        }

        self.route = route;
    }

    // tick in main loop
    pub fn on_tick(&mut self, tick_rate: Duration) {
        // update route
        if let Some(next_route) = self.route.update() {
            self.set_current_route(next_route);
        }

        // Increment progress locally
        if let Some(playback) = &mut self.playback && playback.is_playing {
            playback.progress += tick_rate;
        }

        self.sync_data();
    }

    fn sync_data(&mut self) {
        if let Ok(mut shared_state) = self.shared_state.lock() {

            if self.user.id.is_empty() && !shared_state.user.id.is_empty() {
                self.user = shared_state.user.clone();
            }

            if let Some(playback) = shared_state.playback.take() {
                self.playback = Some(playback);
            }

            if !shared_state.playlists.items.is_empty() {
                self.playlists_menu.items = shared_state.playlists.items.drain(..).collect();
            }

            match &mut self.route {
                Route::Home(home_state) => {

                    if !shared_state.recent_tracks.items.is_empty() {
                        home_state.recent_tracks.items = shared_state.recent_tracks.items.drain(..).collect();
                    }

                    if !shared_state.top_tracks.items.is_empty() {
                        home_state.top_tracks.items = shared_state.top_tracks.items.drain(..).collect();
                    }

                    if !shared_state.top_artists.items.is_empty() {
                        home_state.top_artists.items = shared_state.top_artists.items.drain(..).collect();
                    }
                }

                Route::PlaylistDetail(playlist_state) => {
                    if !shared_state.playlist_items.items.is_empty() {
                        playlist_state.tracks.items = shared_state.playlist_items.items.drain(..).collect();
                    }
                }

                Route::Search(search_state) => {
                    let results = &mut shared_state.search_results;

                    let has_tracks = results.tracks.as_ref().is_some_and(|t| !t.items.is_empty());
                    let has_artists = results.artists.as_ref().is_some_and(|a| !a.items.is_empty());

                    if has_tracks || has_artists {
                        if let Some(page) = results.tracks.take() {
                            search_state.tracks_state.items = page.items;
                        }
                        if let Some(page) = results.artists.take() {
                            search_state.artists_state.items = page.items;
                        }
                        if let Some(page) = results.albums.take() {
                            search_state.albums_state.items = page.items;
                        }
                        if let Some(page) = results.playlists.take() {
                            search_state.playlists_state.items = page.items;
                        }
                    }
                }

                Route::Queue(queue_state) => {
                    if let Some(pb) = &self.playback && let Some(item) = &pb.item {
                        queue_state.currently_playing = Some(item.clone());
                    }

                    if let Some((current, items)) = shared_state.queue_data.take() {
                        if let Some(c) = current {
                            queue_state.currently_playing = Some(c);
                        }
                        queue_state.queue_items.items = items;

                        if queue_state.queue_items.state.selected().is_none() && !queue_state.queue_items.items.is_empty() {
                            queue_state.queue_items.state.select(Some(0));
                        }
                    }
                }

                Route::AlbumDetail(album_state) => {
                    if let Some(album) = shared_state.album_detail.take() {
                        album_state.album = Some(album.clone());

                        if let Some(page) = album.tracks {
                            album_state.tracks.items = page.items;
                            if album_state.tracks.state.selected().is_none() && !album_state.tracks.items.is_empty() {
                                album_state.tracks.state.select(Some(0));
                            }
                        }
                    }
                }

                Route::LikedSongs(like_songs_state) => {
                    if !shared_state.liked_songs.items.is_empty() {
                        like_songs_state.tracks.items = shared_state.liked_songs.items.drain(..).collect();
                    }
                }

                Route::SavedAlbums(saved_albums_state) => {
                    if !shared_state.saved_albums.items.is_empty() {
                        saved_albums_state.albums.items = shared_state.saved_albums.items.drain(..).collect();
                    }
                }

                Route::SavedArtists(saved_artists_state) => {
                    if !shared_state.saved_artists.items.is_empty() {
                        saved_artists_state.artists.items = shared_state.saved_artists.items.drain(..).collect();
                    }
                }

                Route::SavedPodcasts(saved_podcasts_state) => {
                    if !shared_state.saved_podcasts.items.is_empty() {
                        saved_podcasts_state.podcasts.items = shared_state.saved_podcasts.items.drain(..).collect();
                    }
                }

                _ => {}
            }
        }
    }
}
