use crate::app::App;
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

            _ => {}
        }
    }
}
