use crate::app::{ActiveBlock, App};
use crate::app::route::Route;
use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line as TuiLine, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
};

use crate::network::models::PlayableItem;

pub fn draw_text(f: &mut Frame, app: &mut App, area: Rect) {
    let border_color = if app.active_block == ActiveBlock::LyricsText { 
        Color::Green 
    } else { 
        Color::White 
    };

    let block = Block::default()
        .title(" [3] LYRICS ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color));

    let progress_ms = match &app.playback {
        Some(playback) => playback.progress.as_millis() as u32,
        None => {
            let p = Paragraph::new("\n\n♫ ... No song is currently playing ... ♫\n\n")
                .block(block)
                .alignment(Alignment::Center);
            f.render_widget(p, area);
            return;
        }
    };

    let Route::Lyrics(state) = &app.route else { return };

    if state.is_loading {
        let p = Paragraph::new("\n\n♫ ... Loading lyrics ... ♫\n\n")
            .block(block)
            .alignment(Alignment::Center);
        f.render_widget(p, area);
        return;
    }

    match &state.data {
        Some(lyrics_data) => {
            let mut active_idx: usize = 0;

            for (i, line) in lyrics_data.lyrics.lines.iter().enumerate() {
                let time_ms: u32 = line.start_time_ms.parse().unwrap_or(0);

                if time_ms <= progress_ms {
                    active_idx = i;
                } else {
                    break;
                }
            }

            let mut spans = Vec::new();
            for (i, line) in lyrics_data.lyrics.lines.iter().enumerate() {
                let text = &line.words;

                if i == active_idx {
                    spans.push(TuiLine::from(Span::styled(
                        text.clone(),
                        Style::default().fg(Color::LightGreen).add_modifier(Modifier::BOLD),
                    )));
                } else {
                    spans.push(TuiLine::from(Span::styled(
                        text.clone(),
                        Style::default().fg(Color::DarkGray),
                    )));
                }
            }

            let half_screen = (area.height / 2).saturating_sub(1) as usize;
            let scroll_y = active_idx.saturating_sub(half_screen) as u16;

            let paragraph = Paragraph::new(spans)
                .block(block)
                .alignment(Alignment::Center)
                .wrap(Wrap { trim: true })
                .scroll((scroll_y, 0));

            f.render_widget(paragraph, area);
        }
        None => {
            let p = Paragraph::new("\n\n♫ ... No synced lyrics found ... ♫\n\n")
                .block(block)
                .alignment(Alignment::Center);
            f.render_widget(p, area);
        }
    }
}

pub fn draw_info(f: &mut Frame, app: &mut App, area: Rect) {
    let border_color = if app.active_block == ActiveBlock::LyricsInfo { 
        Color::Green 
    } else { 
        Color::White 
    };

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
        Block::default()
            .title(" INFORMATION ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(border_color)),
    );

    f.render_widget(block, area);
}