use std::sync::{Arc, Mutex};
use crate::network::models::{Track, Playlist, PlaybackState};

// the place where network push data to and UI get data from,
// Arc and Mutex help us to maintain only one of these operation once time
#[derive(Default)]
pub struct IoSharedState {
    pub playlists: Vec<Playlist>,
    pub playback: Option<PlaybackState>,
    pub recent_tracks: Vec<Track>,
}

pub type SharedState = Arc<Mutex<IoSharedState>>;