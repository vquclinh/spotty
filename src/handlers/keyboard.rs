use crate::app::{ActiveBlock, App};
use crate::handlers::search;
use crossterm::event::{KeyEvent, KeyCode, KeyModifiers};

use super::{global, sidebar, home, playlist};

pub fn handle_key_events(key: KeyEvent, app: &mut App) {
    if app.show_help {
        app.show_help = false;
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

    // Tab focus cycling
    if key.code == KeyCode::Tab && !key.modifiers.contains(KeyModifiers::CONTROL) {
        app.active_block = match app.active_block {
            ActiveBlock::LibraryMenu => ActiveBlock::PlaylistsMenu,
            ActiveBlock::PlaylistsMenu => ActiveBlock::HomeBlock,
            ActiveBlock::HomeBlock => ActiveBlock::QueueBlock,
            ActiveBlock::QueueBlock => ActiveBlock::LibraryMenu,
            _ => ActiveBlock::LibraryMenu,
        };

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
        _ => {}
    }
}
