use crate::app::{App, Route};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
};

use super::{splash, home, lyrics, playbar, queue, search, sidebar, playlist};

pub fn draw(f: &mut Frame, app: &mut App) {
    if let Route::Splash(splash_state) = &app.route {
        splash::draw(f, splash_state, f.area());
        return; 
    }

    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(f.area());

    playbar::draw(f, app, main_chunks[1]);

    if matches!(app.route, Route::Lyrics) {
        let lyrics_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
            .split(main_chunks[0]);

        lyrics::draw_text(f, app, lyrics_chunks[0]);
        lyrics::draw_info(f, app, lyrics_chunks[1]);
    } else {
        let content_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(15), Constraint::Percentage(85)])
            .split(main_chunks[0]);

        sidebar::draw(f, app, content_chunks[0]);

        match &app.route {
            Route::Home(_) => home::draw(f, app, content_chunks[1]),
            Route::Search(search_state) => search::draw(f, &search_state, &app.active_block, content_chunks[1]),
            Route::Queue => queue::draw(f, app, content_chunks[1]),
            Route::PlaylistDetail => playlist::draw(f, app, content_chunks[1]),
            _ => {}
        }
    }
}
