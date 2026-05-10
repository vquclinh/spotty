use std::sync::{Arc, Mutex};
use crate::network::models::*;

use librespot_metadata::Lyrics;

#[derive(Default)]
pub struct DataPayload<T> {
    pub items: Vec<T>,
    pub is_end: bool,
    pub should_append: bool
}

impl<T> DataPayload<T> {
    pub fn clear(&mut self) {
        self.items.clear();
        self.is_end = false;
        self.should_append = true;
    }
}

impl<T> From<Page<T>> for DataPayload<T> {
    fn from(page: Page<T>) -> Self {
        Self {
            items: page.items,
            is_end: page.next.is_none() && page.after.is_none(),
            should_append: page.offset.unwrap_or(0) != 0
        }
    }
}

// Stores unified network data
#[derive(Default)]
pub struct IoSharedState {
    pub user: User,
    
    pub playlists: DataPayload<Playlist>,
    pub playback: Option<Playback>,

    // home-state
    pub recent_tracks: DataPayload<Track>,
    pub top_tracks: DataPayload<Track>,
    pub top_artists: DataPayload<Artist>,

    // playlist-detail-state
    pub playlist_items: DataPayload<PlayableItem>,

    // search-results
    pub search_results: SearchResult,

    // queue-state
    pub queue_data: Option<(Option<PlayableItem>, Vec<PlayableItem>)>,

    // album-state
    pub album_detail: Option<Album>,

    // playback state
    pub playback_state: Option<Playback>,

    // library state
    pub liked_songs: DataPayload<Track>,
    pub saved_albums: DataPayload<Album>,
    pub saved_artists: DataPayload<Artist>,
    pub saved_podcasts: DataPayload<Episode>,

    // lyrics data 
    pub lyrics_data: Option<Lyrics>,
}

// SharedState uses Arc and Mutex to ensure thread-safe,
// exclusive access to IoSharedState
pub type SharedState = Arc<Mutex<IoSharedState>>;
