use crate::app::{ActiveBlock, App, route::Route};
use crossterm::event::KeyEvent;

use super::{global, sidebar, home, playlist};

pub fn handle_key_events(key: KeyEvent, app: &mut App) {
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
            return;
        }
        ActiveBlock::PlaylistTracks => {
            playlist::handle_playlist_events(key, app);
            return;
        }
        _ => {}
    }

    // keybinds for route (not for block)
    match app.route {
        Route::Search(_) => {
            // TODO
        }
        _ => {}
    }
}