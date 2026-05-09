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
}

impl PlaybarState {
    pub fn new() -> Self {
        Self::default()
    }
}