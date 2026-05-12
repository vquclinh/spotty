use crate::app::*;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
};
use crate::app::home_state::*;
use crate::app::search_state::*;

use super::{home, splash, lyrics, playbar, queue, search, sidebar, playlist, album, popups, library};

pub fn draw(f: &mut Frame, app: &mut App) {
    // splash
    if let Route::Splash(splash_state) = &app.route {
        splash::draw(f, splash_state, f.area());
        return; 
    }

    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(f.area());

    playbar::draw(f, app, main_chunks[1]);

    match &mut app.route {
        Route::Lyrics(_) => {
            let lyrics_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
                .split(main_chunks[0]);

            lyrics::draw_text(f, app, lyrics_chunks[0]);
            lyrics::draw_info(f, app, lyrics_chunks[1]);
        }

        Route::Search(_) => {
            search::draw(f, app, main_chunks[0]);
        }

        // state that has sidebar and library
        _ => {
            let content_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(22), Constraint::Percentage(78)])
                .split(main_chunks[0]);

            sidebar::draw(f, app, content_chunks[0]);

            match &mut app.route {
                Route::Home(home_state) => home::draw(f, home_state, &app.active_block, content_chunks[1]),
                Route::Queue(queue_state) => queue::draw(f, queue_state, &app.active_block, content_chunks[1]),
                Route::AlbumDetail(album_state) => album::draw(f, album_state, &app.active_block, content_chunks[1]),
                Route::PlaylistDetail(playlist_state) => playlist::draw(f, playlist_state, &app.active_block, content_chunks[1]),
                Route::LikedSongs(liked_songs_state) => library::draw_liked_songs(f, liked_songs_state, &app.active_block, content_chunks[1]),
                Route::SavedAlbums(saved_albums_state) => library::draw_saved_albums(f, saved_albums_state, &app.active_block, content_chunks[1]),
                Route::SavedArtists(saved_artists_state) => library::draw_saved_artists(f, saved_artists_state, &app.active_block, content_chunks[1]),
                Route::SavedPodcasts(saved_podcasts_state) => library::draw_saved_podcasts(f, saved_podcasts_state, &app.active_block, content_chunks[1]),
                _ => {}
            }
        }
    }

    if app.show_help {
        popups::draw_help(f, f.area());
    } else if app.show_quick_actions {
        popups::draw_quick_actions(f, f.area());
    } else if app.action_menu.is_open {
        let (area, selected, offset) = match &app.route {
            Route::Home(h) => {
                let (idx, off) = match h.active_tab {
                    HomeTab::TopTracks => (h.top_tracks.list.state.selected(), h.top_tracks.list.state.offset()),
                    HomeTab::RecentlyPlayed => (h.recent_tracks.list.state.selected(), h.recent_tracks.list.state.offset()),
                    HomeTab::TopArtists => (h.top_artists.list.state.selected(), h.top_artists.list.state.offset()),
                };
                (h.last_area, idx.unwrap_or(0), off)
            }
            
            Route::PlaylistDetail(p) => {
                (p.last_area, p.tracks.state.selected().unwrap_or(0), p.tracks.state.offset())
            }

            Route::Search(s) => {
                let (idx, off) = match s.hovered_pane {
                    SearchHoveredPane::Tracks => (s.tracks_state.state.selected(), s.tracks_state.state.offset()),
                    SearchHoveredPane::Artists => (s.artists_state.state.selected(), s.artists_state.state.offset()),
                    SearchHoveredPane::Albums => (s.albums_state.state.selected(), s.albums_state.state.offset()),
                    SearchHoveredPane::Playlists => (s.playlists_state.state.selected(), s.playlists_state.state.offset()),
                    _ => (None, 0),
                };
                (s.last_area, idx.unwrap_or(0), off)
            }

            Route::Queue(q) => {
                (
                    q.last_area, 
                    q.queue_items.state.selected().unwrap_or(0),
                    q.queue_items.state.offset()
                )
            }

            Route::AlbumDetail(a) => {
                (
                    a.last_area, 
                    a.tracks.state.selected().unwrap_or(0), 
                    a.tracks.state.offset()
                )
            }
            
            Route::LikedSongs(a) => {
                (
                    a.last_area, 
                    a.tracks.state.selected().unwrap_or(0), 
                    a.tracks.state.offset()
                )
            }

            Route::SavedAlbums(a) => {
                (
                    a.last_area, 
                    a.albums.state.selected().unwrap_or(0), 
                    a.albums.state.offset()
                )
            }
            
            Route::SavedArtists(a) => {
                (
                    a.last_area, 
                    a.artists.state.selected().unwrap_or(0), 
                    a.artists.state.offset()
                )
            }
            
            Route::SavedPodcasts(a) => {
                (
                    a.last_area, 
                    a.podcasts.state.selected().unwrap_or(0), 
                    a.podcasts.state.offset()
                )
            }

            _ => (f.area(), 0, 0)
        };

        popups::draw_action_menu(f, app, area, selected, offset);
    }
}

pub fn truncate(text: &str, max_width: u16) -> String {
    let max_width = max_width as usize;
    let char_count = text.chars().count();
    
    if char_count > max_width {
        if max_width <= 3 {
            return text.chars().take(max_width).collect();
        }
        
        let truncated: String = text.chars().take(max_width - 3).collect();
        format!("{}...", truncated)
    } else {
        text.to_string()
    }
}
