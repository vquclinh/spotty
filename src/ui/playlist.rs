use crate::app::playlist_state::PlaylistState;
use crate::app::ActiveBlock;
use ratatui::{
    Frame,
    layout::{Constraint, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Table, Row, HighlightSpacing},
};
use crate::network::models::*;

pub fn draw(f: &mut Frame, state: &mut PlaylistState, active_block: &ActiveBlock, area: Rect) {
    let is_focused = *active_block == ActiveBlock::PlaylistTracks;
    let border_color = if is_focused { Color::LightCyan } else { Color::White };

    // block
    let outer_block = Block::default()
        .title(format!(" Playlists: {} ", state.playlist.name)) 
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color));

    let inner_area = outer_block.inner(area);
    f.render_widget(outer_block, area);

    let table_width = inner_area.width;
    
    let title_max = ((table_width as f32 * 0.45) as u16).saturating_sub(6);
    let artist_max = ((table_width as f32 * 0.35) as u16).saturating_sub(2);

    // table
    let header_style = Style::default().fg(Color::DarkGray).add_modifier(Modifier::BOLD);
    let highlight_style = Style::default().fg(Color::LightMagenta).add_modifier(Modifier::BOLD);

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
        .row_highlight_style(highlight_style)
        .highlight_symbol("▶ ")
        .highlight_spacing(HighlightSpacing::Always);

    f.render_stateful_widget(table, inner_area, &mut state.tracks.state);
}

fn truncate(text: &str, max_width: u16) -> String {
    let max_width = max_width as usize;
    let char_count = text.chars().count();
    
    if char_count > max_width {
        if max_width <= 3 {
            return text.chars().take(max_width).collect();
        }
        let truncated: String = text.chars().take(max_width - 3).collect();
        format!("{}...", truncated)
    } else {
        text.to_string()
    }
}
