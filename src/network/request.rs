use crate::network::models::{
    TimeRange, SearchType, RepeatState
};

#[derive(Clone, Debug)]
pub enum PlayerRequest {
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
        offset: u32,
    },

    SearchItems {
        query: String,
        search_types: Vec<SearchType>,
        limit: u32,
        offset: u32,
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
}
