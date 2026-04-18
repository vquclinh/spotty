use crossterm::event::{KeyCode, KeyEvent};
use ratatui::widgets::ListState;
use crate::app::{ActiveBlock, App, route::Route};
use crate::app::search_state::SearchHoveredPane;
use crate::network::request::ClientRequest;
use crate::network::models::*;

pub fn handle_search_events(key: KeyEvent, app: &mut App) {
    let mut query_to_send = None;

    let (tracks_len, artists_len, albums_len, playlists_len) = {
        let state_lock = app.shared_state.lock().unwrap();
        (
            state_lock.search_results.tracks.as_ref().map_or(0, |t| t.len()),
            state_lock.search_results.artists.as_ref().map_or(0, |t| t.len()),
            state_lock.search_results.albums.as_ref().map_or(0, |t| t.len()),
            state_lock.search_results.playlists.as_ref().map_or(0, |t| t.len()),
        )
    };

    if let Route::Search(search_state) = &mut app.route {
        match app.active_block {
            ActiveBlock::SearchInput => {
                match key.code {
                    KeyCode::Char(c) => {
                        search_state.input.push(c);
                    }
                    KeyCode::Backspace => {
                        search_state.input.pop();
                    }
                    KeyCode::Enter => {
                        let query = search_state.input.clone();
                        if !query.is_empty() {
                            query_to_send = Some(query);
                        }

                        app.active_block = ActiveBlock::SearchResults;
                        search_state.hovered_pane = SearchHoveredPane::Tracks;
                    }

                    _ => {}
                }
            }

            ActiveBlock::SearchResults => {
                match key.code {
                    KeyCode::Tab => {
                        match search_state.hovered_pane {
                            SearchHoveredPane::Tracks => search_state.hovered_pane = SearchHoveredPane::Artists,
                            SearchHoveredPane::Artists => search_state.hovered_pane = SearchHoveredPane::Albums,
                            SearchHoveredPane::Albums => search_state.hovered_pane = SearchHoveredPane::Playlists,
                            SearchHoveredPane::Playlists => {
                                app.active_block = ActiveBlock::Playbar;
                                search_state.hovered_pane = SearchHoveredPane::Tracks;
                            }
                            _ => search_state.hovered_pane = SearchHoveredPane::Tracks,
                        }
                    }

                    KeyCode::Down => {
                        match search_state.hovered_pane {
                            SearchHoveredPane::Tracks => next_item(&mut search_state.tracks_state, tracks_len),
                            SearchHoveredPane::Artists => next_item(&mut search_state.artists_state, artists_len),
                            SearchHoveredPane::Albums => next_item(&mut search_state.albums_state, albums_len),
                            SearchHoveredPane::Playlists => next_item(&mut search_state.playlists_state, playlists_len),
                            _ => {}
                        }
                    }
                    KeyCode::Up => {
                        match search_state.hovered_pane {
                            SearchHoveredPane::Tracks => prev_item(&mut search_state.tracks_state, tracks_len),
                            SearchHoveredPane::Artists => prev_item(&mut search_state.artists_state, artists_len),
                            SearchHoveredPane::Albums => prev_item(&mut search_state.albums_state, albums_len),
                            SearchHoveredPane::Playlists => prev_item(&mut search_state.playlists_state, playlists_len),
                            _ => {}
                        }
                    }

                    KeyCode::Esc => {
                        app.active_block = ActiveBlock::SearchInput;
                    }
                    _ => {}
                }
            }

            _ => {}
        }
    }

    if let Some(query) = query_to_send {
        let types = vec![SearchType::Track, SearchType::Artist, SearchType::Album, SearchType::Playlist];
        let _ = app.network_tx.send(ClientRequest::SearchItems { 
            query, 
            search_types: types,
            limit: 20 
        });
    }
}

fn next_item(state: &mut ListState, len: usize) {
    if len == 0 { return; }
    let i = match state.selected() {
        Some(i) => if i >= len - 1 { 0 } else { i + 1 },
        None => 0,
    };
    state.select(Some(i));
}

fn prev_item(state: &mut ListState, len: usize) {
    if len == 0 { return; }
    let i = match state.selected() {
        Some(i) => if i == 0 { len - 1 } else { i - 1 },
        None => 0,
    };
    state.select(Some(i));
}
