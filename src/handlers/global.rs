use crate::app::{ActiveBlock, App, route::Route};
use crate::app::home_state::HomeState;
use crate::app::search_state::{SearchState, SearchHoveredPane};
use crate::app::queue_state::QueueState;
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
            let is_playing = playback.is_playing;
            playback.is_playing = !is_playing;
            let _ = app.network_tx.send(ClientRequest::Player(PlayerRequest::TogglePlayback(is_playing)));
            return true;
        }

        // volume
        if key.code == KeyCode::Char('-') {
            let vol = &mut playback.device.volume;
            *vol = vol.saturating_sub(5);
            let _ = app.network_tx.send(ClientRequest::Player(PlayerRequest::SetVolume(*vol)));
            return true;
        }
        if key.code == KeyCode::Char('+') {
            let vol = &mut playback.device.volume;
            *vol = vol.saturating_add(5).min(100);
            let _ = app.network_tx.send(ClientRequest::Player(PlayerRequest::SetVolume(*vol)));
            return true;
        }
        
        // next
        if key.code == KeyCode::Char('n') {
            let _ = app.network_tx.send(ClientRequest::Player(PlayerRequest::NextTrack));
            return true;
        }

        // prev
        if key.code == KeyCode::Char('p') {
            let _ = app.network_tx.send(ClientRequest::Player(PlayerRequest::PreviousTrack));
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
            let _ = app.network_tx.send(ClientRequest::Player(
                PlayerRequest::SetRepeatMode(state))
            );
            return true;
        }

        // toggle shuffle
        if key.code == KeyCode::Char('s') {
            let shuffling = playback.shuffle_state;
            playback.shuffle_state = !shuffling;
            let _ = app.network_tx.send(ClientRequest::Player(
                PlayerRequest::ToggleShuffle(shuffling))
            );
            return true;
        }
    }

    // ----------------------------------- home -------------------------------------
    if key.code == KeyCode::Char('H') {
        if matches!(app.route, Route::Home(_)) {
            return true; 
        }

        app.set_current_route(Route::Home(HomeState::default())); 
        app.active_block = ActiveBlock::HomeBlock;

        return true;
    }

    // search
    if key.code == KeyCode::Char('S') {
        if !matches!(app.route, Route::Search(_)) {
            app.set_current_route(Route::Search(SearchState::default()));
        }

        app.active_block = ActiveBlock::SearchInput;
        
        return true;
    }

    // queue
    if key.code == KeyCode::Char('Q') {
        if matches!(app.route, Route::Queue(_)) {
            app.active_block = ActiveBlock::QueueBlock;
            return true;
        }

        app.set_current_route(Route::Queue(QueueState::default()));
        app.active_block = ActiveBlock::QueueBlock;
        
        return true;
    }

    // active block
    if key.code == KeyCode::Tab && !key.modifiers.contains(KeyModifiers::CONTROL) {
        if app.active_block == ActiveBlock::SearchResults {
            return false;
        }
        
        app.active_block = match app.active_block {
            ActiveBlock::LibraryMenu => ActiveBlock::PlaylistsMenu,
            ActiveBlock::PlaylistsMenu => match app.route {
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
                if let Route::Search(ref mut search_state) = app.route {
                    search_state.hovered_pane = SearchHoveredPane::Tracks;
                }
                ActiveBlock::SearchResults
            },

            ActiveBlock::HomeBlock 
            | ActiveBlock::PlaylistTracks 
            | ActiveBlock::QueueBlock 
            | ActiveBlock::LyricsText
            | ActiveBlock::AlbumBlock
            | ActiveBlock::LikedSongs
            | ActiveBlock::SavedAlbums
            | ActiveBlock::SavedArtists
            | ActiveBlock::SavedPodcasts => {
                ActiveBlock::Playbar
            },

            ActiveBlock::Playbar => {
                if matches!(app.route, Route::Search(_)) {
                    ActiveBlock::SearchInput 
                } else {
                    ActiveBlock::LibraryMenu
                }
            },

            _ => ActiveBlock::LibraryMenu,
        };
        return true; 
    }

    if key.code == KeyCode::Char('?') {
        app.show_help = true;
        return true;
    }


    false
}
