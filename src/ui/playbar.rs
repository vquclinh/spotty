use crate::app::{App, ActiveBlock};
use crate::app::playbar_state::PlaybarItem;
use crate::network::models::{PlayableItem, RepeatState};
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
    
    let outer_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color));
        
    let inner_area = outer_block.inner(area);
    f.render_widget(outer_block, area);

    let Some(playback) = &app.playback else {
        let empty = Paragraph::new("󰏤 | No track currently playing")
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::DarkGray));
        f.render_widget(empty, inner_area);
        return;
    };

    // Try to extract spotify data
    let (is_playing, track_name, artist_name, progress_ms, duration_ms, repeat, shuffle, volume) = {
        let is_playing = playback.is_playing;
        let progress = playback.progress.as_millis() as u32; 
        let repeat = playback.repeat_state;
        let shuffle = playback.shuffle_state;
        let volume = playback.device.volume;

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
            (is_playing, track.name.clone(), artist, progress, track.duration.as_millis() as u32, repeat, shuffle, volume)
        } else {
            (is_playing, "Podcast / Episode".to_string(), "Unknown".to_string(), progress, 0, repeat, shuffle, volume)
        }
    };

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Percentage(45),
            Constraint::Percentage(25),
        ])
        .split(inner_area);

    let status = if is_playing { "󰐊" } else { "󰏤" }; 
    
    let info_line = Line::from(vec![
        Span::styled(format!(" {} ", status), Style::default().fg(Color::Green)),
        Span::styled(format!("{} ", track_name), Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
        Span::styled(format!("- {}", artist_name), Style::default().fg(Color::DarkGray)),
    ]);
    let info_widget = Paragraph::new(info_line);
    f.render_widget(info_widget, chunks[0]);

    // Progress bar
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

    let mut spans = Vec::new();
    let is_playbar_active = app.active_block == ActiveBlock::Playbar;
    let hovered_item = app.playbar.hovered_item;

    let (first_char, rest_text) = if is_playbar_active {
        match hovered_item {
            PlaybarItem::Volume => ("V", format!("olume ({}%)", volume)),
            PlaybarItem::Lyrics => ("L", "yrics".to_string()),
            PlaybarItem::Queue => ("Q", "ueue".to_string()),
            PlaybarItem::Shuffle => ("S", format!("huffle ({})", if shuffle { "On" } else { "Off" })),
            PlaybarItem::Repeat => {
                let state_str = match repeat {
                    RepeatState::Off => "Off",
                    RepeatState::Context => "Context",
                    RepeatState::Track => "Track",
                };
                ("R", format!("epeat ({})", state_str))
            }
        }
    } else {
        ("V", format!("olume ({}%)", volume))
    };

    spans.push(Span::styled(first_char, Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)));
    spans.push(Span::styled(format!("{} | ", rest_text), Style::default().fg(Color::White)));

    let create_icon = |item: PlaybarItem, icon: &str, is_on: bool, default_color: Color| -> Span<'static> {
        let mut style = Style::default();
        
        if is_playbar_active && hovered_item == item {
            if is_on {
                style = style.fg(Color::LightGreen).add_modifier(Modifier::BOLD);
            } else {
                style = style.fg(Color::Cyan).add_modifier(Modifier::BOLD);
            }
        } else {
            style = style.fg(if is_on { Color::Green } else { default_color });
        }
        
        Span::styled(format!(" {} ", icon), style)
    };

    let vol_icon = if volume == 0 { "󰝟" } else { "󰕾" };
    spans.push(create_icon(PlaybarItem::Volume, vol_icon, false, Color::White));
    spans.push(create_icon(PlaybarItem::Lyrics, "󰎆", false, Color::White));
    spans.push(create_icon(PlaybarItem::Queue, "󰲹", false, Color::White));
    spans.push(create_icon(PlaybarItem::Shuffle, "󰒟", shuffle, Color::DarkGray));

    let (repeat_icon, repeat_on) = match repeat {
        RepeatState::Off => ("󰑖", false),
        RepeatState::Context => ("󰑖", true),
        RepeatState::Track => ("󰑘", true),
    };
    spans.push(create_icon(PlaybarItem::Repeat, repeat_icon, repeat_on, Color::DarkGray));

    let controls_line = Line::from(spans);
    f.render_widget(Paragraph::new(controls_line).alignment(Alignment::Right), chunks[2]);
}

