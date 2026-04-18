use std::sync::{Arc, Mutex};
use crate::network::models::*;

// Stores unified network data
#[derive(Default)]
pub struct IoSharedState {
    pub playlists: Vec<Playlist>,
    pub playback: Option<Playback>,

    // home-state
    pub recent_tracks: Vec<Track>,
    pub top_tracks: Vec<Track>,
    pub top_artists: Vec<Artist>,

    // playlist-detail-state
    pub playlist_items: Vec<PlayableItem>,
}

// SharedState uses Arc and Mutex to ensure thread-safe,
// exclusive access to IoSharedState
pub type SharedState = Arc<Mutex<IoSharedState>>;
