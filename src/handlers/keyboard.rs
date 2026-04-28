use crate::app::{ActiveBlock, App, Route};
use crate::handlers::{album, library, queue, search, sidebar};
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
            return;
        }
        ActiveBlock::HomeBlock => {
            home::handle_home_events(key, app);
            // No return here to allow route-specific tab logic below
        }
        ActiveBlock::PlaylistTracks => {
            playlist::handle_playlist_events(key, app);
            return;
        }
        ActiveBlock::SearchResults => {
            search::handle_search_events(key, app);
            return;
        }
        ActiveBlock::QueueBlock => {
            queue::handle_queue_events(key, app);
            return;
        }
        ActiveBlock::AlbumBlock => {
            album::handle_album_events(key, app);
            return;
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
        _ => {}
    }
}

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
                // TODO
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
            // TODO
            _ => true,
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
