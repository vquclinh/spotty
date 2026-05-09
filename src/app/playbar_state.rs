#[derive(PartialEq, Clone, Copy, Default)]
pub enum PlaybarItem {
    #[default]
    Volume,
    Lyrics,
    Queue,
    Shuffle,
    Repeat,
}

impl PlaybarItem {
    pub fn next(self) -> Self {
        match self {
            PlaybarItem::Volume => PlaybarItem::Lyrics,
            PlaybarItem::Lyrics => PlaybarItem::Queue,
            PlaybarItem::Queue => PlaybarItem::Shuffle,
            PlaybarItem::Shuffle => PlaybarItem::Repeat,
            PlaybarItem::Repeat => PlaybarItem::Volume,
        }
    }

    pub fn prev(self) -> Self {
        match self {
            PlaybarItem::Volume => PlaybarItem::Repeat,
            PlaybarItem::Lyrics => PlaybarItem::Volume,
            PlaybarItem::Queue => PlaybarItem::Lyrics,
            PlaybarItem::Shuffle => PlaybarItem::Queue,
            PlaybarItem::Repeat => PlaybarItem::Shuffle,
        }
    }
}

#[derive(Clone, Default)]
pub struct PlaybarState {
    pub hovered_item: PlaybarItem,

    pub scroll_offset: usize,
    pub scroll_forward: bool,
    pub scroll_wait: usize,
    pub tick_count: usize,
}

impl PlaybarState {
    pub fn new() -> Self {
        Self {
            hovered_item: PlaybarItem::default(),
            scroll_offset: 0,
            scroll_forward: true,
            scroll_wait: 10,
            tick_count: 0,
        }
    }
}