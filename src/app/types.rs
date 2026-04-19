use ratatui::widgets::TableState;
use ratatui::widgets::ListState;

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
    Playbar,
}

#[derive(Clone, Default)]
pub struct StatefulList {
    pub state: ListState,
}

impl StatefulList {
    pub fn new() -> Self {
        Self {
            state: ListState::default(),
        }
    }

    pub fn next(&mut self, len: usize) {
        if len == 0 { return; }
        let i = match self.state.selected() {
            Some(i) => if i >= len - 1 { 0 } else { i + 1 },
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self, len: usize) {
        if len == 0 { return; }
        let i = match self.state.selected() {
            Some(i) => if i == 0 { len - 1 } else { i - 1 },
            None => 0,
        };
        self.state.select(Some(i));
    }
}

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