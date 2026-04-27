use crate::app::{ActiveBlock, App, route::Route};
use crate::network::models::*;
use crossterm::event::{KeyCode, KeyEvent};

pub fn handle_album_events(key: KeyEvent, app: &mut App) {
    let mut target_to_open = None;

    if let Route::AlbumDetail(album_state) = &mut app.route {
        match key.code {
            KeyCode::Down | KeyCode::Char('j') => album_state.tracks.next(),
            KeyCode::Up | KeyCode::Char('k') => album_state.tracks.previous(),
            
            KeyCode::Char('t') => {
                target_to_open = album_state.tracks.state.selected()
                    .and_then(|idx| album_state.tracks.items.get(idx))
                    .map(|track| MenuTarget::Track(track.clone()));
            }

            KeyCode::Backspace | KeyCode::Char('b') | KeyCode::Esc => {
                app.active_block = ActiveBlock::PlaylistsMenu;
            }
            _ => {}
        }
    }
    
    if let Some(target) = target_to_open {
        app.action_menu.open(target, &app.route);
    }
}
