#[derive(Default, PartialEq)]
pub enum PlaybarHover {
    #[default]
    None,
    PlayPause,
    Seek,
    Volume,
    Lyrics,
    Queue,
}

#[derive(Default)]
pub struct PlaybarState {
    pub hover: PlaybarHover,
}