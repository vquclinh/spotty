use crate::network::models::{
    TimeRange, SearchType, RepeatState
};
use std::time::Duration;

#[derive(Clone, Debug)]
pub enum PlayerRequest {
    Play(String),
    PlayContext(String, Option<u32>),
    TogglePlayback(bool),
    NextTrack,
    PreviousTrack,
    SetRepeatMode(RepeatState),
    SeekToPosition(u32),
    SetVolume(u8),
    ToggleShuffle(bool),
    AddItemToQueue(String),
}

#[derive(Clone, Debug)]
pub enum ClientRequest {
    GetCurrentUser,
    GetCurrentPlayback,

    GetUserPlaylists {
        limit: u32,
        offset: u32
    },

    GetQueue,

    GetPlaylistItems {
        playlist_id: String,
        limit: u32,
        offset: u32,
    },

    GetUserTopTracks {
        time_range: TimeRange,
        limit: u32,
        offset: u32,
    },

    GetUserTopArtists {
        time_range: TimeRange,
        limit: u32,
        offset: u32,
    },

    GetRecentlyPlayed {
        limit: u32,
        after: Option<Duration>
    },

    SearchItems {
        query: String,
        search_types: Vec<SearchType>,
        limit: u32,
        offset: u32,
    },
    
    // Helper request to deal with Spotify's tight limit on search endpoint
    // (only 10 items at a time)
    SearchItemsUpTo {
        query: String,
        search_types: Vec<SearchType>,
        total_limit: u32,
        start_offset: u32,
    },

    AddItemsToPlaylist {
        playlist_id: String,
        uris: Vec<String>,
    },

    RemoveItemsFromPlaylist {
        playlist_id: String,
        uris: Vec<String>,
    },

    GetAlbum {
        id: String,
    },

    Player(PlayerRequest),

    SaveItemsToLibrary(Vec<String>),

    RemoveItemsFromLibrary(Vec<String>),

    GetUserLikedSongs {
        limit: u32,
        offset: u32
    },

    GetUserSavedAlbums {
        limit: u32,
        offset: u32
    },

    GetUserSavedArtists {
        limit: u32,
        after: Option<String>
    },

    GetUserSavedPodcasts {
        limit: u32,
        offset: u32
    },

    GetLyrics {
        track_id: String,
    },
}
