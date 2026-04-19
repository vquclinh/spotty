use crossterm::event::{KeyCode, KeyEvent};
use crate::app::{ActiveBlock, App, route::Route};
use crate::app::search_state::SearchHoveredPane;
use crate::network::request::ClientRequest;
use crate::network::models::*;

pub fn handle_search_events(key: KeyEvent, app: &mut App) {
    let mut query_to_send = None;

    if let Route::Search(search_state) = &mut app.route {
        let tracks_len = search_state.results.tracks.as_ref().map_or(0, |t| t.items.len());
        let artists_len = search_state.results.artists.as_ref().map_or(0, |t| t.items.len());
        let albums_len = search_state.results.albums.as_ref().map_or(0, |t| t.items.len());
        let playlists_len = search_state.results.playlists.as_ref().map_or(0, |t| t.items.len());

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

                    KeyCode::Down | KeyCode::Char('j') => {
                        match search_state.hovered_pane {
                            SearchHoveredPane::Tracks => search_state.tracks_state.next(tracks_len),
                            SearchHoveredPane::Artists => search_state.artists_state.next(artists_len),
                            SearchHoveredPane::Albums => search_state.albums_state.next(albums_len),
                            SearchHoveredPane::Playlists => search_state.playlists_state.next(playlists_len),
                            _ => {}
                        }
                    }
                    
                    KeyCode::Up | KeyCode::Char('k') => {
                        match search_state.hovered_pane {
                            SearchHoveredPane::Tracks => search_state.tracks_state.previous(tracks_len),
                            SearchHoveredPane::Artists => search_state.artists_state.previous(artists_len),
                            SearchHoveredPane::Albums => search_state.albums_state.previous(albums_len),
                            SearchHoveredPane::Playlists => search_state.playlists_state.previous(playlists_len),
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
            limit: 10,
            offset: 0
        });
    }
}