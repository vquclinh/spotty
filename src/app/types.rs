use ratatui::widgets::TableState;
use ratatui::widgets::ListState;
use crate::network::models::MenuTarget;
use crate::network::models::*;

// -------------------------------- Active Block ----------------------------------
#[derive(Clone, PartialEq, Eq, Debug)]
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
    Playbar,
    AlbumBlock,
    LikedSongs,
    SavedAlbums,
    SavedArtists,
    SavedPodcasts
}

// -------------------------------- Action Menu ------------------------------------
#[derive(Clone, Debug, PartialEq)]
pub enum MenuAction {
    PlayNow,
    AddToQueue,
    AddToPlaylist,
    GoToAlbum,
    GoToArtist,
    GoToShow,
    SaveToLibrary,
    FollowArtist,
    ViewDetails,
}

impl MenuAction {
    pub fn as_str(&self) -> &'static str {
        match self {
            MenuAction::PlayNow => "Play Now",
            MenuAction::AddToQueue => "Add to Queue",
            MenuAction::AddToPlaylist => "Add to Playlist",
            MenuAction::GoToAlbum => "Go to Album",
            MenuAction::GoToArtist => "Go to Artist",
            MenuAction::GoToShow => "Go to Podcast Show",
            MenuAction::SaveToLibrary => "Save to Library",
            MenuAction::FollowArtist => "Follow Artist",
            MenuAction::ViewDetails => "View Details",
        }
    }
}
#[derive(Default)]
pub struct ActionMenu {
    pub is_open: bool,
    pub target: Option<MenuTarget>,
    pub actions: Vec<MenuAction>,
    pub state: ListState,
}

impl ActionMenu {
    pub fn new() -> Self {
        Self {
            is_open: false,
            target: None,
            actions: vec![],
            state: ListState::default(),
        }
    }

    pub fn open(&mut self, target: MenuTarget) {
        let mut dynamic_actions = Vec::new();

        match &target {
            MenuTarget::Track(t) => {
                dynamic_actions.push(MenuAction::PlayNow);
                dynamic_actions.push(MenuAction::AddToQueue);
                dynamic_actions.push(MenuAction::AddToPlaylist);
                if let Some(album) = &t.album &&
                    !album.id.is_empty() &&
                    album.album_type == "album"
                {
                    dynamic_actions.push(MenuAction::GoToAlbum);
                }
                
                if !t.artists.is_empty() {
                    dynamic_actions.push(MenuAction::GoToArtist);
                }
            }
            MenuTarget::Artist(_) => {
                dynamic_actions.push(MenuAction::PlayNow);
                dynamic_actions.push(MenuAction::FollowArtist);
                dynamic_actions.push(MenuAction::ViewDetails);
            }
            MenuTarget::Album(a) => {
                dynamic_actions.push(MenuAction::PlayNow);
                dynamic_actions.push(MenuAction::SaveToLibrary);
                if !a.artists.is_empty() {
                    dynamic_actions.push(MenuAction::GoToArtist);
                }
            }
            MenuTarget::Playlist(_) => {
                dynamic_actions.push(MenuAction::PlayNow);
                dynamic_actions.push(MenuAction::SaveToLibrary);
                dynamic_actions.push(MenuAction::ViewDetails);
            }
            MenuTarget::Episode(e) => {
                dynamic_actions.push(MenuAction::PlayNow);
                dynamic_actions.push(MenuAction::AddToQueue);
                if !e.show_name.is_empty() {
                    dynamic_actions.push(MenuAction::GoToShow);
                }
            }
        }

        self.actions = dynamic_actions;
        self.target = Some(target);
        self.state.select(Some(0));
        self.is_open = true;
    }

    pub fn close(&mut self) {
        self.is_open = false;
        self.target = None;
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => if i >= self.actions.len() - 1 { 0 } else { i + 1 },
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => if i == 0 { self.actions.len() - 1 } else { i - 1 },
            None => 0,
        };
        self.state.select(Some(i));
    }
}

// ----------------------------- Playlist Selector ---------------------------------
#[derive(Default)]
pub struct PlaylistSelector {
    pub is_open: bool,
    pub state: ListState,
    pub playlists: Vec<Playlist>,
}

impl PlaylistSelector {
    pub fn new() -> Self {
        Self {
            is_open: false,
            state: ListState::default(),
            playlists: vec![],
        }
    }

    pub fn close(&mut self) {
        self.is_open = false;
        self.playlists.clear();
        self.state.select(None);
    }

    pub fn next(&mut self) {
        if self.playlists.is_empty() { return; }
        let i = match self.state.selected() {
            Some(i) => if i >= self.playlists.len() - 1 { 0 } else { i + 1 },
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        if self.playlists.is_empty() { return; }
        let i = match self.state.selected() {
            Some(i) => if i == 0 { self.playlists.len() - 1 } else { i - 1 },
            None => 0,
        };
        self.state.select(Some(i));
    }
}

// -------------------------------- Stateful List ----------------------------------
#[derive(Clone, Default)]
pub struct StatefulList<T> {
    pub items: Vec<T>,
    pub state: ListState,
}

impl<T> StatefulList<T> {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            state: ListState::default(),
        }
    }

    pub fn with_items(items: Vec<T>) -> Self {
        Self {
            items,
            state: ListState::default()
        }
    }

    pub fn next(&mut self) {
        if self.items.is_empty() {
            return;
        }
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        if self.items.is_empty() {
            return;
        }
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}

// -------------------------------- Stateful Table ----------------------------------
#[derive(Clone, Default)]
pub struct StatefulTable<T> {
    pub items: Vec<T>,
    pub state: TableState,
}

impl<T> StatefulTable<T> {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            state: TableState::default(),
        }
    }

    pub fn with_items(items: Vec<T>) -> Self {
        Self {
            items,
            state: TableState::default(),
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => if i >= self.items.len() - 1 { 0 } else { i + 1 },
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => if i == 0 { self.items.len() - 1 } else { i - 1 },
            None => 0,
        };
        self.state.select(Some(i));
    }
}
