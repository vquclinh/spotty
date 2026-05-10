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

    let block = Block::default()
        .title(" EQUALIZER ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color));

    let inner_area = block.inner(area);
    f.render_widget(block, area);

    // split: top for track info, bottom for equalizer
    let chunks = ratatui::layout::Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([
            ratatui::layout::Constraint::Length(7),
            ratatui::layout::Constraint::Min(3),
        ])
        .split(inner_area);

    // track info
    let mut info_lines = vec![TuiLine::from("")];

    if let Some(playback) = &app.playback {
        if let Some(PlayableItem::Track(track)) = &playback.item {
            let artist_name = track.artists.first()
                .map(|a| a.name.clone())
                .unwrap_or_else(|| "Unknown".to_string());
            let album_name = track.album.as_ref()
                .map(|a| a.name.as_str())
                .unwrap_or("Unknown");
            let duration_secs = track.duration.as_secs();
            let duration_str = format!("{}:{:02}", duration_secs / 60, duration_secs % 60);

            let make_line = |key: &str, val: &str| -> TuiLine {
                TuiLine::from(vec![
                    Span::styled(format!("  {:<8}: ", key), Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
                    Span::styled(val.to_string(), Style::default().fg(Color::White)),
                ])
            };

            info_lines.push(make_line("Song", &track.name));
            info_lines.push(make_line("Artist", &artist_name));
            info_lines.push(make_line("Album", album_name));
            info_lines.push(make_line("Length", &duration_str));
        } else {
            info_lines.push(TuiLine::from(Span::styled("  No track info available.", Style::default().fg(Color::DarkGray))));
        }
    } else {
        info_lines.push(TuiLine::from(Span::styled("  No song currently playing.", Style::default().fg(Color::DarkGray))));
    }

    f.render_widget(Paragraph::new(info_lines), chunks[0]);

    // equalizer area
    let eq_area = chunks[1];
    let w = eq_area.width as usize;
    let h = eq_area.height as usize;
    if h < 3 || w < 4 {
        return;
    }

    let tick = if let Route::Lyrics(state) = &app.route {
        state.tick
    } else {
        0
    };

    // bass intensity from playback progress
    let is_playing = app.playback.as_ref().map_or(false, |pb| pb.is_playing);
    let progress_ratio = app.playback.as_ref().map_or(0.0, |pb| {
        if let Some(PlayableItem::Track(t)) = &pb.item {
            let dur = t.duration.as_secs_f64();
            if dur > 0.0 { pb.progress.as_secs_f64() / dur } else { 0.0 }
        } else {
            0.0
        }
    });

    // bass multiplier: when playing, bars bounce actively; when paused, they decay
    let bass = if is_playing {
        0.6 + progress_ratio.sin().abs() * 0.4
    } else {
        0.15
    };

    // split area: top portion for bars, bottom for reflection
    let reflection_h = (h / 5).max(1).min(4);
    let bars_h = h.saturating_sub(reflection_h + 1);
    if bars_h < 2 {
        return;
    }

    let mut grid: Vec<Vec<(char, Color)>> = vec![vec![(' ', Color::Reset); w]; h];

    // bar layout: each bar is 2 chars wide with 1 char gap (like the reference image)
    let bar_width = 2;
    let gap = 1;
    let stride = bar_width + gap;
    let num_bars = (w + gap) / stride;

    for bar_idx in 0..num_bars {
        let f1 = ((bar_idx % 5) as f64 + 1.0) * 0.22;
        let f2 = ((bar_idx % 3) as f64 + 1.0) * 0.37;
        let phase1 = bar_idx as f64 * 0.6;
        let phase2 = bar_idx as f64 * 1.1 + 2.0;
        let t = tick as f64 * 0.12;

        let wave = ((t + phase1) * f1).sin() * 0.35
                 + ((t + phase2) * f2).sin() * 0.25
                 + ((t * 0.07 + bar_idx as f64 * 0.3).sin()) * 0.2
                 + 0.3;
        let bar_height = (wave.clamp(0.0, 1.0) * bass * bars_h as f64).round() as usize;
        let bar_h = bar_height.min(bars_h);

        let x_start = bar_idx * stride;

        // draw main bars
        for row in 0..bar_h {
            let y = bars_h - 1 - row;
            let ratio = row as f64 / bars_h as f64;

            let color = if row == bar_h - 1 && bar_h > 1 {
                Color::White
            } else if ratio > 0.7 {
                Color::Rgb(40, 80, 120)    // dark blue-teal (top)
            } else if ratio > 0.5 {
                Color::Rgb(0, 130, 160)    // mid teal
            } else if ratio > 0.3 {
                Color::Rgb(0, 180, 200)    // teal-cyan
            } else {
                Color::Rgb(0, 230, 240)    // bright cyan (bottom)
            };

            for dx in 0..bar_width {
                let x = x_start + dx;
                if x < w && y < bars_h {
                    grid[y][x] = ('▇', color);
                }
            }
        }

        // draw reflection
        let reflection_start = bars_h + 1;
        let reflect_bars = bar_h.min(reflection_h);
        for row in 0..reflect_bars {
            let y = reflection_start + row;
            if y >= h { break; }

            let fade_ratio = 1.0 - (row as f64 / reflection_h as f64);
            let alpha = (fade_ratio * 60.0) as u8;

            let color = Color::Rgb(0, alpha.min(80), alpha.min(90));

            for dx in 0..bar_width {
                let x = x_start + dx;
                if x < w {
                    grid[y][x] = ('▇', color);
                }
            }
        }
    }

    // render
    let mut lines: Vec<TuiLine> = Vec::with_capacity(h);
    for row in &grid {
        let spans: Vec<Span> = row.iter().map(|&(ch, color)| {
            if color == Color::Reset {
                Span::raw(ch.to_string())
            } else {
                Span::styled(ch.to_string(), Style::default().fg(color))
            }
        }).collect();
        lines.push(TuiLine::from(spans));
    }

    f.render_widget(Paragraph::new(lines), eq_area);
}