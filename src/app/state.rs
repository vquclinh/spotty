use std::sync::{Arc, Mutex};
use crate::network::models::{Track, Playlist, PlaybackState};

// Stores unified network data
#[derive(Default)]
pub struct IoSharedState {
    pub playlists: Vec<Playlist>,
    pub playback: Option<PlaybackState>,
    pub recent_tracks: Vec<Track>,
}

// SharedState uses Arc and Mutex to ensure thread-safe,
// exclusive access to IoSharedState
pub type SharedState = Arc<Mutex<IoSharedState>>;
