use crate::app::types::{SimplifiedArtist, SimplifiedTrack, StatefulTable};

#[derive(Clone, PartialEq)]
pub enum HomeTab {
    TopTracks,
    TopArtists,
    RecentlyPlayed,
}

#[derive(Clone)]
pub struct HomeState {
    pub greeting: String,
    pub active_tab: HomeTab,

    pub top_tracks: StatefulTable<SimplifiedTrack>,
    pub top_artists: StatefulTable<SimplifiedArtist>,
    pub recent_tracks: StatefulTable<SimplifiedTrack>,
}

impl HomeState {
    pub fn new() -> Self {
        Self {
            greeting: "Good Morning".to_string(),
            active_tab: HomeTab::TopTracks,
            top_tracks: StatefulTable::with_items(mock_top_tracks()),
            top_artists: StatefulTable::with_items(mock_top_artists()),
            recent_tracks: StatefulTable::with_items(mock_recent_tracks()),
        }
    }
}

impl Default for HomeState {
    fn default() -> Self {
        Self::new()
    }
}

fn mock_top_tracks() -> Vec<SimplifiedTrack> {
    vec![
        SimplifiedTrack { title: "Nau An Cho Em".to_string(), artist: "Den".to_string(), extra_info: "4:20".to_string() },
        SimplifiedTrack { title: "Making My Way".to_string(), artist: "Son Tung M-TP".to_string(), extra_info: "3:15".to_string() },
        SimplifiedTrack { title: "Die For You".to_string(), artist: "The Weeknd".to_string(), extra_info: "4:20".to_string() },
        SimplifiedTrack { title: "Loi Choi".to_string(), artist: "Wren Evans".to_string(), extra_info: "3:05".to_string() },
        SimplifiedTrack { title: "Ai".to_string(), artist: "tlinh".to_string(), extra_info: "3:12".to_string() },
    ]
}

fn mock_top_artists() -> Vec<SimplifiedArtist> {
    vec![
        SimplifiedArtist { name: "Son Tung M-TP".to_string(), genres: "V-pop, Hip-hop".to_string() },
        SimplifiedArtist { name: "The Weeknd".to_string(), genres: "R&B, Pop".to_string() },
        SimplifiedArtist { name: "Ngot".to_string(), genres: "Indie Pop".to_string() },
        SimplifiedArtist { name: "Taylor Swift".to_string(), genres: "Pop".to_string() },
    ]
}

fn mock_recent_tracks() -> Vec<SimplifiedTrack> {
    vec![
        SimplifiedTrack { title: "Loi Choi".to_string(), artist: "Wren Evans".to_string(), extra_info: "15 mins ago".to_string() },
        SimplifiedTrack { title: "Ai".to_string(), artist: "tlinh".to_string(), extra_info: "2 hours ago".to_string() },
        SimplifiedTrack { title: "Seven".to_string(), artist: "Jung Kook".to_string(), extra_info: "3 hours ago".to_string() },
    ]
}