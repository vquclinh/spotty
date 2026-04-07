use std::time::Duration;

use crossterm::event::KeyEvent;

pub enum Event {
    Tick,
    Key(KeyEvent),
}

pub fn read(_tick_rate: Duration) -> anyhow::Result<Event> {
    Ok(Event::Tick)
}
