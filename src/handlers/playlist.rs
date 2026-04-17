use crate::app::{ActiveBlock, App, route::Route};
use crossterm::event::{KeyCode, KeyEvent};

pub fn handle_playlist_events(key: KeyEvent, app: &mut App) {
    if let Route::PlaylistDetail(playlist_state) = &mut app.route {
        match key.code {
            KeyCode::Down | KeyCode::Char('j') => playlist_state.tracks.next(),
            KeyCode::Up | KeyCode::Char('k') => playlist_state.tracks.previous(),
            
            KeyCode::Backspace | KeyCode::Char('b') => {
                app.active_block = ActiveBlock::PlaylistsMenu;
            }
            _ => {}
        }
    }
}