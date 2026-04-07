use std::time::Duration;

use crossterm::event::{self, Event as CrosstermBackend, KeyEvent};

pub enum Event {
    Tick,
    Key(KeyEvent),
}

pub fn read(tick_rate: Duration) -> anyhow::Result<Event> {
    if event::poll(tick_rate)? {
        if let CrosstermBackend::Key(key) = event::read()? {
            return Ok(Event::Key(key));
        }
    }

    Ok(Event::Tick)
}
