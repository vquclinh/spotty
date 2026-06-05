use tokio::sync::mpsc;
use std::time::Duration;
use rspotify::model::TrackId;
use rspotify::prelude::Id;

use super::home_state::HomeTab;
use super::splash_state::SplashState;
use super::playbar_state::PlaybarState;
use super::device_state::DeviceState;
use super::types::{
    ActionMenu, ActiveBlock, PlaylistSelector,
    StatefulList, StatefulTable, QuickAction
};
use super::route::Route;
use super::state::SharedState;
use super::library_state::*;
use super::state::DataPayload;
use super::cache::AppCache;
use super::{AppState, StateHistory};

use crate::network::models::*;
use crate::network::request::ClientRequest;

use crate::audio::events::*;

// Global/non route-specific data will be stored in app
pub struct App {
    pub state: StateHistory,
    pub app_cache: AppCache,
    pub device_state: DeviceState,
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
    pub show_quick_actions: bool,
    pub show_device_selector: bool,
    pub action_menu: ActionMenu,
    pub playlist_selector: PlaylistSelector,
    pub quick_actions: Vec<QuickAction>,

    pub playbar: PlaybarState,

    pub track_ended: bool,
    pub pending_device_claim: bool,
}

impl App {
    pub const APP_CACHE_PATH: &str = ".spotty_cache/app_cache.json";

    pub fn new(
        app_cache: AppCache,
        network_tx: mpsc::UnboundedSender<ClientRequest>,
        audio_event_rx: mpsc::UnboundedReceiver<AudioEvent>,
        shared_state: SharedState,
    ) -> Self {
        let page_limit = 50;

        // At initialization, send a request to get current playback and playlists
        let _ = network_tx.send(ClientRequest::GetCurrentUser);
        let _ = network_tx.send(ClientRequest::GetCurrentPlayback);
        let _ = network_tx.send(ClientRequest::GetUserPlaylists {
            limit: page_limit,
            offset: 0
        });

        let initial_state = AppState::new(
            Route::Splash(SplashState::new()),
            ActiveBlock::LibraryMenu
        );

        Self {
            state: StateHistory::new(initial_state),
            app_cache,
            device_state: DeviceState::default(),
            page_limit,

            show_help: false,
            show_quick_actions: false,
            show_device_selector: false,
            should_quit: false,

            network_tx,
            audio_event_rx,
            shared_state,

            user: User::default(),

            playback: None,
            // Initialize the items we want to have in the library menu
            library_menu: StatefulList::with_items(vec![
                LibraryMenuItem::LikedSongs(LikedSongsState::default()),
                LibraryMenuItem::SavedArtists(SavedArtistsState::default()),
                LibraryMenuItem::SavedAlbums(SavedAlbumsState::default()),
                LibraryMenuItem::SavedPodcasts(SavedPodcastsState::default())
            ]),
            playlists_menu: StatefulTable::new(),

            action_menu: ActionMenu::new(),
            playlist_selector: PlaylistSelector::new(),
            quick_actions: Vec::new(),

            playbar: PlaybarState::new(),

            track_ended: false,
            pending_device_claim: false,
        }
    }