pub fn draw_narrow(f: &mut Frame, app: &mut App, area: Rect) {
    let border_color = if app.active_block == ActiveBlock::Playbar { Color::LightCyan } else { Color::White };
    
    let outer_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color));
        
    let inner_area = outer_block.inner(area);
    f.render_widget(outer_block, area);

    let Some(playback) = &app.playback else {
        let empty = Paragraph::new("󰏤 | No track currently playing")
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::DarkGray));
        f.render_widget(empty, inner_area);
        return;
    };

    let (is_playing, track_name, artist_name, progress_ms, duration_ms, repeat, shuffle, volume) = {
        let is_playing = playback.is_playing;
        let progress = playback.progress.as_millis() as u32; 
        let repeat = playback.repeat_state;
        let shuffle = playback.shuffle_state;
        let volume = playback.device.volume;

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
            (is_playing, track.name.clone(), artist, progress, track.duration.as_millis() as u32, repeat, shuffle, volume)
        } else {
            (is_playing, "Podcast / Episode".to_string(), "Unknown".to_string(), progress, 0, repeat, shuffle, volume)
        }
    };

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Percentage(40),
            Constraint::Percentage(30),
        ])
        .split(inner_area);

    let status = if is_playing { "󰐊" } else { "󰏤" };
    let info_line = Line::from(vec![
        Span::styled(format!(" {} ", status), Style::default().fg(Color::Green)),
        Span::styled(format!("{} ", track_name), Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
        Span::styled(format!("- {}", artist_name), Style::default().fg(Color::DarkGray)),
    ]);
    let info_widget = Paragraph::new(info_line);
    f.render_widget(info_widget, chunks[0]);

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

    let mut spans = Vec::new();
    let is_playbar_active = app.active_block == ActiveBlock::Playbar;
    let hovered_item = app.playbar.hovered_item;

    let (first_char, rest_text) = if is_playbar_active {
        match hovered_item {
            PlaybarItem::Volume => ("V", format!("ol ({}%)", volume)),
            PlaybarItem::Lyrics => ("L", "yr".to_string()),
            PlaybarItem::Queue => ("Q", "ue".to_string()),
            PlaybarItem::Shuffle => ("S", format!("hu ({})", if shuffle { "On" } else { "Off" })),
            PlaybarItem::Repeat => {
                let state_str = match repeat {
                    RepeatState::Off => "Off",
                    RepeatState::Context => "Ctx",
                    RepeatState::Track => "Trk",
                };
                ("R", format!("ep ({})", state_str))
            }
        }
    } else {
        ("V", format!("ol ({}%)", volume))
    };

    spans.push(Span::styled(first_char, Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)));
    spans.push(Span::styled(format!("{} | ", rest_text), Style::default().fg(Color::White)));

    let create_icon = |item: PlaybarItem, icon: &str, is_on: bool, default_color: Color| -> Span<'static> {
        let mut style = Style::default();
        
        if is_playbar_active && hovered_item == item {
            if is_on {
                style = style.fg(Color::LightGreen).add_modifier(Modifier::BOLD);
            } else {
                style = style.fg(Color::Cyan).add_modifier(Modifier::BOLD);
            }
        } else {
            style = style.fg(if is_on { Color::Green } else { default_color });
        }
        
        Span::styled(format!(" {} ", icon), style)
    };

    let vol_icon = if volume == 0 { "󰝟" } else { "󰕾" };
    spans.push(create_icon(PlaybarItem::Volume, vol_icon, false, Color::White));
    spans.push(create_icon(PlaybarItem::Lyrics, "󰎆", false, Color::White));
    spans.push(create_icon(PlaybarItem::Queue, "󰲹", false, Color::White));
    spans.push(create_icon(PlaybarItem::Shuffle, "󰒟", shuffle, Color::DarkGray));
    
    let (repeat_icon, repeat_on) = match repeat {
        RepeatState::Off => ("󰑖", false),
        RepeatState::Context => ("󰑖", true),
        RepeatState::Track => ("󰑘", true),
    };
    spans.push(create_icon(PlaybarItem::Repeat, repeat_icon, repeat_on, Color::DarkGray));

    let controls_line = Line::from(spans);
    f.render_widget(Paragraph::new(controls_line).alignment(Alignment::Right), chunks[2]);
}