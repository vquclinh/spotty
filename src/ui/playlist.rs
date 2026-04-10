use crate::app::{ActiveBlock, App};
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
};

pub fn draw(f: &mut Frame, app: &mut App, area: Rect) {
    let border_color = if app.active_block == ActiveBlock::PlaylistTracks { Color::Green } else { Color::White };
    let block = Paragraph::new("List of Songs...").block(
        Block::default().title(" Playlist ").borders(Borders::ALL).border_style(Style::default().fg(border_color)),
    );
    f.render_widget(block, area);
}
