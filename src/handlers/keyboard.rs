use crate::app::{ActiveBlock, App, Route};
use crate::app::types::MenuAction;
use crate::app::album_state::AlbumState;
use super::{global, home, playlist};
use crate::handlers::{album, library, playbar, queue, search, sidebar, lyrics};
use crate::network::models::*;
use crate::network::request::{PlayerRequest, ClientRequest};

use crossterm::event::{KeyEvent, KeyCode};
use librespot_connect::{LoadRequestOptions, PlayingTrack};
use std::time::Duration;

pub fn handle_key_events(key: KeyEvent, app: &mut App) {
    if app.show_help {
        app.show_help = false;
        return;
    }

    let active_block = app.state.current().active_block;

    if app.show_quick_actions {
        // Handle global keybinds first, early return if success
        if match key.code {
            KeyCode::Char('t') if !app.show_device_selector => {
                let _ = app.network_tx.send(ClientRequest::GetDevices);
                app.show_device_selector = true;
                true
            }
            _ => false
        } {
            app.show_quick_actions = false;
            return;
        }

        if let Some(uri) = match &app.state.current().route {
            Route::AlbumDetail(s) => Some(s.album.uri.clone()),
            Route::PlaylistDetail(s) => Some(s.playlist.uri.clone()),
            _ => None
        } {
            match key.code {
                KeyCode::Char('s') => {
                    let shuffle = app.app_cache.shuffle(&uri);
                    app.app_cache.shuffle_state.insert(uri.clone(), !shuffle);
                    if let Some(pb) = &app.playback
                    && let Some(context_uri) = &pb.context_uri
                    && *context_uri == uri {
                        let _ = app.network_tx.send(ClientRequest::Player {
                            request: PlayerRequest::ToggleShuffle(shuffle),
                            is_active_device: app.device_state.is_active_device()
                        });
                    }
                }
                KeyCode::Char('l') => {
                    let _ = app.network_tx.send(
                        ClientRequest::SaveItemsToLibrary(vec![uri])
                    );
                }
                KeyCode::Enter => {
                    let _ = app.network_tx.send(ClientRequest::Player {
                        request: build_play_context_request(&uri, None, app),
                        is_active_device: app.device_state.is_active_device()
                    });
                }
                _ => {}
            }
        }
        app.show_quick_actions = false;
        return;
    }

    if app.playlist_selector.is_open {
        match key.code {
            KeyCode::Esc => {
                app.playlist_selector.close();
            }
            KeyCode::Down | KeyCode::Char('j') => {
                app.playlist_selector.next();
            }
            KeyCode::Up | KeyCode::Char('k') => {
                app.playlist_selector.previous();
            }
            KeyCode::Enter => {
                execute_add_to_playlist(app);
                app.playlist_selector.close();
                app.action_menu.close();
            }
            _ => {}
        }
        return;
    }

    if app.action_menu.is_open {
        match key.code {
            KeyCode::Esc | KeyCode::Char('t') => {
                app.action_menu.close();
            }
            KeyCode::Down | KeyCode::Char('j') => {
                app.action_menu.next();
            }
            KeyCode::Up | KeyCode::Char('k') => {
                app.action_menu.previous();
            }
            KeyCode::Enter => {
                if execute_action_menu_command(app) {
                    app.action_menu.close();
                }
            }
            _ => {}
        }
        return;
    }

    if app.show_device_selector {
        match key.code {
            KeyCode::Down | KeyCode::Char('j') => {
                app.device_state.online_devices.next(1, false);
            }
            KeyCode::Up | KeyCode::Char('k') => {
                app.device_state.online_devices.previous(1, false);
            }
            KeyCode::Enter => {
                let selecting_active_device = |selected_idx: usize| if let Some(idx) = app.device_state.active_device_idx() {
                    selected_idx == idx
                } else {
                    false
                };
                if let Some(idx) = app.device_state.online_devices.state.selected()
                && !selecting_active_device(idx) {
                    app.device_state.online_devices.state.select(Some(idx));
                    let id_opt = app.device_state.device_id_from_idx(idx);
                    let _ = app.network_tx.send(ClientRequest::TransferPlayback {
                        device_id: id_opt.clone(),
                        should_play: false
                    });
                    // Optimistic update -> Delayed fetch to ensure accurate device state
                    if let Some(id) = id_opt {
                        app.device_state.set_active_device_optimistic(&id);
                    }
                    let net_tx = app.network_tx.clone();
                    tokio::spawn(async move {
                        tokio::time::sleep(Duration::from_millis(400)).await;
                        let _ = net_tx.send(ClientRequest::GetDevices);
                    });
                    
                    app.show_device_selector = false;
                }
            }
            _ => {
                app.show_device_selector = false;
            }
        }
        return;
    }

    if active_block == ActiveBlock::SearchInput {
        search::handle_search_events(key, app);
        return;
    }
    // global keyboard
    if global::handle_global_events(key, app) {
        return;
    }

    // each active_block
    match active_block {
        ActiveBlock::PlaylistsMenu | ActiveBlock::LibraryMenu => {
            sidebar::handle_sidebar_events(key, app);
        }
        ActiveBlock::HomeBlock => {
            home::handle_home_events(key, app);
            // No return here to allow route-specific tab logic below
        }
        ActiveBlock::PlaylistTracks => {
            playlist::handle_playlist_events(key, app);
        }
        ActiveBlock::SearchResults => {
            search::handle_search_events(key, app);
        }
        ActiveBlock::QueueBlock => {
            queue::handle_queue_events(key, app);
        }
        ActiveBlock::AlbumBlock => {
            album::handle_album_events(key, app);
        }
        ActiveBlock::LikedSongs => {
            library::handle_liked_songs_events(key, app);
        }
        ActiveBlock::SavedAlbums => {
            library::handle_saved_albums_events(key, app);
        }
        ActiveBlock::SavedArtists => {
            library::handle_saved_artists_events(key, app);
        }
        ActiveBlock::SavedPodcasts => {
            library::handle_saved_podcasts_events(key, app);
        }
        ActiveBlock::Playbar => {
            playbar::handle_playbar_events(key, app);
        }
        ActiveBlock::LyricsText => {
            lyrics::handle_lyrics_events(key, app);
        }
        _ => {}
    }
}

