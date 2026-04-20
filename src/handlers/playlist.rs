use crate::app::{ActiveBlock, App, route::Route};
use crossterm::event::{KeyCode, KeyEvent};
use crate::network::models::*;

pub fn handle_playlist_events(key: KeyEvent, app: &mut App) {
    let mut target_to_open = None;

    if let Route::PlaylistDetail(playlist_state) = &mut app.route {
        match key.code {
            KeyCode::Down | KeyCode::Char('j') => playlist_state.tracks.next(),
            KeyCode::Up | KeyCode::Char('k') => playlist_state.tracks.previous(),
            
            KeyCode::Char('t') => {
                target_to_open = playlist_state.tracks.state.selected()
                    .and_then(|idx| playlist_state.tracks.items.get(idx))
                    .map(|item| match item {
                        PlayableItem::Track(t) => MenuTarget::Track(t.clone()),
                        PlayableItem::Episode(e) => MenuTarget::Episode(e.clone()),
                    });
            }

            KeyCode::Backspace | KeyCode::Char('b') => {
                app.active_block = ActiveBlock::PlaylistsMenu;
            }
            _ => {}
        }
    }
    
    if let Some(target) = target_to_open {
        app.action_menu.open(target);
    }
}