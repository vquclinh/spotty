use crate::network::models::{
    TimeRange, SearchType
};

#[derive(Clone, Debug)]
pub enum PlayerRequest {
    TogglePlayback(bool),
    NextTrack,
    PreviousTrack,
}

#[derive(Clone, Debug)]
pub enum ClientRequest {
    GetCurrentUser,
    GetCurrentPlayback,
    GetUserPlaylists,
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
    },

    SearchItems {
        query: String,
        search_types: Vec<SearchType>,
        limit: u32,
    },
}
