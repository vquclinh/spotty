use crate::app::{ActiveBlock, App};
use crate::handlers::{queue, search};
use crossterm::event::{KeyEvent, KeyCode};

use super::{global, sidebar, home, playlist};
use crate::app::types::MenuAction;

pub fn handle_key_events(key: KeyEvent, app: &mut App) {
    if app.show_help {
        app.show_help = false;
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
                execute_action_menu_command(app);
                app.action_menu.close();
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

fn execute_action_menu_command(app: &mut App) {
    let selected_action = app.action_menu.state.selected()
        .and_then(|idx| app.action_menu.actions.get(idx));

    if let (Some(action), Some(_target)) = (selected_action, &app.action_menu.target) {
        match action {
            MenuAction::PlayNow => {
                // TODO
            }
            MenuAction::AddToQueue => {
                // TODO
            }
            MenuAction::AddToPlaylist => {
                // TODO
            }
            MenuAction::GoToAlbum => {
                // TODO
            }
            MenuAction::GoToArtist => {
                // TODO
            }
            MenuAction::ViewDetails => {
                // TODO
            }
            // TODO
            _ => {}
        }
    }
}