use std::sync::{Arc, Mutex};
use crate::network::models::{Artist, PlaybackState, Playlist, Track};

// Stores unified network data
#[derive(Default)]
pub struct IoSharedState {
    pub playlists: Vec<Playlist>,
    pub playback: Option<PlaybackState>,

    pub recent_tracks: Vec<Track>,
    pub top_tracks: Vec<Track>,
    pub top_artists: Vec<Artist>,
}

// SharedState uses Arc and Mutex to ensure thread-safe,
// exclusive access to IoSharedState
pub type SharedState = Arc<Mutex<IoSharedState>>;
