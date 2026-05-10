use librespot_metadata::Lyrics;

#[derive(Clone, Default, Debug)]
pub struct LyricsState {
    pub data: Option<Lyrics>,
    
    pub is_loading: bool,     
    
    pub scroll_offset: u16,

    /// animation tick counter
    pub tick: u64,
}

impl LyricsState {
    pub fn new() -> Self {
        Self {
            data: None,
            is_loading: true,
            scroll_offset: 0,
            tick: 0,
        }
    }
}