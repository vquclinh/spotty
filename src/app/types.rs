#[derive(Clone, PartialEq, Debug)]
pub struct Track {
    pub title: String,
    pub artist: String,
    pub album: String,
}

#[derive(Clone, PartialEq, Debug)]
pub enum ActiveBlock {
    LibraryMenu,
    PlaylistsMenu,
    SearchInput,
    SearchResults,
    HomeBlock,
    QueueBlock,
    PlaylistTracks,
    LyricsText,
    LyricsInfo,
}

pub struct PlayerState {
    pub is_playing: bool,
    pub current_track: Option<Track>,
    pub queue: Vec<Track>,
}