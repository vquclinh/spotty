use crate::app::{ActiveBlock, App};
use crate::handlers::{queue, search};
use crate::network::models::MenuTarget;
use crate::network::request::{PlayerRequest, ClientRequest};
use crossterm::event::{KeyEvent, KeyCode};

use super::{global, sidebar, home, playlist};
use crate::app::types::MenuAction;

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
        ActiveBlock::PlaylistsMenu => {
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
                app.playlist_selector.playlists = app.playlists.items.clone();
                app.playlist_selector.is_open = true;
                app.playlist_selector.state.select(Some(0));
                false
            }
            MenuAction::GoToAlbum => {
                // TODO
                true
            }
            MenuAction::GoToArtist => {
                // TODO
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