use crate::app::{App, ActiveBlock};
use crate::network::models::PlayableItem;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect, Alignment},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, Paragraph},
};

pub fn draw(f: &mut Frame, app: &mut App, area: Rect) {
    let border_color = if app.active_block == ActiveBlock::Playbar { Color::LightCyan } else { Color::White };

    // data from spotify
    let (is_playing, track_name, artist_name, progress_ms, duration_ms) = if let Some(playback) = &app.playback {
        let is_playing = playback.is_playing;
        
        let progress = playback.progress.as_millis() as u32; 

        if let Some(PlayableItem::Track(track)) = &playback.item {
            let artist = track.artists.first()
                .map(|a| a.name.clone())
                .unwrap_or_else(|| "Unknown".to_string());
            (is_playing, track.name.clone(), artist, progress, track.duration.as_millis() as u32)
        } else {
            (is_playing, "Podcast / Episode".to_string(), "Unknown".to_string(), progress, 0)
        }
    } else {
        (false, "No song".to_string(), "".to_string(), 0, 0)
    };
    
    let outer_block = Block::default()
        .title(" Now Playing ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color));
        
    let inner_area = outer_block.inner(area);
    f.render_widget(outer_block, area);

    

    // no song
    if duration_ms == 0 && track_name == "No song" {
        let empty = Paragraph::new("⏸ | No track currently playing")
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::DarkGray));
        f.render_widget(empty, inner_area);
        return;
    }

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(35),
            Constraint::Percentage(40),
            Constraint::Percentage(25),
        ])
        .split(inner_area);

    // song info
    let status = if is_playing { "▶" } else { "⏸" };
    let info_line = Line::from(vec![
        Span::styled(format!(" {} ", status), Style::default().fg(Color::Green)),
        Span::styled(format!("{} ", track_name), Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
        Span::styled(format!("- {}", artist_name), Style::default().fg(Color::DarkGray)),
    ]);
    let info_widget = Paragraph::new(info_line);
    f.render_widget(info_widget, chunks[0]);

    // playing bar
    let percent = if duration_ms > 0 {
        (progress_ms as f64 / duration_ms as f64).clamp(0.0, 1.0)
    } else {
        0.0
    };
    
    let progress_sec = progress_ms / 1000;
    let duration_sec = duration_ms / 1000;
    let label = format!("{}:{:02} / {}:{:02}", progress_sec / 60, progress_sec % 60, duration_sec / 60, duration_sec % 60);

    let gauge = Gauge::default()
        .gauge_style(Style::default().fg(Color::LightGreen).bg(Color::DarkGray))
        .ratio(percent)
        .label(Span::styled(label, Style::default().fg(Color::White).add_modifier(Modifier::BOLD)));
    
    let gauge_area = Rect {
        x: chunks[1].x + 1, y: chunks[1].y,
        width: chunks[1].width.saturating_sub(2), height: chunks[1].height,
    };
    f.render_widget(gauge, gauge_area);

    // keybind hints
    let hints_line = Line::from(vec![
        Span::raw(" Vol "),
        Span::styled("-/+", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::raw(" | "),
        Span::styled("L", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        Span::raw("yrics | "),
        Span::styled("Q", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        Span::raw("ueue "),
    ]);
    let hints_widget = Paragraph::new(hints_line).alignment(Alignment::Right);
    
    f.render_widget(hints_widget, chunks[2]);
}
