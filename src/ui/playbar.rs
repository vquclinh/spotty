use crate::app::App;
use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Block, Borders},
};

pub fn draw(f: &mut Frame, app: &App, area: Rect) {
    let block = Block::default().title(" Playbar ").borders(Borders::ALL);
    f.render_widget(block, area);
}
