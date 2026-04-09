// Route, it means what is the current state
#[derive(Clone, PartialEq, Debug)] // Create a clone or do a comparison, print debug
pub enum Route {
    Home,
    Search,
    Queue,
    Lyrics,
}

// ActiveBlock, it means what block in screen is currently focus on
#[derive(Clone, PartialEq, Debug)]
pub enum ActiveBlock {
    Sidebar,
    SearchInput,
    SearchResults,
    HomeBlock,
    QueueBlock,
}

pub struct PlayerState {
    pub is_playing: bool,
    pub currennt_track: Option<String>,
    pub queue: Vec<String>,
}

pub struct SearchState {
    pub input: String,
    pub results_artists: Vec<String>,
    pub results_tracks: Vec<String>,
}

// Global data
pub struct App {
    pub route: Route,
    pub active_block: ActiveBlock,

    pub should_quit: bool,
    pub show_help: bool,

    pub player: PlayerState,
    pub search: SearchState,

    pub liked_songs: Vec<String>,
    pub my_albums: Vec<String>,
}

impl App {
    pub fn new() -> Self {
        Self {
            route: Route::Home,
            active_block: ActiveBlock::Sidebar,
            show_help: false,
            should_quit: false,

            player: PlayerState {
                is_playing: false,
                currennt_track: None,
                queue: vec![],
            },
            search: SearchState {
                input: String::new(),
                results_artists: vec![],
                results_tracks: vec![],
            },

            liked_songs: vec![],
            my_albums: vec![],
        }
    }

    // pub fn change_route(&mut self, new_route: Route) {
    //     self.route = new_route.clone();

    //     self.active_block = match new_route {
    //         Route::Home => ActiveBlock::HomeBlock,
    //         Route::Search => ActiveBlock::SearchInput,
    //         Route::Playlist => ActiveBlock::TrackTable,
    //         Route::Lyrics => ActiveBlock::HomeBlock,
    //     }
    // }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
