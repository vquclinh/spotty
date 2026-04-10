use crate::app::{ App, ActiveBlock };
use crate::event::Event;
use crossterm::event::{KeyCode, KeyModifiers};


pub fn handle(event: Event, app: &mut App) {
    if let Event::Key(key) = event {
        match key.code {
            KeyCode::Char('q') => {
                app.should_quit = true;
            }
            KeyCode::Esc => {
                app.should_quit = true;
            }
            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                app.should_quit = true;
            }
            KeyCode::Down => {
                if app.active_block == ActiveBlock::HomeBlock {
                    let current_idx = app.track_list_state.selected().unwrap_or(0);
                    let next_idx = if current_idx >= app.track_list.len() - 1 {
                        0
                    } else {
                        current_idx + 1
                    };
                    app.track_list_state.select(Some(next_idx));
                }
            }
            
            KeyCode::Up => {
                if app.active_block == ActiveBlock::HomeBlock {
                    let current_idx = app.track_list_state.selected().unwrap_or(0);
                    let prev_idx = if current_idx == 0 {
                        app.track_list.len() - 1
                    } else {
                        current_idx - 1
                    };
                    app.track_list_state.select(Some(prev_idx));
                }
            }
            _ => {}
        }
    }
}
