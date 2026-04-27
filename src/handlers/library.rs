use crate::app::{ActiveBlock, App, route::Route};
use crossterm::event::{KeyCode, KeyEvent};
use crate::network::models::*;

pub fn handle_liked_songs_events(key: KeyEvent, app: &mut App) {
    let mut target_to_open = None;

    if let Route::LikedSongs(liked_songs) = &mut app.route {
        match key.code {
            KeyCode::Down | KeyCode::Char('j') => liked_songs.tracks.next(),
            KeyCode::Up | KeyCode::Char('k') => liked_songs.tracks.previous(),
            
            KeyCode::Char('t') => {
                target_to_open = liked_songs.tracks.state.selected()
                    .and_then(|idx| liked_songs.tracks.items.get(idx))
                    .map(|track| MenuTarget::Track(track.clone()));
            }

            KeyCode::Backspace | KeyCode::Char('b') => {
                app.active_block = ActiveBlock::LikedSongs;
            }
            _ => {}
        }
    }
    
    if let Some(target) = target_to_open {
        app.action_menu.open(target, &app.route);
    }
}

pub fn handle_saved_albums_events(key: KeyEvent, app: &mut App) {
    let mut target_to_open = None;

    if let Route::SavedAlbums(saved_albums) = &mut app.route {
        match key.code {
            KeyCode::Down | KeyCode::Char('j') => saved_albums.albums.next(),
            KeyCode::Up | KeyCode::Char('k') => saved_albums.albums.previous(),
            
            KeyCode::Char('t') => {
                target_to_open = saved_albums.albums.state.selected()
                    .and_then(|idx| saved_albums.albums.items.get(idx))
                    .map(|album| MenuTarget::Album(album.clone()));
            }

            KeyCode::Backspace | KeyCode::Char('b') => {
                app.active_block = ActiveBlock::SavedAlbums;
            }
            _ => {}
        }
    }
    
    if let Some(target) = target_to_open {
        app.action_menu.open(target, &app.route);
    }
}

pub fn handle_saved_artists_events(key: KeyEvent, app: &mut App) {
    let mut target_to_open = None;

    if let Route::SavedArtists(saved_artists) = &mut app.route {
        match key.code {
            KeyCode::Down | KeyCode::Char('j') => saved_artists.artists.next(),
            KeyCode::Up | KeyCode::Char('k') => saved_artists.artists.previous(),
            
            KeyCode::Char('t') => {
                target_to_open = saved_artists.artists.state.selected()
                    .and_then(|idx| saved_artists.artists.items.get(idx))
                    .map(|artist| MenuTarget::Artist(artist.clone()));
            }

            KeyCode::Backspace | KeyCode::Char('b') => {
                app.active_block = ActiveBlock::SavedArtists;
            }
            _ => {}
        }
    }
    
    if let Some(target) = target_to_open {
        app.action_menu.open(target, &app.route);
    }
}

pub fn handle_saved_podcasts_events(key: KeyEvent, app: &mut App) {
    let mut target_to_open = None;

    if let Route::SavedPodcasts(saved_podcasts) = &mut app.route {
        match key.code {
            KeyCode::Down | KeyCode::Char('j') => saved_podcasts.podcasts.next(),
            KeyCode::Up | KeyCode::Char('k') => saved_podcasts.podcasts.previous(),
            
            KeyCode::Char('t') => {
                target_to_open = saved_podcasts.podcasts.state.selected()
                    .and_then(|idx| saved_podcasts.podcasts.items.get(idx))
                    .map(|episode| MenuTarget::Episode(episode.clone()));
            }

            KeyCode::Backspace | KeyCode::Char('b') => {
                app.active_block = ActiveBlock::SavedPodcasts;
            }
            _ => {}
        }
    }
    
    if let Some(target) = target_to_open {
        app.action_menu.open(target, &app.route);
    }
}
