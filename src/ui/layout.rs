use crate::app::{App, Route};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
};

use super::{splash, lyrics, playbar, queue, search, sidebar, playlist, help};

pub fn draw(f: &mut Frame, app: &mut App) {
    // splash
    if let Route::Splash(splash_state) = &app.route {
        splash::draw(f, splash_state, f.area());
        return; 
    }

    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(f.area());

    playbar::draw(f, app, main_chunks[1]);

    match &mut app.route {
        Route::Lyrics => {
            let lyrics_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
                .split(main_chunks[0]);

            lyrics::draw_text(f, app, lyrics_chunks[0]);
            lyrics::draw_info(f, app, lyrics_chunks[1]);
        }

        Route::Search(_) => {
            search::draw(f, app, main_chunks[0]);
        }

        // state that has sidebar and library
        _ => {
            let content_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(22), Constraint::Percentage(70)])
                .split(main_chunks[0]);

            sidebar::draw(f, app, content_chunks[0]);

            match &mut app.route {
                Route::Home(home_state) => crate::ui::home::draw(f, home_state, &app.active_block, content_chunks[1]),
                Route::Queue => queue::draw(f, app, content_chunks[1]),
                Route::PlaylistDetail(playlist_state) => playlist::draw(f, playlist_state, &app.active_block, content_chunks[1]),
                _ => {}
            }
        }
    }

    if app.show_help {
        help::draw(f, f.area());
    }
}
