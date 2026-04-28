use crossterm::event::{KeyCode, KeyEvent};
use crate::app::{ActiveBlock, App, route::Route};
use crate::app::search_state::SearchHoveredPane;
use crate::network::request::ClientRequest;
use crate::network::models::*;

pub fn handle_search_events(key: KeyEvent, app: &mut App) {
    let mut query_to_send = None;
    let mut target_to_open = None;

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

                    KeyCode::Down | KeyCode::Char('j') => {
                        match search_state.hovered_pane {
                            SearchHoveredPane::Tracks => search_state.tracks_state.next(false),
                            SearchHoveredPane::Artists => search_state.artists_state.next(false),
                            SearchHoveredPane::Albums => search_state.albums_state.next(false),
                            SearchHoveredPane::Playlists => search_state.playlists_state.next(false),
                            _ => {}
                        }
                    }

                    KeyCode::Up | KeyCode::Char('k') => {
                        match search_state.hovered_pane {
                            SearchHoveredPane::Tracks => search_state.tracks_state.previous(false),
                            SearchHoveredPane::Artists => search_state.artists_state.previous(false),
                            SearchHoveredPane::Albums => search_state.albums_state.previous(false),
                            SearchHoveredPane::Playlists => search_state.playlists_state.previous(false),
                            _ => {}
                        }
                    }

                    KeyCode::Char('t') => {
                        target_to_open = match search_state.hovered_pane {
                            SearchHoveredPane::Tracks => search_state
                                .tracks_state
                                .state
                                .selected()
                                .and_then(|idx| search_state.tracks_state.items.get(idx))
                                .map(|track| MenuTarget::Track(track.clone())),

                            SearchHoveredPane::Artists => search_state
                                .artists_state
                                .state
                                .selected()
                                .and_then(|idx| search_state.artists_state.items.get(idx))
                                .map(|artist| MenuTarget::Artist(artist.clone())),

                            SearchHoveredPane::Albums => search_state
                                .albums_state
                                .state
                                .selected()
                                .and_then(|idx| search_state.albums_state.items.get(idx))
                                .map(|album| MenuTarget::Album(album.clone())),

                            SearchHoveredPane::Playlists => search_state
                                .playlists_state
                                .state
                                .selected()
                                .and_then(|idx| search_state.playlists_state.items.get(idx))
                                .map(|playlist| MenuTarget::Playlist(playlist.clone())),

                            _ => None,
                        };
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

    if let Some(target) = target_to_open {
        app.action_menu.open(target, &app.route);
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
