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
                let _ = network_tx.send(ClientRequest::GetUserTopTracks { time_range: TimeRange::ShortTerm, limit: 15, offset: 0 });
            },
            KeyCode::Char('2') => {
                home_state.active_tab = HomeTab::TopArtists;
                let _ = network_tx.send(ClientRequest::GetUserTopArtists { time_range: TimeRange::ShortTerm, limit: 15, offset: 0 });
            },
            KeyCode::Char('3') => {
                home_state.active_tab = HomeTab::RecentlyPlayed;
                let _ = network_tx.send(ClientRequest::GetRecentlyPlayed { limit: 15, after: None });
            },
            
            KeyCode::Right | KeyCode::Char('l') => {
                home_state.active_tab = match home_state.active_tab {
                    HomeTab::TopTracks => HomeTab::TopArtists,
                    HomeTab::TopArtists => HomeTab::RecentlyPlayed,
                    HomeTab::RecentlyPlayed => HomeTab::TopTracks,
                };
                match home_state.active_tab {
                    HomeTab::TopTracks => { let _ = network_tx.send(ClientRequest::GetUserTopTracks { time_range: TimeRange::ShortTerm, limit: 15, offset: 0 }); }
                    HomeTab::RecentlyPlayed => { let _ = network_tx.send(ClientRequest::GetRecentlyPlayed { limit: 15, after: None }); }
                    HomeTab::TopArtists => { let _ = network_tx.send(ClientRequest::GetUserTopArtists { time_range: TimeRange::ShortTerm, limit: 15, offset: 0 }); }
                }
            }

            KeyCode::Left | KeyCode::Char('h') => {
                home_state.active_tab = match home_state.active_tab {
                    HomeTab::TopTracks => HomeTab::RecentlyPlayed,
                    HomeTab::TopArtists => HomeTab::TopTracks,
                    HomeTab::RecentlyPlayed => HomeTab::TopArtists,
                };
                match home_state.active_tab {
                    HomeTab::TopTracks => { let _ = network_tx.send(ClientRequest::GetUserTopTracks { time_range: TimeRange::ShortTerm, limit: 15, offset: 0 }); }
                    HomeTab::RecentlyPlayed => { let _ = network_tx.send(ClientRequest::GetRecentlyPlayed { limit: 15, after: None }); }
                    HomeTab::TopArtists => { let _ = network_tx.send(ClientRequest::GetUserTopArtists { time_range: TimeRange::ShortTerm, limit: 15, offset: 0 }); }
                }
            }

            KeyCode::Down | KeyCode::Char('j') => {
                match home_state.active_tab {
                    HomeTab::TopTracks => home_state.top_tracks.next(false),
                    HomeTab::TopArtists => home_state.top_artists.next(false),
                    HomeTab::RecentlyPlayed => home_state.recent_tracks.next(false),
                }
            }
            KeyCode::Up | KeyCode::Char('k') => {
                match home_state.active_tab {
                    HomeTab::TopTracks => home_state.top_tracks.previous(false),
                    HomeTab::TopArtists => home_state.top_artists.previous(false),
                    HomeTab::RecentlyPlayed => home_state.recent_tracks.previous(false),
                }
            }
            KeyCode::Char('t') => {
                let target = match home_state.active_tab {
                    HomeTab::TopTracks => home_state.top_tracks.state.selected()
                        .and_then(|idx| home_state.top_tracks.items.get(idx))
                        .map(|t| MenuTarget::Track(t.clone())),
                    HomeTab::RecentlyPlayed => home_state.recent_tracks.state.selected()
                        .and_then(|idx| home_state.recent_tracks.items.get(idx))
                        .map(|t| MenuTarget::Track(t.clone())),
                    HomeTab::TopArtists => home_state.top_artists.state.selected()
                        .and_then(|idx| home_state.top_artists.items.get(idx))
                        .map(|a| MenuTarget::Artist(a.clone())),
                };

                if let Some(t) = target {
                    action_menu.open(t, &route);
                }
            }
            _ => {}
        }
    }
}
