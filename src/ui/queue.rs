use crate::app::{ActiveBlock, App};
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
};

pub fn draw(f: &mut Frame, app: &mut App, area: Rect) {
    let border_color = if app.active_block == ActiveBlock::QueueBlock { Color::Green } else { Color::White };

    let queue_text = "\n  [API] Fetching queue from Spotify is coming soon...".to_string();

    let block = Paragraph::new(queue_text).block(
        Block::default()
            .title(" Queue ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(border_color)),
    );

    f.render_widget(block, area);
}
