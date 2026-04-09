use crate::app::{App, Route};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
};

use super::{help_popup, home, lyrics, playbar, queue, search, sidebar};

pub fn draw(f: &mut Frame, app: &App) {
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(f.area());

    playbar::draw(f, app, main_chunks[1]);

    if app.route == Route::Lyrics {
        lyrics::draw(f, app, main_chunks[0]);
    } else {
        let content_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(25), Constraint::Percentage(75)])
            .split(main_chunks[0]);

        sidebar::draw(f, app, content_chunks[0]);

        match app.route {
            Route::Home => home::draw(f, app, content_chunks[1]),
            Route::Search => search::draw(f, app, content_chunks[1]),
            Route::Queue => queue::draw(f, app, content_chunks[1]),
            _ => {}
        }
    }

    if app.show_help {
        help_popup::draw(f, app);
    }
}
