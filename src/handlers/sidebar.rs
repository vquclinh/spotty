use crate::app::{
    ActiveBlock, App, Route::PlaylistDetail,
    library_state::*, playlist_state::PlaylistState,
    Route, AppState
};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub fn handle_sidebar_events(key: KeyEvent, app: &mut App) {
    let AppState { active_block, .. } = app.state.current_mut();
    
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
                    app.playlists_menu.next(steps, false);
                }

                KeyEvent { code: KeyCode::Up, .. }
                | KeyEvent { code: KeyCode::Char('k'), .. }
                | KeyEvent {
                    code: KeyCode::Char('u'),
                    modifiers: KeyModifiers::CONTROL,
                    ..
                } => {
                    let steps = if key.code == KeyCode::Char('u') { 10 } else { 1 };
                    app.playlists_menu.previous(steps, false);
                }

                KeyEvent { code: KeyCode::Enter, .. } => {
                    if let Some(selected_idx) = app.playlists_menu.state.selected()
                    && let Some(playlist) = app.playlists_menu.items.get(selected_idx).cloned() {
                        app.set_app_state(
                            PlaylistDetail(PlaylistState::new(playlist)),
                            Some(ActiveBlock::PlaylistTracks)
                        );
                    }
                }
                _ => {}
            }
        },

        ActiveBlock::LibraryMenu => {
            match key.code {
                KeyCode::Down | KeyCode::Char('j') => app.library_menu.next(1, true),

                KeyCode::Up | KeyCode::Char('k') => app.library_menu.previous(1, true),

                KeyCode::Enter => {
                    if let Some(selected_idx) = app.library_menu.state.selected()
                    && let Some(library_item) = app.library_menu.items.get(selected_idx).cloned() {
                        match library_item {
                            LibraryMenuItem::LikedSongs(state) => {
                                app.set_app_state(
                                    Route::LikedSongs(state),
                                    Some(ActiveBlock::LikedSongs)
                                );
                            }

                            LibraryMenuItem::SavedAlbums(state) => {
                                app.set_app_state(
                                    Route::SavedAlbums(state),
                                    Some(ActiveBlock::SavedAlbums)
                                );
                            }

                            LibraryMenuItem::SavedArtists(state) => {
                                app.set_app_state(
                                    Route::SavedArtists(state),
                                    Some(ActiveBlock::SavedArtists)
                                );
                            }

                            LibraryMenuItem::SavedPodcasts(state) => {
                                app.set_app_state(
                                    Route::SavedPodcasts(state),
                                    Some(ActiveBlock::SavedPodcasts)
                                );
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
