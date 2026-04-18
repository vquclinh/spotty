use crate::app::{ActiveBlock, App, route::Route, home_state::HomeTab};
use crate::handlers::search;
use crate::network::request::ClientRequest;
use crossterm::event::{KeyEvent, KeyCode, KeyModifiers};

use super::{global, sidebar, home, playlist};

pub fn handle_key_events(key: KeyEvent, app: &mut App) {
    if app.show_help {
        app.show_help = false;
        return;
    }
    
    if app.active_block == ActiveBlock::SearchInput {
        search::handle_search_events(key, app);
        return;
    }
    // global keyboard
    if global::handle_global_events(key, app) {
        return;
    }

    // Tab focus cycling
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

    // each active_block
    match app.active_block {
        ActiveBlock::PlaylistsMenu => {
            sidebar::handle_sidebar_events(key, app);
            return;
        }
        ActiveBlock::HomeBlock => {
            home::handle_home_events(key, app);
            // No return here to allow route-specific tab logic below
        }
        ActiveBlock::PlaylistTracks => {
            playlist::handle_playlist_events(key, app);
            return;
        }
        ActiveBlock::SearchResults => {
            search::handle_search_events(key, app);
            return;
        }
        _ => {}
    }

    // keybinds for route (not for block)
    match &mut app.route {
        Route::Home(home_state) => {
            if app.active_block == ActiveBlock::HomeBlock {
                match key.code {
                    KeyCode::Char('1') => home_state.active_tab = HomeTab::TopTracks,
                    KeyCode::Char('2') => home_state.active_tab = HomeTab::TopArtists,
                    KeyCode::Char('3') => {
                        home_state.active_tab = HomeTab::RecentlyPlayed;
                        let _ = app.network_tx.send(ClientRequest::GetRecentlyPlayed { limit: 50, offset: 0 });
                    }

                    KeyCode::Right | KeyCode::Char('l') => {
                        home_state.active_tab = match home_state.active_tab {
                            HomeTab::TopTracks => HomeTab::TopArtists,
                            HomeTab::TopArtists => HomeTab::RecentlyPlayed,
                            HomeTab::RecentlyPlayed => HomeTab::TopTracks,
                        };
                        match home_state.active_tab {
                            HomeTab::RecentlyPlayed => {
                                let _ = app.network_tx.send(ClientRequest::GetRecentlyPlayed { limit: 50, offset: 0 });
                            }
                            _ => {}
                        }
                    }

                    KeyCode::Left | KeyCode::Char('h') => {
                        home_state.active_tab = match home_state.active_tab {
                            HomeTab::TopTracks => HomeTab::RecentlyPlayed,
                            HomeTab::TopArtists => HomeTab::TopTracks,
                            HomeTab::RecentlyPlayed => HomeTab::TopArtists,
                        };

                        match home_state.active_tab {
                            HomeTab::RecentlyPlayed => {
                                let _ = app.network_tx.send(ClientRequest::GetRecentlyPlayed { limit: 50, offset: 0 });
                            }
                            _ => {}
                        }
                    }

                    KeyCode::Down | KeyCode::Char('j') => match home_state.active_tab {
                        HomeTab::TopTracks => home_state.top_tracks.next(),
                        HomeTab::TopArtists => home_state.top_artists.next(),
                        HomeTab::RecentlyPlayed => home_state.recent_tracks.next(),
                    },
                    KeyCode::Up | KeyCode::Char('k') => match home_state.active_tab {
                        HomeTab::TopTracks => home_state.top_tracks.previous(),
                        HomeTab::TopArtists => home_state.top_artists.previous(),
                        HomeTab::RecentlyPlayed => home_state.recent_tracks.previous(),
                    },
                    _ => {}
                }
            }
        }
        Route::Search(_) => {
            // TODO
        }
        _ => {}
    }
}
