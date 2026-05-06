use crate::app::{ActiveBlock, App, library_state::*, playlist_state::PlaylistState, route::Route};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crate::ClientRequest;

pub fn handle_sidebar_events(key: KeyEvent, app: &mut App) {
    let App { playlists_menu, library_menu, route, active_block, network_tx, .. } = app;

    match *active_block {
        ActiveBlock::PlaylistsMenu => {
            match key {
                KeyEvent{ code: KeyCode::Down, .. }
                | KeyEvent { code: KeyCode::Char('j'), ..}
                | KeyEvent {
                    code: KeyCode::Char('d'),
                    modifiers: KeyModifiers::CONTROL,
                    ..
                } => {
                    let steps = if key.code == KeyCode::Char('d') { 10 } else { 1 };
                    playlists_menu.next(steps, false);
                }

                KeyEvent { code: KeyCode::Up, .. }
                | KeyEvent { code: KeyCode::Char('k'), .. }
                | KeyEvent {
                    code: KeyCode::Char('u'),
                    modifiers: KeyModifiers::CONTROL,
                    ..
                } => {
                    let steps = if key.code == KeyCode::Char('u') { 10 } else { 1 };
                    playlists_menu.previous(steps, false);
                }

                KeyEvent { code: KeyCode::Enter, .. } => {
                    if let Some(selected_idx) = playlists_menu.state.selected()
                    && let Some(playlist) = playlists_menu.items.get(selected_idx).cloned() {
                        *route = Route::PlaylistDetail(PlaylistState::new(playlist.clone()));
                        *active_block = ActiveBlock::PlaylistTracks; 

                        let _ = network_tx.send(ClientRequest::GetPlaylistItems { 
                            playlist_id: playlist.id,
                            limit: 50, 
                            offset: 0
                        });
                    }
                }
                _ => {}
            }
        },

        ActiveBlock::LibraryMenu => {
            match key.code {
                KeyCode::Down | KeyCode::Char('j') => library_menu.next(1, true),

                KeyCode::Up | KeyCode::Char('k') => library_menu.previous(1, true),

                KeyCode::Enter => {
                    if let Some(selected_idx) = library_menu.state.selected()
                    && let Some(library_item) = library_menu.items.get(selected_idx).cloned() {
                        match library_item {
                            LibraryMenuItem::LikedSongs(state) => {
                                *route = Route::LikedSongs(LikedSongsState::new(state.tracks.items.clone()));
                                *active_block = ActiveBlock::LikedSongs; 

                                let _ = network_tx.send(ClientRequest::GetUserLikedSongs { 
                                    limit: 50, 
                                    offset: 0 
                                });
                            }

                            LibraryMenuItem::SavedAlbums(state) => {
                                *route = Route::SavedAlbums(SavedAlbumsState::new(state.albums.items.clone()));
                                *active_block = ActiveBlock::SavedAlbums; 

                                let _ = network_tx.send(ClientRequest::GetUserSavedAlbums { 
                                    limit: 50, 
                                    offset: 0 
                                });
                            }

                            LibraryMenuItem::SavedArtists(state) => {
                                *route = Route::SavedArtists(SavedArtistsState::new(state.artists.items.clone()));
                                *active_block = ActiveBlock::SavedArtists; 

                                let _ = network_tx.send(ClientRequest::GetUserSavedArtists { 
                                    limit: 50, 
                                    after: None
                                });
                            }

                            LibraryMenuItem::SavedPodcasts(state) => {
                                *route = Route::SavedPodcasts(SavedPodcastsState::new(state.podcasts.items.clone()));
                                *active_block = ActiveBlock::SavedPodcasts; 

                                let _ = network_tx.send(ClientRequest::GetUserSavedPodcasts { 
                                    limit: 50, 
                                    offset: 0 
                                });
                            }
                        }
                    }
                }
                _ => {}
            }
        },

        _ => {}
    }
}
