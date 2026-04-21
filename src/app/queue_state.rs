use crate::app::types::*;
use crate::network::models::*;
use ratatui::layout::Rect;

#[derive(Clone)]
pub struct QueueState {
    pub currently_playing: Option<PlayableItem>,
    pub queue_items: StatefulTable<PlayableItem>,
    pub last_area: Rect,
}

impl Default for QueueState {
    fn default() -> Self {
        Self {
            currently_playing: None,
            queue_items: StatefulTable::new(),
            last_area: Rect::default(),
        }
    }
}

