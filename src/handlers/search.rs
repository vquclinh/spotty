use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
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
                match key {
                    KeyEvent { code: KeyCode::Tab, .. } => {
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

                    KeyEvent{ code: KeyCode::Down, .. }
                    | KeyEvent { code: KeyCode::Char('j'), ..}
                    | KeyEvent {
                        code: KeyCode::Char('d'),
                        modifiers: KeyModifiers::CONTROL,
                        ..
                    } => {
                        let steps = if key.code == KeyCode::Char('d') { 10 } else { 1 };
                        match search_state.hovered_pane {
                            SearchHoveredPane::Tracks => search_state.tracks_state.next(steps, false),
                            SearchHoveredPane::Artists => search_state.artists_state.next(steps, false),
                            SearchHoveredPane::Albums => search_state.albums_state.next(steps, false),
                            SearchHoveredPane::Playlists => search_state.playlists_state.next(steps, false),
                            _ => {}
                        }
                    }

                    KeyEvent { code: KeyCode::Up, .. }
                    | KeyEvent { code: KeyCode::Char('k'), .. }
                    | KeyEvent {
                        code: KeyCode::Char('u'),
                        modifiers: KeyModifiers::CONTROL,
                        ..
                    } => {
                        let steps = if key.code == KeyCode::Char('u') { 10 } else { 1 };
                        match search_state.hovered_pane {
                            SearchHoveredPane::Tracks => search_state.tracks_state.previous(steps, false),
                            SearchHoveredPane::Artists => search_state.artists_state.previous(steps, false),
                            SearchHoveredPane::Albums => search_state.albums_state.previous(steps, false),
                            SearchHoveredPane::Playlists => search_state.playlists_state.previous(steps, false),
                            _ => {}
                        }
                    }

                    KeyEvent { code: KeyCode::Char('t'), .. } => {
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

                    KeyEvent { code: KeyCode::Esc, .. } => {
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
        let _ = app.network_tx.send(ClientRequest::SearchItemsUpTo {
            query,
            search_types: types,
            total_limit: 20,
            start_offset: 0
        });
    }
}
