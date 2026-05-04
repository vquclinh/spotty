use tokio::sync::mpsc;
use std::time::Duration;

use crate::app::home_state::HomeTab;
use crate::app::splash_state::SplashState;
use crate::app::types::{ActionMenu, ActiveBlock, PlaylistSelector, StatefulList, StatefulTable};
use crate::app::route::Route;
use crate::app::state::SharedState;
use crate::app::library_state::*;
use crate::app::state::DataPayload;

use crate::network::models::*;
use crate::network::request::ClientRequest;

use crate::audio::events::*;

// Global/non route-specific data will be stored in app
pub struct App {
    pub route: Route,
    pub active_block: ActiveBlock,
    pub history: Vec<(Route, ActiveBlock)>, // store history about Route and ActiveBlock
    // Limit for each Spotify Web Api page fetch
    pub page_limit: u32,

    pub network_tx: mpsc::UnboundedSender<ClientRequest>, // the bridge between UI and Network
    pub audio_event_rx: mpsc::UnboundedReceiver<AudioEvent>,
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
        audio_event_rx: mpsc::UnboundedReceiver<AudioEvent>,
        shared_state: SharedState,
    ) -> Self {
        let page_limit = 50;

        // At initialization, send a request to get current playback and playlists
        let _ = network_tx.send(ClientRequest::GetCurrentUser);
        let _ = network_tx.send(ClientRequest::GetCurrentPlayback);
        let _ = network_tx.send(ClientRequest::GetUserPlaylists { limit: page_limit, offset: 0 });

        Self {
            route: Route::Splash(SplashState::new()),
            active_block: ActiveBlock::LibraryMenu,
            history: vec![],
            page_limit,

            show_help: false,
            should_quit: false,

            network_tx,
            audio_event_rx,
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
                        let _ = self.network_tx.send(ClientRequest::GetRecentlyPlayed { limit: self.page_limit, after: None });
                    }
                    HomeTab::TopTracks => {
                        let _ = self.network_tx.send(ClientRequest::GetUserTopTracks { time_range: TimeRange::ShortTerm, limit: self.page_limit, offset: 0 });
                    }
                    HomeTab::TopArtists => {
                        let _ = self.network_tx.send(ClientRequest::GetUserTopArtists { time_range: TimeRange::ShortTerm, limit: self.page_limit, offset: 0 });
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
                let _ = self.network_tx.send(ClientRequest::GetUserLikedSongs { limit: self.page_limit, offset: 0 });
            }
            Route::SavedAlbums(_) => {
                let _ = self.network_tx.send(ClientRequest::GetUserSavedAlbums { limit: self.page_limit, offset: 0 });
            }
            Route::SavedArtists(_) => {
                let _ = self.network_tx.send(ClientRequest::GetUserSavedArtists { limit: self.page_limit, after: None });
            }
            Route::SavedPodcasts(_) => {
                let _ = self.network_tx.send(ClientRequest::GetUserSavedPodcasts { limit: self.page_limit, offset: 0 });
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

        // update progress
        while let Ok(event) = self.audio_event_rx.try_recv() {
            match event {
                AudioEvent::Changed { .. } => {
                    if let Some(pb) = &mut self.playback {
                        pb.progress = Duration::from_millis(0);
                    }
                }

                AudioEvent::Playing { position_ms, .. } => {
                    if let Some(pb) = &mut self.playback {
                        pb.is_playing = true;
                        pb.progress = Duration::from_millis(position_ms as u64);
                    }
                }

                AudioEvent::Paused { position_ms, .. } => {
                    if let Some(pb) = &mut self.playback {
                        pb.is_playing = false;
                        pb.progress = Duration::from_millis(position_ms as u64);
                    }
                }

                AudioEvent::EndOfTrack { .. } => {
                    if let Some(pb) = &mut self.playback {
                        pb.is_playing = false;
                    }
                }
            }
        }

        if let Some(playback) = &mut self.playback {
            if playback.is_playing {
                playback.progress += tick_rate;

                if let Some(PlayableItem::Track(t)) = &playback.item {
                    if playback.progress > t.duration {
                        playback.progress = t.duration;
                    }
                }
            }
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

            #[allow(clippy::collapsible_match)]
            match &mut self.route {
                Route::Home(home_state) => {

                    if !shared_state.recent_tracks.items.is_empty() {
                        assign_or_append_payload(
                            &mut home_state.recent_tracks.list.items,
                            &mut shared_state.recent_tracks
                        );
                    }

                    if !shared_state.top_tracks.items.is_empty() {
                        assign_or_append_payload(
                            &mut home_state.top_tracks.list.items,
                            &mut shared_state.top_tracks
                        );
                        home_state.top_tracks.is_loading = false;
                        home_state.top_tracks.is_end = shared_state.top_tracks.is_end;
                    }

                    if !shared_state.top_artists.items.is_empty() {
                        assign_or_append_payload(
                            &mut home_state.top_artists.list.items,
                            &mut shared_state.top_artists
                        );
                        home_state.top_artists.is_loading = false;
                        home_state.top_artists.is_end = shared_state.top_artists.is_end;
                    }
                }

                Route::PlaylistDetail(playlist_state) if !shared_state.playlist_items.items.is_empty() => {
                    assign_or_append_payload(
                        &mut playlist_state.tracks.items,
                        &mut shared_state.playlist_items
                    );
                    playlist_state.is_loading = false;
                    playlist_state.is_end = shared_state.playlist_items.is_end;
                }

                Route::Search(search_state) => {
                    let results = &mut shared_state.search_results;

                    if let Some(mut page) = results.tracks.take() && !page.items.is_empty() {
                        if page.offset.unwrap_or(0) == 0 {
                            search_state.tracks_state.items = std::mem::take(&mut page.items);
                        } else {
                            search_state.tracks_state.items.append(&mut page.items);
                        }
                    }

                    if let Some(mut page) = results.artists.take() && !page.items.is_empty() {
                        if page.offset.unwrap_or(0) == 0 {
                            search_state.artists_state.items = std::mem::take(&mut page.items);
                        } else {
                            search_state.artists_state.items.append(&mut page.items);
                        }
                    }

                    if let Some(mut page) = results.albums.take() && !page.items.is_empty() {
                        if page.offset.unwrap_or(0) == 0 {
                            search_state.albums_state.items = std::mem::take(&mut page.items);
                        } else {
                            search_state.albums_state.items.append(&mut page.items);
                        }
                    }

                    if let Some(mut page) = results.playlists.take() && !page.items.is_empty() {
                        if page.offset.unwrap_or(0) == 0 {
                            search_state.playlists_state.items = std::mem::take(&mut page.items);
                        } else {
                            search_state.playlists_state.items.append(&mut page.items);
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
                        if let Some(ref tracks) = album.tracks {
                            album_state.tracks.items = tracks.items.clone();
                        }
                        album_state.album = Some(album);
                    }
                }

                Route::LikedSongs(liked_songs_state) => {
                    if !shared_state.liked_songs.items.is_empty() {
                        assign_or_append_payload(
                            &mut liked_songs_state.tracks.items,
                            &mut shared_state.liked_songs
                        );
                        liked_songs_state.is_loading = false;
                        liked_songs_state.is_end = shared_state.liked_songs.is_end;
                    }
                }

                Route::SavedAlbums(saved_albums_state) => {
                    if !shared_state.saved_albums.items.is_empty() {
                        assign_or_append_payload(
                            &mut saved_albums_state.albums.items,
                            &mut shared_state.saved_albums
                        );
                        saved_albums_state.is_loading = false;
                        saved_albums_state.is_end = shared_state.saved_albums.is_end;
                    }
                }

                Route::SavedArtists(saved_artists_state) => {
                    if !shared_state.saved_artists.items.is_empty() {
                        assign_or_append_payload(
                            &mut saved_artists_state.artists.items,
                            &mut shared_state.saved_artists
                        );
                        saved_artists_state.is_loading = false;
                        saved_artists_state.is_end = shared_state.saved_artists.is_end;
                    }
                }

                Route::SavedPodcasts(saved_podcasts_state) => {
                    if !shared_state.saved_podcasts.items.is_empty() {
                        assign_or_append_payload(
                            &mut saved_podcasts_state.podcasts.items,
                            &mut shared_state.saved_podcasts
                        );
                        saved_podcasts_state.is_loading = false;
                        saved_podcasts_state.is_end = shared_state.saved_podcasts.is_end;
                    }
                }

                _ => {}
            }
        }
    }
}

fn assign_or_append_payload<T>(into: &mut Vec<T>, payload: &mut DataPayload<T>) {
    if payload.items.is_empty() {
        return;
    }

    if payload.should_append {
        into.append(&mut payload.items);
    } else {
        *into = std::mem::take(&mut payload.items);
    }
}
