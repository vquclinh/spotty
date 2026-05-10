use crate::app::{ActiveBlock, App, Route};
use crate::handlers::{album, library, playbar, queue, search, sidebar};
use crate::network::models::*;
use crate::network::request::{PlayerRequest, ClientRequest};
use crossterm::event::{KeyEvent, KeyCode};

use super::{global, home, playlist};
use crate::app::types::MenuAction;

use crate::app::album_state::AlbumState;

pub fn handle_key_events(key: KeyEvent, app: &mut App) {
    if app.show_help {
        app.show_help = false;
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

    if app.active_block == ActiveBlock::SearchInput {
        search::handle_search_events(key, app);
        return;
    }
    // global keyboard
    if global::handle_global_events(key, app) {
        return;
    }

    // each active_block
    match app.active_block {
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
        _ => {}
    }
}

// TODO: handle the AddToPlaylist action outside the function and
// return nothing here
fn execute_action_menu_command(app: &mut App) -> bool {
    let selected_action = app.action_menu.state.selected()
        .and_then(|idx| app.action_menu.actions.get(idx));

    if let (Some(action), Some(target)) = (selected_action, &app.action_menu.target) {
        let uri = match target {
            MenuTarget::Track(t) => Some(t.uri.clone()),
            MenuTarget::Episode(e) => Some(e.uri.clone()),
            MenuTarget::Artist(a) => Some(a.uri.clone()),
            MenuTarget::Album(a) => Some(a.uri.clone()),
            MenuTarget::Playlist(p) => Some(p.uri.clone()),
        };

        match action {
            MenuAction::PlayNow => {
                if let Some(u) = uri {
                    let player_req = match target {
                        // If this is a playable item and we are in a playlist/album then
                        // playing the item will also play the whole collection
                        MenuTarget::Track(_) | MenuTarget::Episode(_) => {
                            match &app.route {
                                Route::PlaylistDetail(s) => {
                                    let offset = s.tracks.state.selected().map(|i| i as u32);
                                    PlayerRequest::PlayContext(s.playlist.uri.clone(), offset)
                                }
                                Route::AlbumDetail(s) => {
                                    let offset = s.tracks.state.selected().map(|i| i as u32);
                                    PlayerRequest::PlayContext(s.album.uri.clone(), offset)
                                }
                                _ => PlayerRequest::Play(u),
                            }
                        }
                        MenuTarget::Album(_) | MenuTarget::Playlist(_) | MenuTarget::Artist(_)
                            => PlayerRequest::PlayContext(u, None),
                    };

                    let _ = app.network_tx.send(ClientRequest::Player(player_req));
                }
                true
            }
            MenuAction::AddToQueue => {
                if let Some(u) = uri {
                    let _ = app.network_tx.send(ClientRequest::Player(PlayerRequest::AddItemToQueue(u)));
                }
                true
            }
            MenuAction::AddToPlaylist => {
                let my_id = &app.user.id;

                let writable_playlists: Vec<Playlist> = app.playlists_menu.items
                    .iter()
                    .filter(|p| {
                        let is_owner = p.owner.id == *my_id;
                        let is_collaborator = p.collaborative;

                        is_owner || is_collaborator
                    })
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
            MenuAction::RemoveFromThisPlaylist => {
                if let Route::PlaylistDetail(route) = &mut app.route
                    && let Some(uri) = uri && !uri.is_empty() {
                    let _ = app.network_tx.send(ClientRequest::RemoveItemsFromPlaylist {
                        playlist_id: route.playlist.id.clone(), uris: vec![uri.clone()]
                    });
                    // Update the list locally
                    route.tracks.items.retain(|item| match item {
                        PlayableItem::Track(i) => i.uri != uri,
                        PlayableItem::Episode(i) => i.uri != uri
                    });
                    // Fix the selected index
                    if let Some(idx) = route.tracks.state.selected() {
                        let len = route.tracks.items.len();
                        if idx >= len {
                            route.tracks.state.select(Some(len - 1));
                        }
                    }

                }
                true
            }
            MenuAction::GoToAlbum => {
                let album_id = match target {
                    MenuTarget::Track(t) => t.album.as_ref().map(|a| a.id.clone()),
                    MenuTarget::Album(a) => Some(a.id.clone()),
                    _ => None,
                };

                if let Some(id) = album_id {
                    let new_album_state = AlbumState::new(id);
                    app.set_current_route(Route::AlbumDetail(new_album_state));
                    app.active_block = ActiveBlock::AlbumBlock;
                }
                true
            }
            MenuAction::ViewDetails => {
                // TODO
                true
            }
            MenuAction::GoToShow => {
                true
            }
            MenuAction::SaveToLibrary => {
                if let Some(uri) = uri && !uri.is_empty() {
                    match target {
                        MenuTarget::Track(_) => {
                            let _ = app.network_tx
                                .send(ClientRequest::SaveItemsToLibrary(vec![uri]));
                        }
                        MenuTarget::Album(_) => {
                            let _ = app.network_tx
                                .send(ClientRequest::SaveItemsToLibrary(vec![uri]));
                        }
                        MenuTarget::Artist(_) => {
                            let _ = app.network_tx
                                .send(ClientRequest::SaveItemsToLibrary(vec![uri]));
                        }
                        MenuTarget::Episode(_) => {
                            let _ = app.network_tx
                                .send(ClientRequest::SaveItemsToLibrary(vec![uri]));
                        }
                        _ => {}
                    }
                }
                true
            }
            #[allow(clippy::collapsible_if)]
            MenuAction::RemoveFromLibrary => {
                if let Some(uri) = uri && !uri.is_empty() {
                    if matches!(target, MenuTarget::Track(_))
                    || matches!(target, MenuTarget::Album(_))
                    || matches!(target, MenuTarget::Artist(_))
                    || matches!(target, MenuTarget::Episode(_)) {
                        let _ = app.network_tx
                            .send(ClientRequest::RemoveItemsFromLibrary(vec![uri.clone()]));
                        // Update the list locally
                        let state_info = match &mut app.route {
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
                        // Fix the selected index
                        if let Some((len, list_state)) = state_info
                        && let Some(idx) = list_state.selected()
                        && idx >= len {
                            list_state.select(Some(len - 1));
                        }
                    }
                }
                true
            }
            MenuAction::FollowArtist => {
                if let MenuTarget::Artist(_) = target && let Some(uri) = uri {
                    let _ = app.network_tx.send(ClientRequest::SaveItemsToLibrary(vec![uri]));
                }
                true
            }
            MenuAction::UnfollowArtist => {
                if let MenuTarget::Artist(_) = target && let Some(uri) = uri {
                    let _ = app.network_tx.send(ClientRequest::RemoveItemsFromLibrary(vec![uri]));
                }
                true
            }
        }
    } else {
        true
    }
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
