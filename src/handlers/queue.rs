use crate::app::{ActiveBlock, App, route::Route};
use crossterm::event::{KeyCode, KeyEvent};
use crate::network::models::*;

pub fn handle_queue_events(key: KeyEvent, app: &mut App) {
    let mut target_to_open = None;

    if let Route::Queue(queue_state) = &mut app.route {
        match key.code {
            KeyCode::Down | KeyCode::Char('j') => queue_state.queue_items.next(),
            KeyCode::Up | KeyCode::Char('k') => queue_state.queue_items.previous(),
            
            KeyCode::Char('t') => {
                target_to_open = queue_state.queue_items.state.selected()
                    .and_then(|idx| queue_state.queue_items.items.get(idx))
                    .map(|item| match item {
                        PlayableItem::Track(t) => MenuTarget::Track(t.clone()),
                        PlayableItem::Episode(e) => MenuTarget::Episode(e.clone()),
                    });
            }

            KeyCode::Backspace | KeyCode::Char('b') | KeyCode::Esc => {
                app.active_block = ActiveBlock::PlaylistsMenu;
            }
            _ => {}
        }
    }
    
    if let Some(target) = target_to_open {
        app.action_menu.open(target);
    }
}