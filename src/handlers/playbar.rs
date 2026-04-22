use crate::app::{ActiveBlock, App, route::Route};
use crate::app::queue_state::QueueState;
use crate::network::request;
use crate::network::models::RepeatState;
use crossterm::event::{KeyCode, KeyEvent};

pub fn handle_playbar_events(key: KeyEvent, app: &mut App) -> bool {
    let Some(playback) = &mut app.playback else {
        return false;
    };

    // Pause/resume
    if key.code == KeyCode::Char(' ') {
        let is_playing = playback.is_playing;
        playback.is_playing = !is_playing;
        let _ = app.network_tx.send(request::ClientRequest::Player(request::PlayerRequest::TogglePlayback(is_playing)));
        
        return true;
    }

    // Volume
    if key.code == KeyCode::Char('-') {
        let vol = &mut playback.device.volume;
        *vol = vol.saturating_sub(10);
        let _ = app.network_tx.send(request::ClientRequest::Player(request::PlayerRequest::SetVolume(*vol)));
        
        return true;
    }
    if key.code == KeyCode::Char('+') {
        let vol = &mut playback.device.volume;
        *vol = vol.saturating_add(10).min(100);
        let _ = app.network_tx.send(request::ClientRequest::Player(request::PlayerRequest::SetVolume(*vol)));
        
        return true;
    }
    
    // Next
    if key.code == KeyCode::Char('n') {
        let _ = app.network_tx.send(request::ClientRequest::Player(request::PlayerRequest::NextTrack));
        
        return true;
    }

    // Prev
    if key.code == KeyCode::Char('p') {
        let _ = app.network_tx.send(request::ClientRequest::Player(request::PlayerRequest::PreviousTrack));
        
        return true;
    }

    // Cycle repeat
    if key.code == KeyCode::Char('r') {
        let state = match playback.repeat_state {
            RepeatState::Off => RepeatState::Context,
            RepeatState::Context => RepeatState::Track,
            RepeatState::Track => RepeatState::Off,
        };
        playback.repeat_state = state;
        let _ = app.network_tx.send(request::ClientRequest::Player(
            request::PlayerRequest::SetRepeatMode(state))
        );
        
        return true;
    }

    // Toggle shuffle
    if key.code == KeyCode::Char('s') {
        let shuffling = playback.shuffle_state;
        playback.shuffle_state = !shuffling;
        let _ = app.network_tx.send(request::ClientRequest::Player(
            request::PlayerRequest::ToggleShuffle(shuffling))
        );
        
        return true;
    }

    // Queue
    if key.code == KeyCode::Char('Q') {
        if matches!(app.route, Route::Queue(_)) {
            app.active_block = ActiveBlock::QueueBlock;
            return true;
        }

        app.set_current_route(Route::Queue(QueueState::default()));
        app.active_block = ActiveBlock::QueueBlock;
        
        return true;
    }

    false
}
