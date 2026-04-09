use crate::app::App;
use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Block, Borders},
};

pub fn draw(f: &mut Frame, app: &mut App, area: Rect) {
    let block = Block::default().title(" Playlist ").borders(Borders::ALL);
    f.render_widget(block, area);
}
