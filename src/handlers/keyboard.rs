use crate::app::{ActiveBlock, App, route::Route};
use crossterm::event::{KeyCode, KeyModifiers, KeyEvent};

use crate::app::home_state::HomeTab;

pub fn handle_key_events(key: KeyEvent, app: &mut App) {
    if key.code == KeyCode::Char('q') 
        || key.code == KeyCode::Esc 
        || (key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL)) 
    {
        app.should_quit = true;
        return;
    }

    if key.code == KeyCode::Tab && !key.modifiers.contains(KeyModifiers::CONTROL) {
        app.active_block = match app.active_block {
            ActiveBlock::LibraryMenu => ActiveBlock::PlaylistsMenu,
            ActiveBlock::PlaylistsMenu => ActiveBlock::HomeBlock,
            ActiveBlock::HomeBlock => ActiveBlock::QueueBlock,
            ActiveBlock::QueueBlock => ActiveBlock::LibraryMenu,
            _ => ActiveBlock::LibraryMenu,
        };
        
        return; 
    }

    match &mut app.route {
        Route::Home(home_state) => {
            if app.active_block == ActiveBlock::HomeBlock {
                match key.code {
                    KeyCode::Char('1') => home_state.active_tab = HomeTab::TopTracks,
                    KeyCode::Char('2') => home_state.active_tab = HomeTab::TopArtists,
                    KeyCode::Char('3') => home_state.active_tab = HomeTab::RecentlyPlayed,
                    
                    KeyCode::Right => {
                        home_state.active_tab = match home_state.active_tab {
                            HomeTab::TopTracks => HomeTab::TopArtists,
                            HomeTab::TopArtists => HomeTab::RecentlyPlayed,
                            HomeTab::RecentlyPlayed => HomeTab::TopTracks,
                        };
                    }

                    KeyCode::Left => {
                        home_state.active_tab = match home_state.active_tab {
                            HomeTab::TopTracks => HomeTab::RecentlyPlayed, // Vòng lại cuối
                            HomeTab::TopArtists => HomeTab::TopTracks,
                            HomeTab::RecentlyPlayed => HomeTab::TopArtists,
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
        }
        Route::Search(_search_state) => {
            // TODO
        }

        _ => {}
    }
}