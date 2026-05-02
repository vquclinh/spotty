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
                        let threshold = 20;

                        match search_state.hovered_pane {
                            SearchHoveredPane::Tracks => {
                                search_state.tracks_state.list.next(false);
                                if let Some(selected) = search_state.tracks_state.list.state.selected()
                                    && search_state.tracks_state.list.items.len() - selected <= threshold
                                    && !search_state.tracks_state.is_loading
                                    && !search_state.tracks_state.is_end {
                                    let _ = app.network_tx.send(ClientRequest::SearchItems {
                                        query: search_state.input.clone(),
                                        search_types: vec![SearchType::Track],
                                        limit: app.page_limit,
                                        offset: search_state.tracks_state.list.items.len() as u32,
                                    });

                                    search_state.tracks_state.is_loading = true;
                                }
                            }
                            SearchHoveredPane::Artists => {
                                search_state.artists_state.list.next(false);
                                if let Some(selected) = search_state.artists_state.list.state.selected()
                                    && search_state.artists_state.list.items.len() - selected <= threshold
                                    && !search_state.artists_state.is_loading
                                    && !search_state.artists_state.is_end {
                                    let _ = app.network_tx.send(ClientRequest::SearchItems {
                                        query: search_state.input.clone(),
                                        search_types: vec![SearchType::Artist],
                                        limit: app.page_limit,
                                        offset: search_state.artists_state.list.items.len() as u32,
                                    });

                                    search_state.artists_state.is_loading = true;
                                }
                            }
                            SearchHoveredPane::Albums => {
                                search_state.albums_state.list.next(false);
                                if let Some(selected) = search_state.albums_state.list.state.selected()
                                    && search_state.albums_state.list.items.len() - selected <= threshold
                                    && !search_state.albums_state.is_loading
                                    && !search_state.albums_state.is_end {
                                    let _ = app.network_tx.send(ClientRequest::SearchItems {
                                        query: search_state.input.clone(),
                                        search_types: vec![SearchType::Album],
                                        limit: app.page_limit,
                                        offset: search_state.albums_state.list.items.len() as u32,
                                    });

                                    search_state.albums_state.is_loading = true;
                                }
                            }
                            SearchHoveredPane::Playlists => {
                                search_state.playlists_state.list.next(false);
                                if let Some(selected) = search_state.playlists_state.list.state.selected()
                                    && search_state.playlists_state.list.items.len() - selected <= threshold
                                    && !search_state.playlists_state.is_loading
                                    && !search_state.playlists_state.is_end {
                                    let _ = app.network_tx.send(ClientRequest::SearchItems {
                                        query: search_state.input.clone(),
                                        search_types: vec![SearchType::Playlist],
                                        limit: app.page_limit,
                                        offset: search_state.playlists_state.list.items.len() as u32,
                                    });

                                    search_state.playlists_state.is_loading = true;
                                }
                            }
                            _ => {}
                        }
                    }

                    KeyCode::Up | KeyCode::Char('k') => {
                        match search_state.hovered_pane {
                            SearchHoveredPane::Tracks => search_state.tracks_state.list.previous(false),
                            SearchHoveredPane::Artists => search_state.artists_state.list.previous(false),
                            SearchHoveredPane::Albums => search_state.albums_state.list.previous(false),
                            SearchHoveredPane::Playlists => search_state.playlists_state.list.previous(false),
                            _ => {}
                        }
                    }

                    KeyCode::Char('t') => {
                        target_to_open = match search_state.hovered_pane {
                            SearchHoveredPane::Tracks => search_state
                                .tracks_state
                                .list.state
                                .selected()
                                .and_then(|idx| search_state.tracks_state.list.items.get(idx))
                                .map(|track| MenuTarget::Track(track.clone())),

                            SearchHoveredPane::Artists => search_state
                                .artists_state
                                .list.state
                                .selected()
                                .and_then(|idx| search_state.artists_state.list.items.get(idx))
                                .map(|artist| MenuTarget::Artist(artist.clone())),

                            SearchHoveredPane::Albums => search_state
                                .albums_state
                                .list.state
                                .selected()
                                .and_then(|idx| search_state.albums_state.list.items.get(idx))
                                .map(|album| MenuTarget::Album(album.clone())),

                            SearchHoveredPane::Playlists => search_state
                                .playlists_state
                                .list.state
                                .selected()
                                .and_then(|idx| search_state.playlists_state.list.items.get(idx))
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
