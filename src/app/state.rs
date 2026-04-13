use std::sync::{Arc, Mutex};
use crate::network::models::{Track, Playlist, PlaybackState};

#[derive(Default)]
pub struct IoSharedState {
    pub playlists: Vec<Playlist>,
    pub playback: Option<PlaybackState>,
    pub recent_tracks: Vec<Track>,
}

pub type SharedState = Arc<Mutex<IoSharedState>>;