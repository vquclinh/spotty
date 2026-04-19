use crate::app::{ActiveBlock, App};
use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
};

use crate::network::models::PlayableItem;

pub fn draw_text(f: &mut Frame, app: &mut App, area: Rect) {
    let border_color = if app.active_block == ActiveBlock::LyricsText { Color::Green } else { Color:: White };
    let text = "\n\n♫ ... Loading lyrics ... ♫\n\n";

    let block = Paragraph::new(text)
        .block(
            Block::default().title(" LYRICS ").borders(Borders::ALL).border_style(Style::default().fg(border_color)),
        )
        .alignment(Alignment::Center);
    
    f.render_widget(block, area);
}

pub fn draw_info(f: &mut Frame, app: &mut App, area: Rect) {
    let border_color = if app.active_block == ActiveBlock::LyricsInfo { Color::Green } else { Color::White };

    let info = if let Some(playback) = &app.playback {
        
        if let Some(PlayableItem::Track(track)) = &playback.item {
            let artist_name = track.artists.first()
                .map(|a| a.name.clone())
                .unwrap_or_else(|| "Unknown".to_string());

            let album_name = track.album.as_ref()
                .map(|a| a.name.as_str())
                .unwrap_or("None");

            format!("\n Song: {}\n Artist: {}\n Album: {}", track.name, artist_name, album_name)
        } else {
            "\n No track info available.".to_string()
        }

    } else {
        "\n No song currently playing.".to_string()
    };

    let block = Paragraph::new(info).block(
        Block::default().title(" INFORMATION ").borders(Borders::ALL).border_style(Style::default().fg(border_color)),
    );

    f.render_widget(block, area);
}