fn execute_action_menu_command(app: &mut App) -> bool {
    let action = app.action_menu.state.selected()
        .and_then(|idx| app.action_menu.actions.get(idx).cloned());
    let target = app.action_menu.target.clone();

    if let (Some(action), Some(target)) = (action, target) {
        let uri = match &target {
            MenuTarget::Track(t) => Some(t.uri.clone()),
            MenuTarget::Episode(e) => Some(e.uri.clone()),
            MenuTarget::Artist(a) => Some(a.uri.clone()),
            MenuTarget::Album(a) => Some(a.uri.clone()),
            MenuTarget::Playlist(p) => Some(p.uri.clone()),
        };

        match action {
            MenuAction::PlayNow => handle_play_now_action(app, &target, uri),
            MenuAction::AddToQueue => handle_add_to_queue_action(app, uri),
            MenuAction::AddToPlaylist => handle_add_to_playlist_menu_action(app),
            MenuAction::RemoveFromThisPlaylist => handle_remove_from_playlist_action(app, uri),
            MenuAction::GoToAlbum => handle_go_to_album_action(app, &target),
            MenuAction::ViewDetails | MenuAction::GoToShow => true,
            MenuAction::SaveToLibrary => handle_save_to_library_action(app, &target, uri),
            MenuAction::RemoveFromLibrary => handle_remove_from_library_action(app, &target, uri),
            MenuAction::FollowArtist => handle_follow_artist_action(app, &target, uri),
            MenuAction::UnfollowArtist => handle_unfollow_artist_action(app, &target, uri),
        }
    } else {
        true
    }
}

