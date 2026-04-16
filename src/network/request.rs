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
    GetCurrentUserQueue,

    GetPlaylistTracks {
        playlist_id: String,
        limit: Option<u32>,
        offset: Option<u32>,
    },

    GetRecentlyPlayed {
        limit: u32,
    },

    GetTopTracks {
        limit: u32,
    },
    
    Player(PlayerRequest),
}