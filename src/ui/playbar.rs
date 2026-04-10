use crate::app::App;
use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
};

pub fn draw(f: &mut Frame, app: &mut App, area: Rect) {
    let status = if app.player.is_playing { "▶" } else { "||" };
    
    let track_info = if let Some(track) = &app.player.current_track {
        format!(" {} | {} - {} ", status, track.title, track.artist)
    } else {
        format!(" {} | No song ", status)
    };

    let block = Paragraph::new(track_info)
        .block(
            Block::default()
                .title(" Now Playing ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan)),
        )
        .alignment(Alignment::Center);

    f.render_widget(block, area);
}