    pub fn set_app_state(&mut self, route: Route, active_block: Option<ActiveBlock>) {
        match &route {
            Route::Home(state) => {
                match state.active_tab {
                    HomeTab::RecentlyPlayed => {
                        let _ = self.network_tx.send(ClientRequest::GetRecentlyPlayed {
                            limit: self.page_limit,
                            after: None
                        });
                    }
                    HomeTab::TopTracks => {
                        let _ = self.network_tx.send(ClientRequest::GetUserTopTracks {
                            time_range: TimeRange::ShortTerm,
                            limit: self.page_limit,
                            offset: 0
                        });
                    }
                    HomeTab::TopArtists => {
                        let _ = self.network_tx.send(ClientRequest::GetUserTopArtists {
                            time_range: TimeRange::ShortTerm,
                            limit: self.page_limit,
                            offset: 0
                        });
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
            Route::PlaylistDetail(state) => {
                let id = state.playlist.id.clone();
                let _ = self.network_tx.send(ClientRequest::GetPlaylistItems {
                    playlist_id: id,
                    limit: self.page_limit,
                    offset: 0
                });
            }
            Route::LikedSongs(_) => {
                let _ = self.network_tx.send(ClientRequest::GetUserLikedSongs {
                    limit: self.page_limit,
                    offset: 0
                });
            }
            Route::SavedAlbums(_) => {
                let _ = self.network_tx.send(ClientRequest::GetUserSavedAlbums {
                    limit: self.page_limit,
                    offset: 0
                });
            }
            Route::SavedArtists(_) => {
                let _ = self.network_tx.send(ClientRequest::GetUserSavedArtists {
                    limit: self.page_limit,
                    after: None
                });
            }
            Route::SavedPodcasts(_) => {
                let _ = self.network_tx.send(ClientRequest::GetUserSavedPodcasts {
                    limit: self.page_limit,
                    offset: 0
                });
            }
            Route::Lyrics(_) => {
                if let Some(playback) = &self.playback {
                    if let Some(PlayableItem::Track(track)) = &playback.item {
                        let track_id = track.id.clone();
                        let _ = self.network_tx.send(ClientRequest::GetLyrics { track_id });
                    }
                }
            }
            _ => {}
        }

        self.state.set_state(route, active_block);
    }

    // tick in main loop
    pub fn on_tick(&mut self, tick_rate: Duration) {
        // update route
        if let Some(next_route) = self.state.current_mut().route.update() {
            self.set_app_state(next_route, None);
        }

        // increment lyrics animation tick
        if let Route::Lyrics(state) = &mut self.state.current_mut().route {
            state.tick = state.tick.wrapping_add(1);
        }
        
        // update progress
        while let Ok(event) = self.audio_event_rx.try_recv() {
            match event {
                AudioEvent::Changed { uri } => {
                    self.track_ended = false;
                    if let Some(pb) = &mut self.playback {
                        pb.progress = Duration::from_millis(0);

                        if let Route::Queue(_) = self.state.current().route {
                            let _ = self.network_tx.send(ClientRequest::GetQueue);
                        }

                        if let Route::Lyrics(_) = self.state.current().route
                        && let Ok(track_id) = TrackId::from_uri(uri.as_str()) {
                            let track_id = track_id.id().to_string();
                            let _ = self.network_tx.send(ClientRequest::GetLyrics { track_id });
                        }
                    }
                }

                AudioEvent::Playing { position_ms, .. } => {
                    self.track_ended = false;
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
                    self.track_ended = true;
                    if let Some(pb) = &mut self.playback {
                        pb.is_playing = false;
                    }
                }

                AudioEvent::VolumeChanged { volume } => {
                    if let Some(pb) = &mut self.playback {
                        self.app_cache.volume = volume;
                        pb.device.volume = volume;
                    }
                }

                AudioEvent::ShuffleChanged { shuffle } => {
                    if let Some(pb) = &mut self.playback
                    && let Some(context_uri) = &pb.context_uri {
                        self.app_cache.shuffle_state.insert(context_uri.to_string(), shuffle);
                        pb.shuffle_state = shuffle;
                    }
                }

                AudioEvent::RepeatChanged { repeat } => {
                    if let Some(pb) = &mut self.playback {
                        pb.repeat_state = repeat;
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

            if let Some(mut playback) = shared_state.playback.take() {
                if self.track_ended {
                    playback.is_playing = false;
                }
                self.playback = Some(playback);
            }

            if let Some(device_state) = shared_state.devices.take() {
                self.device_state = device_state;
                
                // Claim the playback if there is no active device
                if self.device_state.active_device_id().is_some() {
                    self.pending_device_claim = false;
                }
                else if !self.pending_device_claim {
                    self.pending_device_claim = true;
                    
                    let id_opt = self.device_state.local_device_id();
                    let _ = self.network_tx.send(ClientRequest::TransferPlayback {
                        device_id: id_opt.clone(),
                        should_play: false
                    });
                    
                    if let Some(id) = id_opt {
                        self.device_state.set_active_device_optimistic(&id);
                    }
                    let net_tx = self.network_tx.clone();
                    tokio::spawn(async move {
                        tokio::time::sleep(Duration::from_millis(400)).await;
                        let _ = net_tx.send(ClientRequest::GetDevices);
                    });
                }
            }

            if !shared_state.playlists.items.is_empty() {
                self.playlists_menu.items = shared_state.playlists.items.drain(..).collect();
            }

            #[allow(clippy::collapsible_match)]
            match &mut self.state.current_mut().route {
                Route::Home(home_state) => {

                    if !shared_state.recent_tracks.items.is_empty() {
                        assign_or_append_payload(
                            &mut home_state.recent_tracks.list.items,
                            &mut shared_state.recent_tracks
                        );
                        home_state.recent_tracks.is_loading = false;
                        home_state.recent_tracks.is_end = shared_state.recent_tracks.is_end
                            || home_state.recent_tracks.list.items.len() >= 100;
                    }

                    if !shared_state.top_tracks.items.is_empty() {
                        assign_or_append_payload(
                            &mut home_state.top_tracks.list.items,
                            &mut shared_state.top_tracks
                        );
                        home_state.top_tracks.is_loading = false;
                        home_state.top_tracks.is_end = shared_state.top_tracks.is_end
                            || home_state.top_tracks.list.items.len() >= 100;
                    }

                    if !shared_state.top_artists.items.is_empty() {
                        assign_or_append_payload(
                            &mut home_state.top_artists.list.items,
                            &mut shared_state.top_artists
                        );
                        home_state.top_artists.is_loading = false;
                        home_state.top_artists.is_end = shared_state.top_artists.is_end
                            || home_state.top_artists.list.items.len() >= 100;
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
                        album_state.album = album;
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

                Route::Lyrics(lyrics_state) => {
                    if let Some(lyrics) = shared_state.lyrics_data.take() {
                        lyrics_state.data = Some(lyrics);
                        lyrics_state.is_loading = false;
                    }
                }

                _ => {}
            }
        }
    }

    pub fn update_quick_actions(&mut self) {
        let actions = &mut self.quick_actions;
        actions.clear();
        match &self.state.current().route {
            Route::PlaylistDetail(s) => {
                actions.push(QuickAction::PlayContext);
                if self.app_cache.shuffle(&s.playlist.uri) {
                    actions.push(QuickAction::UnshuffleContext);
                } else {
                    actions.push(QuickAction::ShuffleContext);
                }
            }
            Route::AlbumDetail(s) => {
                actions.push(QuickAction::PlayContext);
                actions.push(QuickAction::SaveContext);
                if self.app_cache.shuffle(&s.album.uri) {
                    actions.push(QuickAction::UnshuffleContext);
                } else {
                    actions.push(QuickAction::ShuffleContext);
                }
            }
            _ => {}
        }
        actions.push(QuickAction::TransferPlayback);
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
