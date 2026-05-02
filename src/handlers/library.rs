use crate::app::{ActiveBlock, App, route::Route};
use crossterm::event::{KeyCode, KeyEvent};
use crate::network::models::*;
use crate::network::request::ClientRequest;

pub fn handle_liked_songs_events(key: KeyEvent, app: &mut App) {
    let mut target_to_open = None;

    if let Route::LikedSongs(liked_songs) = &mut app.route {
        match key.code {
            KeyCode::Down | KeyCode::Char('j') => {
                liked_songs.tracks.next(false);
                let threshold = 20;

                if let Some(selected) = liked_songs.tracks.state.selected()
                    && liked_songs.tracks.items.len() - selected <= threshold
                    && !liked_songs.is_loading
                    && !liked_songs.is_end {
                    let _ = app.network_tx.send(ClientRequest::GetUserLikedSongs {
                        limit: app.page_limit,
                        offset: liked_songs.tracks.items.len() as u32
                    });

                    liked_songs.is_loading = true;
                }
            }
            KeyCode::Up | KeyCode::Char('k') => liked_songs.tracks.previous(false),
            
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
            KeyCode::Down | KeyCode::Char('j') => {
                saved_albums.albums.next(false);
                let threshold = 20;

                if let Some(selected) = saved_albums.albums.state.selected()
                    && saved_albums.albums.items.len() - selected <= threshold
                    && !saved_albums.is_loading
                    && !saved_albums.is_end {
                    let _ = app.network_tx.send(ClientRequest::GetUserSavedAlbums {
                        limit: app.page_limit,
                        offset: saved_albums.albums.items.len() as u32
                    });

                    saved_albums.is_loading = true;
                }
            }
            KeyCode::Up | KeyCode::Char('k') => saved_albums.albums.previous(false),
            
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
            KeyCode::Down | KeyCode::Char('j') => {
                saved_artists.artists.next(false);
                let threshold = 20;

                if let Some(selected) = saved_artists.artists.state.selected()
                    && saved_artists.artists.items.len() - selected <= threshold
                    && !saved_artists.is_loading
                    && !saved_artists.is_end {
                    let _ = app.network_tx.send(ClientRequest::GetUserSavedArtists {
                        limit: app.page_limit,
                        after: saved_artists.artists.items
                            .last().map(|a| a.id.clone())
                    });

                    saved_artists.is_loading = true;
                }
            }
            KeyCode::Up | KeyCode::Char('k') => saved_artists.artists.previous(false),
            
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
            KeyCode::Down | KeyCode::Char('j') => {
                saved_podcasts.podcasts.next(false);
                let threshold = 20;

                if let Some(selected) = saved_podcasts.podcasts.state.selected()
                    && saved_podcasts.podcasts.items.len() - selected <= threshold
                    && !saved_podcasts.is_loading
                    && !saved_podcasts.is_end {
                    let _ = app.network_tx.send(ClientRequest::GetUserSavedPodcasts {
                        limit: app.page_limit,
                        offset: saved_podcasts.podcasts.items.len() as u32
                    });

                    saved_podcasts.is_loading = true;
                }
            }
            KeyCode::Up | KeyCode::Char('k') => saved_podcasts.podcasts.previous(false),
            
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
