use crate::app::{App, ActiveBlock, Route};
use crate::network::models::{PlayableItem, RepeatState};
use ratatui::style::Stylize;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect, Alignment},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, Paragraph},
};

pub fn draw(f: &mut Frame, app: &mut App, area: Rect) {
    let width = f.area().width;
    if width > 120 {
        draw_wide(f, app, area)
    } else {
        draw_narrow(f, app, area)
    }
}

pub fn draw_wide(f: &mut Frame, app: &mut App, area: Rect) {
    let border_color = if app.active_block == ActiveBlock::Playbar { Color::LightCyan } else { Color::White };
    
    let title = if let Route::Search(_) = app.route {
        if app.active_block == ActiveBlock::SearchInput {
            ""
        } else {
            " [3] "
        }
    } else {
        " [4] "
    };
    let outer_block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color));
        
    let inner_area = outer_block.inner(area);
    f.render_widget(outer_block, area);

    let Some(playback) = &app.playback else {
        let empty = Paragraph::new("⏸ | No track currently playing")
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::DarkGray));
        f.render_widget(empty, inner_area);
        return;
    };

    // Try to extract spotify data
    let (is_playing, track_name, artist_name, progress_ms, duration_ms, repeat, shuffle) = {
        let is_playing = playback.is_playing;
        let progress = playback.progress.as_millis() as u32; 
        let repeat = playback.repeat_state;
        let shuffle = playback.shuffle_state;

        if let Some(PlayableItem::Track(track)) = &playback.item {
            let artist_names = track.artists
                .iter()
                .map(|a| a.name.as_str())
                .collect::<Vec<&str>>()
                .join(", ");
            let artist = if artist_names.is_empty() {
                "Unknown".to_string()
            } else {
                artist_names
            };
            (is_playing, track.name.clone(), artist, progress, track.duration.as_millis() as u32, repeat, shuffle)
        } else {
            (is_playing, "Podcast / Episode".to_string(), "Unknown".to_string(), progress, 0, repeat, shuffle)
        }
    };

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(33),
            Constraint::Percentage(34),
            Constraint::Percentage(18),
            Constraint::Percentage(15),
        ])
        .split(inner_area);

    // Song info (left)
    let status = if is_playing { "⏸" } else { "▶" };
    let info_line = Line::from(vec![
        Span::styled(format!(" {} ", status), Style::default().fg(Color::Green)),
        Span::styled(format!("{} ", track_name), Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
        Span::styled(format!("- {}", artist_name), Style::default().fg(Color::DarkGray)),
    ]);
    let info_widget = Paragraph::new(info_line);
    f.render_widget(info_widget, chunks[0]);

    // Progress bar (middle)
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
        .label(Span::styled(label, Style::default().add_modifier(Modifier::BOLD)));
    
    let gauge_area = Rect {
        x: chunks[1].x, y: chunks[1].y,
        width: chunks[1].width, height: chunks[1].height,
    };
    f.render_widget(gauge, gauge_area);

    // Playback state (right)
    let mut spans = Vec::new();

    spans.push(Span::styled("R", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)));
    spans.push(Span::raw("epeat: "));
    match repeat {
        RepeatState::Off => spans.push(Span::raw("Off")),
        RepeatState::Track => spans.push(Span::raw("Track")),
        RepeatState::Context => spans.push(Span::raw("Context")),
    }
    spans.push(Span::raw(" | "));

    spans.push(Span::styled("S", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)));
    spans.push(Span::raw("huffle: "));
    if shuffle {
        spans.push(Span::raw("On"));
    } else {
        spans.push(Span::raw("Off"));
    }

    let playback_state = Line::from(spans).add_modifier(Modifier::ITALIC);
    f.render_widget(Paragraph::new(playback_state).alignment(Alignment::Center), chunks[2]);

    // Keybind hints (right most)
    let hints_line = Line::from(vec![
        Span::raw("Vol "),
        Span::styled("-/+", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::raw(" | "),
        Span::styled("L", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        Span::raw("yrics | "),
        Span::styled("Q", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        Span::raw("ueue "),
    ]);
    let hints_widget = Paragraph::new(hints_line).alignment(Alignment::Right);
    
    f.render_widget(hints_widget, chunks[3]);
}

pub fn draw_narrow(f: &mut Frame, app: &mut App, area: Rect) {
    let border_color = if app.active_block == ActiveBlock::Playbar { Color::LightCyan } else { Color::White };
    
    let outer_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color));
        
    let inner_area = outer_block.inner(area);
    f.render_widget(outer_block, area);

    let Some(playback) = &app.playback else {
        let empty = Paragraph::new("⏸ | No track currently playing")
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::DarkGray));
        f.render_widget(empty, inner_area);
        return;
    };

    // Try to extract spotify data
    let (is_playing, track_name, artist_name, progress_ms, duration_ms, _repeat, _shuffle) = {
        let is_playing = playback.is_playing;
        let progress = playback.progress.as_millis() as u32; 
        let repeat = playback.repeat_state;
        let shuffle = playback.shuffle_state;

        if let Some(PlayableItem::Track(track)) = &playback.item {
            let artist_names = track.artists
                .iter()
                .map(|a| a.name.as_str())
                .collect::<Vec<&str>>()
                .join(", ");
            let artist = if artist_names.is_empty() {
                "Unknown".to_string()
            } else {
                artist_names
            };
            (is_playing, track.name.clone(), artist, progress, track.duration.as_millis() as u32, repeat, shuffle)
        } else {
            (is_playing, "Podcast / Episode".to_string(), "Unknown".to_string(), progress, 0, repeat, shuffle)
        }
    };

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(33),
            Constraint::Percentage(34),
            Constraint::Percentage(33),
        ])
        .split(inner_area);

    // Song info (left)
    let status = if is_playing { "⏸" } else { "▶" };
    let info_line = Line::from(vec![
        Span::styled(format!(" {} ", status), Style::default().fg(Color::Green)),
        Span::styled(format!("{} ", track_name), Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
        Span::styled(format!("- {}", artist_name), Style::default().fg(Color::DarkGray)),
    ]);
    let info_widget = Paragraph::new(info_line);
    f.render_widget(info_widget, chunks[0]);

    // Progress bar (middle)
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
        .label(Span::styled(label, Style::default().add_modifier(Modifier::BOLD)));
    
    let gauge_area = Rect {
        x: chunks[1].x, y: chunks[1].y,
        width: chunks[1].width, height: chunks[1].height,
    };
    f.render_widget(gauge, gauge_area);

    // Keybind hints (right most)
    let hints_line = Line::from(vec![
        Span::raw("Vol "),
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
