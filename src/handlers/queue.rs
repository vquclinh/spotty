use crate::app::{ActiveBlock, App, route::Route};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crate::network::models::*;

pub fn handle_queue_events(key: KeyEvent, app: &mut App) {
    let mut target_to_open = None;

    if let Route::Queue(queue_state) = &mut app.route {
        match key {
            KeyEvent{ code: KeyCode::Down, .. }
            | KeyEvent { code: KeyCode::Char('j'), ..}
            | KeyEvent {
                code: KeyCode::Char('d'),
                modifiers: KeyModifiers::CONTROL,
                ..
            } => {
                let steps = if key.code == KeyCode::Char('d') { 10 } else { 1 };
                queue_state.queue_items.next(steps, false);
            }

            KeyEvent { code: KeyCode::Up, .. }
            | KeyEvent { code: KeyCode::Char('k'), .. }
            | KeyEvent {
                code: KeyCode::Char('u'),
                modifiers: KeyModifiers::CONTROL,
                ..
            } => {
                let steps = if key.code == KeyCode::Char('u') { 10 } else { 1 };
                queue_state.queue_items.previous(steps, false);
            }
            
            KeyEvent { code: KeyCode::Char('t'), .. } => {
                target_to_open = queue_state.queue_items.state.selected()
                    .and_then(|idx| queue_state.queue_items.items.get(idx))
                    .map(|item| match item {
                        PlayableItem::Track(t) => MenuTarget::Track(t.clone()),
                        PlayableItem::Episode(e) => MenuTarget::Episode(e.clone()),
                    });
            }

            KeyEvent { code: KeyCode::Backspace, .. }
            | KeyEvent { code: KeyCode::Char('b'), .. }
            | KeyEvent { code: KeyCode::Esc, .. } => {
                app.active_block = ActiveBlock::PlaylistsMenu;
            }

            _ => {}
        }
    }
    
    if let Some(target) = target_to_open {
        app.action_menu.open(target, &app.route);
    }
}
