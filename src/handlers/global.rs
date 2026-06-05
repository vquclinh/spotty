use std::time::Duration;
use crate::app::{ActiveBlock, App, AppState, Route};
use crate::app::home_state::HomeState;
use crate::app::search_state::{SearchState, SearchHoveredPane};
use crate::app::queue_state::QueueState;
use crate::app::lyrics_state::LyricsState;
use crossterm::event::{KeyCode, KeyModifiers, KeyEvent};
use crate::network::request::{ClientRequest, PlayerRequest};
use crate::network::models::*;

pub fn handle_global_events(key: KeyEvent, app: &mut App) -> bool {
    // -------------------------------------- quit ----------------------------------------
    if key.code == KeyCode::Char('q') 
        || (key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL)) 
    {
        app.should_quit = true;
        return true;
    }

    // ----------------------------------- playbar -------------------------------------
    if let Some(playback) = &mut app.playback {
        // pause/resume
        if key.code == KeyCode::Char(' ') {
            if app.track_ended {
                if let Some(item) = &playback.item {
                    let uri = match item {
                        PlayableItem::Track(t) => t.uri.clone(),
                        PlayableItem::Episode(e) => e.uri.clone(),
                    };
                    playback.is_playing = true;
                    playback.progress = Duration::from_millis(0);
                    app.track_ended = false;
                    let _ = app.network_tx.send(ClientRequest::Player {
                        request: PlayerRequest::Play(uri),
                        is_active_device: app.device_state.is_active_device()
                    });
                }
            } else {
                let is_playing = playback.is_playing;
                playback.is_playing = !is_playing;
                let _ = app.network_tx.send(ClientRequest::Player {
                    request: PlayerRequest::TogglePlayback(is_playing),
                    is_active_device: app.device_state.is_active_device()
                });
            }
            return true;
        }

        // volume
        if key.code == KeyCode::Char('-') {
            let vol = &mut playback.device.volume;
            *vol = vol.saturating_sub(5);
            app.app_cache.volume = *vol;
            let _ = app.network_tx.send(ClientRequest::Player {
                request: PlayerRequest::SetVolume(*vol),
                is_active_device: app.device_state.is_active_device()
            });
            return true;
        }
        if key.code == KeyCode::Char('+') {
            let vol = &mut playback.device.volume;
            *vol = vol.saturating_add(5).min(100);
            app.app_cache.volume = *vol;
            let _ = app.network_tx.send(ClientRequest::Player {
                request: PlayerRequest::SetVolume(*vol),
                is_active_device: app.device_state.is_active_device()
            });
            return true;
        }
        
        // next
        if key.code == KeyCode::Char('n') {
            let _ = app.network_tx.send(ClientRequest::Player {
                request: PlayerRequest::NextTrack,
                is_active_device: app.device_state.is_active_device()
            });
            return true;
        }

        // prev
        if key.code == KeyCode::Char('p') {
            let _ = app.network_tx.send(ClientRequest::Player {
                request: PlayerRequest::PreviousTrack,
                is_active_device: app.device_state.is_active_device()
            });
            return true;
        }

        // cycle repeat
        if key.code == KeyCode::Char('r') {
            let state = match playback.repeat_state {
                RepeatState::Off => RepeatState::Context,
                RepeatState::Context => RepeatState::Track,
                RepeatState::Track => RepeatState::Off,
            };
            playback.repeat_state = state;
            let _ = app.network_tx.send(ClientRequest::Player {
                request: PlayerRequest::SetRepeatMode(state),
                is_active_device: app.device_state.is_active_device()
            });
            return true;
        }

        // toggle shuffle
        if key.code == KeyCode::Char('s') {
            let shuffling = playback.shuffle_state;
            playback.shuffle_state = !shuffling;
            if let Some(uri) = &playback.context_uri {
                app.app_cache.shuffle_state.insert(uri.clone(), !shuffling);
            }
            let _ = app.network_tx.send(ClientRequest::Player {
                request: PlayerRequest::ToggleShuffle(shuffling),
                is_active_device: app.device_state.is_active_device()
            });
            return true;
        }
    }

    // ----------------------------------- go back state -------------------------------------
    if key.code == KeyCode::Char('b') {
        if app.state.previous().is_some() {
            // The usage here borrows app so we cannot destructure AppState yet
            app.state.pop_state();
            return true;
        }
        // If we can't pop state then return false to continue checking for an operation
        return false;
    }

    // Only from here can we destructure AppState
    let AppState { route, active_block } = app.state.current_mut();

    // ----------------------------------- home -------------------------------------
    if key.code == KeyCode::Char('H') {
        if !matches!(route, Route::Home(_)) {
            app.set_app_state(
                Route::Home(HomeState::default()),
                Some(ActiveBlock::HomeBlock)
            ); 
        }
        return true;
    }

    // ----------------------------------- search -----------------------------------
    if key.code == KeyCode::Char('S') {
        if !matches!(route, Route::Search(_)) {
            app.set_app_state(
                Route::Search(SearchState::default()),
                Some(ActiveBlock::SearchInput)
            );
        }
        return true;
    }

    // ------------------------------------ queue -----------------------------------
    if key.code == KeyCode::Char('Q') {
        if !matches!(route, Route::Queue(_)) {
            app.set_app_state(
                Route::Queue(QueueState::default()),
                Some(ActiveBlock::QueueBlock)
            );
        }
        return true;
    }

    // ------------------------------------- lyrics ----------------------------------
    if key.code == KeyCode::Char('L') {
        if !matches!(route, Route::Lyrics(_)) {
            app.set_app_state(
                Route::Lyrics(LyricsState::default()),
                Some(ActiveBlock::LyricsText)
            );
        }
        return true;
    }

    // ----------------------------------- active block -------------------------------
    if key.code == KeyCode::Tab && !key.modifiers.contains(KeyModifiers::CONTROL) {
        if *active_block == ActiveBlock::SearchResults {
            return false;
        }
        
        *active_block = match *active_block {
            ActiveBlock::LibraryMenu => ActiveBlock::PlaylistsMenu,
            ActiveBlock::PlaylistsMenu => match route {
                Route::PlaylistDetail(_) => ActiveBlock::PlaylistTracks,
                Route::Search(_) => ActiveBlock::SearchInput,
                Route::Queue(_) => ActiveBlock::QueueBlock,
                Route::AlbumDetail(_) => ActiveBlock::AlbumBlock,

                Route::LikedSongs(_) => ActiveBlock::LikedSongs,
                Route::SavedAlbums(_) => ActiveBlock::SavedAlbums,
                Route::SavedArtists(_) => ActiveBlock::SavedArtists,
                Route::SavedPodcasts(_) => ActiveBlock::SavedPodcasts,
                _ => ActiveBlock::HomeBlock,
            },

            ActiveBlock::SearchInput => {
                if let Route::Search(search_state) = route {
                    search_state.hovered_pane = SearchHoveredPane::Tracks;
                }
                ActiveBlock::SearchResults
            },

            ActiveBlock::LyricsText => ActiveBlock::LyricsInfo,

            ActiveBlock::HomeBlock 
            | ActiveBlock::PlaylistTracks 
            | ActiveBlock::QueueBlock 
            | ActiveBlock::AlbumBlock
            | ActiveBlock::LikedSongs
            | ActiveBlock::SavedAlbums
            | ActiveBlock::SavedArtists
            | ActiveBlock::SavedPodcasts
            | ActiveBlock::LyricsInfo => {
                ActiveBlock::Playbar
            },

            ActiveBlock::Playbar => match route {
                Route::Search(_) => ActiveBlock::SearchInput,
                Route::Lyrics(_) => ActiveBlock::LyricsText,
                _ => ActiveBlock::LibraryMenu,
            },

            _ => {
                if matches!(route, Route::Lyrics(_)) {
                    ActiveBlock::LyricsText
                } else {
                    ActiveBlock::LibraryMenu
                }
            },
        };
        return true; 
    }

    // -------------------------------- number keys ----------------------------------
    match key.code {
        KeyCode::Char('1') => {
            match route {
                Route::Search(_) => {
                    *active_block = ActiveBlock::SearchInput;
                }
                Route::Lyrics(_) => {
                    *active_block = ActiveBlock::LyricsText;
                }
                _ => {
                    *active_block = ActiveBlock::LibraryMenu;
                }
            }

        },
        KeyCode::Char('2') => {
            match route {
                Route::Search(_) => {
                    *active_block = ActiveBlock::SearchResults;
                }
                Route::Lyrics(_) => {
                    *active_block = ActiveBlock::LyricsInfo;
                }
                _ => {
                    *active_block = ActiveBlock::PlaylistsMenu;
                }
            }
        },
        KeyCode::Char('3') => {
            match route {
                Route::Home(_) => {
                    *active_block = ActiveBlock::HomeBlock;
                },
                Route::AlbumDetail(_) => {
                    *active_block = ActiveBlock::AlbumBlock;
                },
                Route::PlaylistDetail(_) => {
                    *active_block = ActiveBlock::PlaylistTracks;
                },
                Route::LikedSongs(_) => {
                    *active_block = ActiveBlock::LikedSongs;
                },
                Route::SavedAlbums(_) => {
                    *active_block = ActiveBlock::SavedAlbums;
                },
                Route::SavedArtists(_) => {
                    *active_block = ActiveBlock::SavedArtists;
                },
                Route::SavedPodcasts(_) => {
                    *active_block = ActiveBlock::SavedPodcasts;
                },
                Route::Queue(_) => {
                    *active_block = ActiveBlock::QueueBlock;
                },
                Route::Lyrics(_) => {
                    *active_block = ActiveBlock::Playbar;
                },
                Route::Search(_) => {
                    *active_block = ActiveBlock::Playbar;
                }
                _ => {}
            }
        },
        KeyCode::Char('4') if !matches!(route, Route::Search(_) | Route::Lyrics(_)) => {
            *active_block = ActiveBlock::Playbar;
        }

        _ => {}
    }

    if key.code == KeyCode::Char('?') {
        app.show_help = true;
        return true;
    }

    if key.code == KeyCode::Char('g') {
        app.update_quick_actions();
        app.show_quick_actions = true;
        return true;
    }

    false
}