fn handle_play_now_action(app: &mut App, target: &MenuTarget, uri: Option<String>) -> bool {
    if let Some(u) = uri {
        let player_req = {
            let route = &app.state.current().route;
            match target {
                MenuTarget::Track(_) | MenuTarget::Episode(_) => {
                    match route {
                        Route::PlaylistDetail(s) => build_play_context_request(
                            &s.playlist.uri,
                            Some(Offset::Uri(u.clone())),
                            app
                        ),
                        Route::AlbumDetail(s) => build_play_context_request(
                            &s.album.uri,
                            Some(Offset::Uri(u.clone())),
                            app
                        ),
                        _ => PlayerRequest::Play(u),
                    }
                }
                MenuTarget::Album(_) | MenuTarget::Playlist(_) | MenuTarget::Artist(_) => {
                    build_play_context_request(&u, None, app)
                }
            }
        };

        let _ = app.network_tx.send(ClientRequest::Player {
            request: player_req,
            is_active_device: app.device_state.is_active_device()
        });
        
        if matches!(app.state.current().route, Route::Queue(_)) {
            let _ = app.network_tx.send(ClientRequest::GetQueue);
        }
    }
    true
}

fn handle_add_to_queue_action(app: &mut App, uri: Option<String>) -> bool {
    if let Some(u) = uri {
        let _ = app.network_tx.send(ClientRequest::Player {
            request: PlayerRequest::AddItemToQueue(u),
            is_active_device: app.device_state.is_active_device()
        });
    }
    true
}

fn handle_add_to_playlist_menu_action(app: &mut App) -> bool {
    let my_id = &app.user.id;
    let writable_playlists: Vec<Playlist> = app.playlists_menu.items
        .iter()
        .filter(|p| p.owner.id == *my_id || p.collaborative)
        .cloned()
        .collect();

    if !writable_playlists.is_empty() {
        app.playlist_selector.playlists = writable_playlists;
        app.playlist_selector.is_open = true;
        app.playlist_selector.state.select(Some(0));
        false
    } else {
        true
    }
}

fn handle_remove_from_playlist_action(app: &mut App, uri: Option<String>) -> bool {
    if let Some(uri) = uri {
        if !uri.is_empty() {
            let route = &mut app.state.current_mut().route;
            if let Route::PlaylistDetail(route_state) = route {
                let _ = app.network_tx.send(ClientRequest::RemoveItemsFromPlaylist {
                    playlist_id: route_state.playlist.id.clone(), uris: vec![uri.clone()]
                });
                route_state.tracks.items.retain(|item| match item {
                    PlayableItem::Track(i) => i.uri != uri,
                    PlayableItem::Episode(i) => i.uri != uri
                });
                if let Some(idx) = route_state.tracks.state.selected() {
                    let len = route_state.tracks.items.len();
                    if idx >= len {
                        route_state.tracks.state.select(len.checked_sub(1));
                    }
                }
            }
        }
    }
    true
}

fn handle_go_to_album_action(app: &mut App, target: &MenuTarget) -> bool {
    let album_id = match target {
        MenuTarget::Track(t) => t.album.as_ref().map(|a| a.id.clone()),
        MenuTarget::Album(a) => Some(a.id.clone()),
        _ => None,
    };

    if let Some(id) = album_id {
        let new_album_state = AlbumState::new(id);
        app.set_app_state(
            Route::AlbumDetail(new_album_state),
            Some(ActiveBlock::AlbumBlock)
        );
    }
    true
}

fn handle_save_to_library_action(app: &mut App, target: &MenuTarget, uri: Option<String>) -> bool {
    if let Some(uri) = uri {
        if !uri.is_empty() && !matches!(target, MenuTarget::Playlist(_)) {
            let _ = app.network_tx.send(ClientRequest::SaveItemsToLibrary(vec![uri]));
        }
    }
    true
}

