use crate::app::{ActiveBlock, album_state::AlbumState};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Table, Row, HighlightSpacing, Paragraph, Padding},
};
use super::layout::truncate;

pub fn draw(f: &mut Frame, state: &mut AlbumState, active_block: &ActiveBlock, area: Rect) {
    let is_focused = *active_block == ActiveBlock::AlbumBlock;
    let border_color = if is_focused { Color::LightCyan } else { Color::White };

    let block = Block::default()
        .title(" Album ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color));

    let inner_area = block.inner(area);
    f.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5),
            Constraint::Min(0),
        ])
        .split(inner_area);
    
    let info_width = chunks[0].width;
    let info_max = info_width.saturating_sub(20);

    // --------------------------------------- Album Info -----------------------------
    let album_content = if let Some(album) = &state.album {
        let artists_full = album.artists.iter().map(|a| a.name.as_str()).collect::<Vec<_>>().join(", ");
        let release_date = album.release_date.as_deref().unwrap_or("Unknown Date");

        vec![
            Line::from(vec![
                Span::styled(" 💿 Album: ", Style::default().fg(Color::LightGreen).add_modifier(Modifier::BOLD)),
                Span::styled(truncate(&album.name, info_max), Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(vec![
                Span::raw("    👤 Artist: "),
                Span::styled(truncate(&artists_full, info_max), Style::default().fg(Color::White)),
            ]),
            Line::from(vec![
                Span::raw("    📅 Released: "),
                Span::styled(release_date.to_string(), Style::default().fg(Color::DarkGray)),
            ]),
        ]
    } else {
        vec![Line::from(Span::styled("   (No album data)", Style::default().fg(Color::DarkGray)))]
    };

    let album_info_widget = Paragraph::new(album_content)
        .block(Block::default().padding(Padding::new(1, 1, 1, 0)));
    f.render_widget(album_info_widget, chunks[0]);

    // --------------------------------------- Track List -----------------------------
    let table_block = Block::default()
        .title(" Tracks ")
        .borders(Borders::TOP)
        .border_style(Style::default().fg(border_color));

    let table_area = table_block.inner(chunks[1]);
    state.last_area = table_area;

    let table_width = table_area.width;
    let show_extra_column = table_width > 60;

    let header_style = Style::default().fg(Color::DarkGray).add_modifier(Modifier::BOLD);
    let highlight_style = Style::default().fg(Color::LightMagenta).add_modifier(Modifier::BOLD);

    let (header_cells, widths) = if show_extra_column {
        (
            vec![" #title", "#artist", "#length"],
            vec![Constraint::Percentage(45), Constraint::Percentage(35), Constraint::Percentage(20)]
        )
    } else {
        (
            vec![" #title", "#artist"],
            vec![Constraint::Percentage(55), Constraint::Percentage(45)]
        )
    };

    let (title_max, artist_max) = if show_extra_column {
        (
            ((table_width as f32 * 0.45) as u16).saturating_sub(15),
            ((table_width as f32 * 0.35) as u16).saturating_sub(4),
        )
    } else {
        (
            ((table_width as f32 * 0.55) as u16).saturating_sub(15),
            ((table_width as f32 * 0.45) as u16).saturating_sub(4),
        )
    };

    let header = Row::new(header_cells).style(header_style).bottom_margin(0);

    let rows: Vec<Row> = state.tracks.items.iter().enumerate().map(|(i, t)| {
        let trunc_title = truncate(&t.name, title_max);
        let artists = t.artists.iter().map(|a| a.name.as_str()).collect::<Vec<_>>().join(", ");
        let trunc_artist = truncate(&artists, artist_max);
        
        let duration_secs = t.duration.as_secs();
        let duration_str = format!("{}:{:02}", duration_secs / 60, duration_secs % 60);
        
        let icon = "󰎆";

        if show_extra_column {
            Row::new(vec![
                format!("  {} {}. {}", icon, i + 1, trunc_title),
                trunc_artist,
                duration_str
            ]).style(Style::default().fg(Color::White))
        } else {
            Row::new(vec![
                format!("  {} {}. {}", icon, i + 1, trunc_title),
                trunc_artist
            ]).style(Style::default().fg(Color::White))
        }
    }).collect();

    let table = Table::new(rows, widths)
        .header(header)
        .block(table_block)
        .highlight_symbol("▶ ")
        .highlight_spacing(HighlightSpacing::Always)
        .row_highlight_style(highlight_style);

    f.render_stateful_widget(table, chunks[1], &mut state.tracks.state);
}