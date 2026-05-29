use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Table, Row, HighlightSpacing, Paragraph, Padding},
};
use std::collections::HashMap;

use crate::app::playlist_state::PlaylistState;
use crate::app::ActiveBlock;
use crate::network::models::*;
use super::layout::truncate;

pub fn draw(
    f: &mut Frame,
    state: &mut PlaylistState,
    active_block: &ActiveBlock,
    shuffle_state: &HashMap<String, bool>,
    area: Rect
) {
    let is_focused = *active_block == ActiveBlock::PlaylistTracks;
    let border_color = if is_focused { Color::LightCyan } else { Color::White };

    // block
    let outer_block = Block::default()
        .title(format!(" [3] Playlist: {} ", state.playlist.name)) 
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color));

    let inner_area = outer_block.inner(area);

    f.render_widget(outer_block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5),
            Constraint::Min(0),
        ])
        .split(inner_area);

    let info_width = chunks[0].width;
    let info_max = info_width.saturating_sub(22);

    let owner_name = if state.playlist.owner.display_name.is_empty() {
        "Unknown"
    } else {
        state.playlist.owner.display_name.as_str()
    };
    let is_shuffle_on = shuffle_state
        .get(&state.playlist.uri)
        .copied()
        .unwrap_or(false);

    let playlist_content = vec![
        Line::from(vec![
            Span::styled(" 🎶 Playlist: ", Style::default().fg(Color::LightGreen).add_modifier(Modifier::BOLD)),
            Span::styled(truncate(&state.playlist.name, info_max), Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::raw("    👤 Owner: "),
            Span::styled(truncate(owner_name, info_max), Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::raw("    🔀 Shuffle: "),
            Span::styled(if is_shuffle_on { "On" } else { "Off" }, Style::default().fg(Color::DarkGray)),
        ]),
    ];

    let playlist_info_widget = Paragraph::new(playlist_content)
        .block(Block::default().padding(Padding::new(1, 1, 1, 0)));
    f.render_widget(playlist_info_widget, chunks[0]);

    let table_block = Block::default()
        .title(" Tracks ")
        .borders(Borders::TOP)
        .border_style(Style::default().fg(border_color));

    let table_area = table_block.inner(chunks[1]);
    state.last_area = table_area;

    let table_width = table_area.width;
    
    let title_max = ((table_width as f32 * 0.45) as u16).saturating_sub(6);
    let artist_max = ((table_width as f32 * 0.35) as u16).saturating_sub(2);

    // table
    let header_style = Style::default().fg(Color::DarkGray).add_modifier(Modifier::BOLD);
    let highlight_style = if is_focused {
        Style::default().fg(Color::LightMagenta).add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    };

    let header = Row::new(vec!["  #title", "#artist", "#length"]).style(header_style);
    let widths = [Constraint::Percentage(45), Constraint::Percentage(35), Constraint::Percentage(20)];

    // get data
    let rows: Vec<Row> = state.tracks.items.iter().map(|t| {
        let trunc_title = truncate(t.name(), title_max);
        let trunc_artist = truncate(t.artists().as_str(), artist_max);
        
        let duration_secs = match t {
            PlayableItem::Track(i) => i.duration.as_secs(),
            PlayableItem::Episode(i) => i.duration.as_secs(),
        };
        let duration_str = format!("{}:{:02}", duration_secs / 60, duration_secs % 60);

        Row::new(vec![
            format!("  {}", trunc_title),
            trunc_artist, 
            duration_str
        ])
    }).collect();

    let table = Table::new(rows, widths)
        .header(header)
        .block(table_block)
        .row_highlight_style(highlight_style)
        .highlight_symbol(if is_focused { "▶ " } else { "  " })
        .highlight_spacing(HighlightSpacing::Always);

    f.render_stateful_widget(table, chunks[1], &mut state.tracks.state);
}