#[allow(clippy::collapsible_if)]
fn handle_remove_from_library_action(app: &mut App, target: &MenuTarget, uri: Option<String>) -> bool {
    if let Some(uri) = uri {
        if !uri.is_empty() {
            if matches!(target, MenuTarget::Track(_) | MenuTarget::Album(_) | MenuTarget::Artist(_) | MenuTarget::Episode(_)) {
                let _ = app.network_tx.send(ClientRequest::RemoveItemsFromLibrary(vec![uri.clone()]));
                
                let route = &mut app.state.current_mut().route;
                let state_info = match route {
                    Route::LikedSongs(s) => {
                        s.tracks.items.retain(|item| item.uri != uri);
                        Some((s.tracks.items.len(), &mut s.tracks.state))
                    }
                    Route::SavedAlbums(s) => {
                        s.albums.items.retain(|item| item.uri != uri);
                        Some((s.albums.items.len(), &mut s.albums.state))
                    }
                    Route::SavedArtists(s) => {
                        s.artists.items.retain(|item| item.uri != uri);
                        Some((s.artists.items.len(), &mut s.artists.state))
                    }
                    Route::SavedPodcasts(s) => {
                        s.podcasts.items.retain(|item| item.uri != uri);
                        Some((s.podcasts.items.len(), &mut s.podcasts.state))
                    }
                    _ => None
                };

                if let Some((len, list_state)) = state_info {
                    if let Some(idx) = list_state.selected() {
                        if idx >= len {
                            list_state.select(len.checked_sub(1));
                        }
                    }
                }
            }
        }
    }
    true
}

fn handle_follow_artist_action(app: &mut App, target: &MenuTarget, uri: Option<String>) -> bool {
    if let MenuTarget::Artist(_) = target {
        if let Some(uri) = uri {
            let _ = app.network_tx.send(ClientRequest::SaveItemsToLibrary(vec![uri]));
        }
    }
    true
}

fn handle_unfollow_artist_action(app: &mut App, target: &MenuTarget, uri: Option<String>) -> bool {
    if let MenuTarget::Artist(_) = target {
        if let Some(uri) = uri {
            let _ = app.network_tx.send(ClientRequest::RemoveItemsFromLibrary(vec![uri.clone()]));
            
            let route = &mut app.state.current_mut().route;
            if let Route::SavedArtists(s) = route {
                s.artists.items.retain(|item| item.uri != uri);
                if let Some(idx) = s.artists.state.selected() {
                    let len = s.artists.items.len();
                    if idx >= len {
                        s.artists.state.select(len.checked_sub(1));
                    }
                }
            }
        }
    }
    true
}

fn execute_add_to_playlist(app: &mut App) {
    if let (Some(idx), Some(target)) = (
        app.playlist_selector.state.selected(),
        &app.action_menu.target
    ) {
        let playlist_id = app.playlist_selector.playlists[idx].id.clone();
        let uri = match target {
            MenuTarget::Track(t) => Some(t.uri.clone()),
            MenuTarget::Episode(e) => Some(e.uri.clone()),
            _ => None,
        };

        if let Some(u) = uri {
            let _ = app.network_tx.send(ClientRequest::AddItemsToPlaylist {
                playlist_id,
                uris: vec![u],
            });
        }
    }
}

fn build_play_context_request(context_uri: &str, offset: Option<Offset>, app: &App) -> PlayerRequest {
    let shuffle = app.app_cache.shuffle(context_uri);
    let context_options = app.playback.as_ref().map(|pb| {
        pb.to_librespot_options(shuffle)
    });
    let playing_track = match offset {
        Some(Offset::Index(i)) => Some(PlayingTrack::Index(i)),
        Some(Offset::Uri(u)) => Some(PlayingTrack::Uri(u)),
        // If we are not active device, then we simulate a shuffle context play
        // request by randomizing the start track.
        None if shuffle => {
            let total = match &app.state.current().route {
                Route::AlbumDetail(s) => s.album.total(),
                Route::PlaylistDetail(s) => s.playlist.total(),
                _ => None,
            };
            total.map(|t| PlayingTrack::Index(rand::random_range(0..t)))
        },
        _ => None,
    };
    let opts = LoadRequestOptions {
        start_playing: true,
        playing_track,
        context_options,
        ..Default::default()
    };
    PlayerRequest::PlayContext(context_uri.to_string(), opts)
}
