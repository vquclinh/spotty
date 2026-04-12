use network::client::{UserProfile, Track, Artist, Album, Playlist};

pub struct UserData {
    profile: Option<UserProfile>,
    top_tracks: Option<Vec<Track>>,
    top_artists: Option<Vec<Artist>>,
    recently_played: Option<Vec<Track>>,
    saved_playlists: Option<Vec<Playlist>>,
    saved_albums: Option<Vec<Album>>,
}
