use crate::app::{ActiveBlock, App};
use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
};

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

    let info = if let Some(track) = &app.player.current_track {
        format!("\n Song: {}\n Artist: {}\n Album: {}", track.title, track.artist, track.album)
    } else {
        "\n No song.".to_string()
    };

    let block = Paragraph::new(info).block(
        Block::default().title(" INFORMATION ").borders(Borders::ALL).border_style(Style::default().fg(border_color)),
    );

    f.render_widget(block, area);
}
