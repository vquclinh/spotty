use crate::app::{ActiveBlock, App, Route, AppState};
use crate::network::models::*;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub fn handle_album_events(key: KeyEvent, app: &mut App) {
    let mut target_to_open = None;
    let AppState { route, active_block } = app.state.current_mut();

    if let Route::AlbumDetail(album_state) = route {
        match key {
            KeyEvent{ code: KeyCode::Down, .. }
            | KeyEvent { code: KeyCode::Char('j'), ..}
            | KeyEvent {
                code: KeyCode::Char('d'),
                modifiers: KeyModifiers::CONTROL,
                ..
            } => {
                let steps = if key.code == KeyCode::Char('d') { 10 } else { 1 };
                album_state.tracks.next(steps, false);
            }

            KeyEvent { code: KeyCode::Up, .. }
            | KeyEvent { code: KeyCode::Char('k'), .. }
            | KeyEvent {
                code: KeyCode::Char('u'),
                modifiers: KeyModifiers::CONTROL,
                ..
            } => {
                let steps = if key.code == KeyCode::Char('u') { 10 } else { 1 };
                album_state.tracks.previous(steps, false);
            }
            
            KeyEvent { code: KeyCode::Char('t'), .. } => {
                target_to_open = album_state.tracks.state.selected()
                    .and_then(|idx| album_state.tracks.items.get(idx))
                    .map(|track| MenuTarget::Track(track.clone()));
            }

            KeyEvent { code: KeyCode::Backspace, .. }
            | KeyEvent { code: KeyCode::Char('b'), .. }
            | KeyEvent { code: KeyCode::Esc, .. } => {
                *active_block = ActiveBlock::PlaylistsMenu;
            }

            _ => {}
        }
    }
    
    if let Some(target) = target_to_open {
        app.action_menu.open(target, route);
    }
}
