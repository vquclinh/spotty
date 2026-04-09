use crate::app::{ActiveBlock, App};
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
};

pub fn draw(f: &mut Frame, app: &App, area: Rect) {
    let border_color = if app.active_block == ActiveBlock::HomeBlock {
        Color::Green
    } else {
        Color::White
    };
    let block = Paragraph::new("Home page.").block(
        Block::default()
            .title(" Home page ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(border_color)),
    );
    f.render_widget(block, area);
}
