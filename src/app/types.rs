use ratatui::widgets::TableState;

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

#[derive(Clone)]
pub struct StatefulTable<T> {
    pub items: Vec<T>,
    pub state: TableState,
}

impl<T> StatefulTable<T> {
    pub fn with_items(items: Vec<T>) -> Self {
        let mut state = TableState::default();
        if !items.is_empty() {
            state.select(Some(0));
        }
        Self { items, state }
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