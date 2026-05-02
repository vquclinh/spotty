use crate::network::models::*;
use crate::app::types::StatefulTable;
use ratatui::layout::Rect;

#[derive(Clone)]
pub enum LibraryMenuItem {
    LikedSongs(LikedSongsState),
    SavedAlbums(SavedAlbumsState),
    SavedArtists(SavedArtistsState),
    SavedPodcasts(SavedPodcastsState)
}

#[derive(Clone)]
pub struct LikedSongsState {
    pub tracks: StatefulTable<Track>,

    pub is_loading: bool,
    pub is_end: bool,
    pub last_area: Rect,
}

impl LikedSongsState {
    pub fn new(tracks: Vec<Track>) -> Self {
        Self {
            tracks: StatefulTable::with_items(tracks),

            is_loading: false,
            is_end: false,
            last_area: Rect::default(),
        }
    }
}

#[derive(Clone)]
pub struct SavedAlbumsState {
    pub albums: StatefulTable<Album>,

    pub is_loading: bool,
    pub is_end: bool,
    pub last_area: Rect,
}

impl SavedAlbumsState {
    pub fn new(albums: Vec<Album>) -> Self {
        Self {
            albums: StatefulTable::with_items(albums),

            is_loading: false,
            is_end: false,
            last_area: Rect::default(),
        }
    }
}

#[derive(Clone)]
pub struct SavedArtistsState {
    pub artists: StatefulTable<Artist>,

    pub is_loading: bool,
    pub is_end: bool,
    pub last_area: Rect
}

impl SavedArtistsState {
    pub fn new(artists: Vec<Artist>) -> Self {
        Self {
            artists: StatefulTable::with_items(artists),

            is_loading: false,
            is_end: false,
            last_area: Rect::default(),
        }
    }
}

#[derive(Clone)]
pub struct SavedPodcastsState {
    pub podcasts: StatefulTable<Episode>,

    pub is_loading: bool,
    pub is_end: bool,
    pub last_area: Rect
}

impl SavedPodcastsState {
    pub fn new(podcasts: Vec<Episode>) -> Self {
        Self {
            podcasts: StatefulTable::with_items(podcasts),

            is_loading: false,
            is_end: false,
            last_area: Rect::default(),
        }
    }
}

