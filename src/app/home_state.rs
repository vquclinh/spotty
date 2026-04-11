use ratatui::widgets::TableState;

#[derive(Clone)]
pub struct SimplifiedPlaylist {
    pub name: String,
    pub description: String,
}

#[derive(Clone)]
pub struct  SimplifiedAlbum {
    pub name: String,
    pub artist: String,
}

#[derive(Clone)]
pub struct SimplifiedTrack {
    pub title: String,
    pub artist: String,
    pub extra_info: String,
}

#[derive(Clone)]
pub struct SimplifiedArtist {
    pub name: String,
    pub genres: String,
}

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

    pub top_tracks: Vec<SimplifiedTrack>,
    pub top_tracks_state: TableState,

    pub top_artists: Vec<SimplifiedArtist>,
    pub top_artists_state: TableState,

    pub recent_tracks: Vec<SimplifiedTrack>,
    pub recent_state: TableState,
}

impl HomeState {
    pub fn new() -> Self {
        let mut default_state = TableState::default();
        default_state.select(Some(0));

        Self {
            greeting: "Good Morning".to_string(),
            active_tab: HomeTab::TopTracks,

            top_tracks: vec![
                SimplifiedTrack { title: "Nau An Cho Em".to_string(), artist: "Den".to_string(), extra_info: "4:20".to_string() },
                SimplifiedTrack { title: "Making My Way".to_string(), artist: "Son Tung M-TP".to_string(), extra_info: "3:15".to_string() },
                SimplifiedTrack { title: "Die For You".to_string(), artist: "The Weeknd".to_string(), extra_info: "4:20".to_string() },
                SimplifiedTrack { title: "Loi Choi".to_string(), artist: "Wren Evans".to_string(), extra_info: "3:05".to_string() },
                SimplifiedTrack { title: "Ai".to_string(), artist: "tlinh".to_string(), extra_info: "3:12".to_string() },
                SimplifiedTrack { title: "Vu Tru Co Bay".to_string(), artist: "Phuong My Chi".to_string(), extra_info: "4:05".to_string() },
                SimplifiedTrack { title: "Cruel Summer".to_string(), artist: "Taylor Swift".to_string(), extra_info: "2:58".to_string() },
                SimplifiedTrack { title: "Du Bao Thoi Tiet Hom Nay Mua".to_string(), artist: "GREY D".to_string(), extra_info: "3:45".to_string() },
                SimplifiedTrack { title: "Lan Cuoi".to_string(), artist: "Ngot".to_string(), extra_info: "3:33".to_string() },
                SimplifiedTrack { title: "Chim Sau".to_string(), artist: "RPT MCK".to_string(), extra_info: "2:45".to_string() },
                SimplifiedTrack { title: "Waiting For You".to_string(), artist: "MONO".to_string(), extra_info: "4:25".to_string() },
                SimplifiedTrack { title: "Seven".to_string(), artist: "Jung Kook".to_string(), extra_info: "3:04".to_string() },
                SimplifiedTrack { title: "As It Was".to_string(), artist: "Harry Styles".to_string(), extra_info: "2:47".to_string() },
                SimplifiedTrack { title: "Ditto".to_string(), artist: "NewJeans".to_string(), extra_info: "3:05".to_string() },
                SimplifiedTrack { title: "Thoi Mien".to_string(), artist: "Phao".to_string(), extra_info: "2:55".to_string() },
                SimplifiedTrack { title: "Thang Dien".to_string(), artist: "JustaTee, Phuong Ly".to_string(), extra_info: "4:10".to_string() },
            ],
            top_tracks_state: default_state.clone(),

            top_artists: vec![
                SimplifiedArtist { name: "Son Tung M-TP".to_string(), genres: "V-pop, Hip-hop".to_string() },
                SimplifiedArtist { name: "The Weeknd".to_string(), genres: "R&B, Pop".to_string() },
                SimplifiedArtist { name: "Ngot".to_string(), genres: "Indie Pop".to_string() },
                SimplifiedArtist { name: "Taylor Swift".to_string(), genres: "Pop".to_string() },
                SimplifiedArtist { name: "Wren Evans".to_string(), genres: "V-pop, R&B".to_string() },
                SimplifiedArtist { name: "tlinh".to_string(), genres: "Hip-hop, R&B".to_string() },
                SimplifiedArtist { name: "Den".to_string(), genres: "Rap, Hip-hop".to_string() },
                SimplifiedArtist { name: "Phuong My Chi".to_string(), genres: "Folk, Pop".to_string() },
                SimplifiedArtist { name: "NewJeans".to_string(), genres: "K-pop".to_string() },
                SimplifiedArtist { name: "GREY D".to_string(), genres: "Pop, R&B".to_string() },
                SimplifiedArtist { name: "HIEUTHUHAI".to_string(), genres: "Rap".to_string() },
                SimplifiedArtist { name: "RPT MCK".to_string(), genres: "Rap".to_string() },
                SimplifiedArtist { name: "Chillies".to_string(), genres: "Indie Pop".to_string() },
                SimplifiedArtist { name: "Da LAB".to_string(), genres: "Pop, Rap".to_string() },
                SimplifiedArtist { name: "Post Malone".to_string(), genres: "Hip-hop, Pop".to_string() },
                SimplifiedArtist { name: "MONO".to_string(), genres: "Pop, R&B".to_string() },
            ],
            top_artists_state: default_state.clone(),

            recent_tracks: vec![
                SimplifiedTrack { title: "Loi Choi".to_string(), artist: "Wren Evans".to_string(), extra_info: "15 mins ago".to_string() },
                SimplifiedTrack { title: "Ai".to_string(), artist: "tlinh".to_string(), extra_info: "2 hours ago".to_string() },
                SimplifiedTrack { title: "Seven".to_string(), artist: "Jung Kook".to_string(), extra_info: "3 hours ago".to_string() },
                SimplifiedTrack { title: "Tinh Kha".to_string(), artist: "Ngot".to_string(), extra_info: "5 hours ago".to_string() },
                SimplifiedTrack { title: "Danh Doi".to_string(), artist: "Obito".to_string(), extra_info: "10 hours ago".to_string() },
                SimplifiedTrack { title: "Buong Doi Tay Nhau Ra".to_string(), artist: "Son Tung M-TP".to_string(), extra_info: "1 day ago".to_string() },
                SimplifiedTrack { title: "Blinding Lights".to_string(), artist: "The Weeknd".to_string(), extra_info: "1 day ago".to_string() },
                SimplifiedTrack { title: "Anh Da On Hon".to_string(), artist: "RPT MCK".to_string(), extra_info: "1 day ago".to_string() },
                SimplifiedTrack { title: "LUNCH".to_string(), artist: "Billie Eilish".to_string(), extra_info: "2 days ago".to_string() },
                SimplifiedTrack { title: "Ngu Toi".to_string(), artist: "Ngot".to_string(), extra_info: "3 days ago".to_string() },
                SimplifiedTrack { title: "Ex's Hate Me".to_string(), artist: "B Ray".to_string(), extra_info: "4 days ago".to_string() },
                SimplifiedTrack { title: "Chay Ngay Di".to_string(), artist: "Son Tung M-TP".to_string(), extra_info: "5 days ago".to_string() },
                SimplifiedTrack { title: "See Tinh".to_string(), artist: "Hoang Thuy Linh".to_string(), extra_info: "1 week ago".to_string() },
                SimplifiedTrack { title: "Muon Roi Ma Sao Con".to_string(), artist: "Son Tung M-TP".to_string(), extra_info: "1 week ago".to_string() },
                SimplifiedTrack { title: "Starboy".to_string(), artist: "The Weeknd".to_string(), extra_info: "2 weeks ago".to_string() },
            ],
            recent_state: default_state,
        }
    }
}

impl Default for HomeState {
    fn default() -> Self {
        Self::new()
    }
}