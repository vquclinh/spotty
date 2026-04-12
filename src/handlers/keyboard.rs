use crate::event::core::Event; 
use crate::app::{App, route::Route};
use crossterm::event::KeyCode;

use crate::app::home_state::HomeTab;

pub fn handle(evt: Event, app: &mut App) {
    match evt {
        Event::Tick => {}

        Event::Key(key) => {
            if key.code == KeyCode::Char('q') {
                app.should_quit = true;
                return;
            }

            match &mut app.route {
                Route::Home(home_state) => {
                    match key.code {
                        KeyCode::Char('1') => home_state.active_tab = HomeTab::TopTracks,
                        KeyCode::Char('2') => home_state.active_tab = HomeTab::TopArtists,
                        KeyCode::Char('3') => home_state.active_tab = HomeTab::RecentlyPlayed,
                        
                        KeyCode::Tab => {
                            home_state.active_tab = match home_state.active_tab {
                                HomeTab::TopTracks => HomeTab::TopArtists,
                                HomeTab::TopArtists => HomeTab::RecentlyPlayed,
                                HomeTab::RecentlyPlayed => HomeTab::TopTracks,
                            };
                        }

                        KeyCode::Down => {
                            match home_state.active_tab {
                                HomeTab::TopTracks => home_state.top_tracks.next(),
                                HomeTab::TopArtists => home_state.top_artists.next(),
                                HomeTab::RecentlyPlayed => home_state.recent_tracks.next(),
                            }
                        }
                        KeyCode::Up => {
                            match home_state.active_tab {
                                HomeTab::TopTracks => home_state.top_tracks.previous(),
                                HomeTab::TopArtists => home_state.top_artists.previous(),
                                HomeTab::RecentlyPlayed => home_state.recent_tracks.previous(),
                            }
                        }
                        _ => {}
                    }
                }

                Route::Search(_search_state) => {
                    // TODO
                }
                
                _ => {}
            }
        }
    }
}