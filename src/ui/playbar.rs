use crate::app::App;
use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
};

pub fn draw(f: &mut Frame, app: &mut App, area: Rect) {
    let is_playing = app.playback.as_ref().map_or(false, |p| p.is_playing);
    let status = if is_playing { "▶" } else { "||" };
    
    let track_info = if let Some(playback) = &app.playback {
        if let Some(crate::network::models::Playable::Track(track)) = &playback.item {
            let artist_name = track.artists.first()
                .map(|a| a.name.clone())
                .unwrap_or_else(|| "Unknown".to_string());
                
            format!(" {} | {} - {} ", status, track.name, artist_name)
        } else {
            format!(" {} | Playing a Podcast ", status)
        }
    } else {
        format!(" {} | No song ", status)
    };

    let block = Paragraph::new(track_info)
        .block(
            Block::default()
                .title(" Now Playing ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::White)),
        )
        .alignment(Alignment::Center);

    f.render_widget(block, area);
}