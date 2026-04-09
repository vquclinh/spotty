use crate::app::{ActiveBlock, App};
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
};

pub fn draw(f: &mut Frame, app: &mut App, area: Rect) {
    let border_color = if app.active_block == ActiveBlock::Sidebar {
        Color::Green
    } else {
        Color::White
    };

    let text = format!(
        " Liked Songs: {}, {} ",
        app.liked_songs.len(),
        app.my_albums.len()
    );
    let block = Paragraph::new(text).block(
        Block::default()
            .title(" Library ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(border_color)),
    );
    f.render_widget(block, area);
}
