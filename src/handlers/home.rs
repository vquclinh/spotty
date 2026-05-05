use crate::app::{App, route::Route, home_state::HomeTab};
use crossterm::event::{KeyCode, KeyEvent};
use crate::ClientRequest;
use crate::network::models::*;

pub fn handle_home_events(key: KeyEvent, app: &mut App) {
    let App { route, network_tx, action_menu, .. } = app;

    if let Route::Home(home_state) = route {
        match key.code {
            KeyCode::Char('1') => {
                home_state.active_tab = HomeTab::TopTracks;
                if home_state.top_tracks.list.items.is_empty() {
                    let _ = network_tx.send(ClientRequest::GetUserTopTracks {
                        time_range: TimeRange::ShortTerm,
                        limit: app.page_limit,
                        offset: 0,
                    });
                }
            }
            KeyCode::Char('2') => {
                home_state.active_tab = HomeTab::TopArtists;
                if home_state.top_artists.list.items.is_empty() {
                    let _ = network_tx.send(ClientRequest::GetUserTopArtists {
                        time_range: TimeRange::ShortTerm,
                        limit: app.page_limit,
                        offset: 0,
                    });
                }
            }
            KeyCode::Char('3') => {
                home_state.active_tab = HomeTab::RecentlyPlayed;
                if home_state.recent_tracks.list.items.is_empty() {
                    let _ = network_tx.send(ClientRequest::GetRecentlyPlayed {
                        limit: app.page_limit,
                        after: None,
                    });
                }
            }

            KeyCode::Right | KeyCode::Char('l') => {
                home_state.active_tab = match home_state.active_tab {
                    HomeTab::TopTracks => HomeTab::TopArtists,
                    HomeTab::TopArtists => HomeTab::RecentlyPlayed,
                    HomeTab::RecentlyPlayed => HomeTab::TopTracks,
                };
                match home_state.active_tab {
                    HomeTab::TopTracks => {
                        let _ = network_tx.send(ClientRequest::GetUserTopTracks {
                            time_range: TimeRange::ShortTerm,
                            limit: app.page_limit,
                            offset: 0,
                        });
                    }
                    HomeTab::RecentlyPlayed => {
                        let _ = network_tx.send(ClientRequest::GetRecentlyPlayed {
                            limit: app.page_limit,
                            after: None,
                        });
                    }
                    HomeTab::TopArtists => {
                        let _ = network_tx.send(ClientRequest::GetUserTopArtists {
                            time_range: TimeRange::ShortTerm,
                            limit: app.page_limit,
                            offset: 0,
                        });
                    }
                }
            }

            KeyCode::Left | KeyCode::Char('h') => {
                home_state.active_tab = match home_state.active_tab {
                    HomeTab::TopTracks => HomeTab::RecentlyPlayed,
                    HomeTab::TopArtists => HomeTab::TopTracks,
                    HomeTab::RecentlyPlayed => HomeTab::TopArtists,
                };
                match home_state.active_tab {
                    HomeTab::TopTracks => {
                        let _ = network_tx.send(ClientRequest::GetUserTopTracks {
                            time_range: TimeRange::ShortTerm,
                            limit: app.page_limit,
                            offset: 0,
                        });
                    }
                    HomeTab::RecentlyPlayed => {
                        let _ = network_tx.send(ClientRequest::GetRecentlyPlayed {
                            limit: app.page_limit,
                            after: None,
                        });
                    }
                    HomeTab::TopArtists => {
                        let _ = network_tx.send(ClientRequest::GetUserTopArtists {
                            time_range: TimeRange::ShortTerm,
                            limit: app.page_limit,
                            offset: 0,
                        });
                    }
                }
            }

            KeyCode::Down | KeyCode::Char('j') => {
                let threshold = 20;

                match home_state.active_tab {
                    HomeTab::TopTracks => {
                        home_state.top_tracks.list.next(false);

                        if let Some(selected) = home_state.top_tracks.list.state.selected()
                            && home_state.top_tracks.list.items.len() - selected <= threshold
                            && !home_state.top_tracks.is_loading
                            && !home_state.top_tracks.is_end
                        {
                            let _ = network_tx.send(ClientRequest::GetUserTopTracks {
                                time_range: TimeRange::ShortTerm,
                                limit: app.page_limit,
                                offset: home_state.top_tracks.list.items.len() as u32,
                            });
                            home_state.top_tracks.is_loading = true;
                        }
                    }
                    HomeTab::TopArtists => {
                        home_state.top_artists.list.next(false);

                        if let Some(selected) = home_state.top_artists.list.state.selected()
                            && home_state.top_artists.list.items.len() - selected <= threshold
                            && !home_state.top_artists.is_loading
                            && !home_state.top_artists.is_end
                        {
                            let _ = network_tx.send(ClientRequest::GetUserTopArtists {
                                time_range: TimeRange::ShortTerm,
                                limit: app.page_limit,
                                offset: home_state.top_artists.list.items.len() as u32,
                            });
                            home_state.top_artists.is_loading = true;
                        }
                    }
                    // Currently this endpoint works with a time-based cursor, while we only
                    // handle index-based and id-based cursor for now
                    HomeTab::RecentlyPlayed => home_state.recent_tracks.list.next(false),
                }
            }
            KeyCode::Up | KeyCode::Char('k') => {
                match home_state.active_tab {
                    HomeTab::TopTracks => home_state.top_tracks.list.previous(false),
                    HomeTab::TopArtists => home_state.top_artists.list.previous(false),
                    HomeTab::RecentlyPlayed => home_state.recent_tracks.list.previous(false),
                }
            }
            KeyCode::Char('t') => {
                let target = match home_state.active_tab {
                    HomeTab::TopTracks => home_state
                        .top_tracks
                        .list.state
                        .selected()
                        .and_then(|idx| home_state.top_tracks.list.items.get(idx))
                        .map(|t| MenuTarget::Track(t.clone())),
                    HomeTab::RecentlyPlayed => home_state
                        .recent_tracks
                        .list.state
                        .selected()
                        .and_then(|idx| home_state.recent_tracks.list.items.get(idx))
                        .map(|t| MenuTarget::Track(t.clone())),
                    HomeTab::TopArtists => home_state
                        .top_artists
                        .list.state
                        .selected()
                        .and_then(|idx| home_state.top_artists.list.items.get(idx))
                        .map(|a| MenuTarget::Artist(a.clone())),
                };

                if let Some(t) = target {
                    action_menu.open(t, route);
                }
            }
            _ => {}
        }
    }
}
