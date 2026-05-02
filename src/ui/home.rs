use crate::app::{ActiveBlock, home_state::{HomeTab, HomeState}};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, Borders, Tabs, Table, Row, HighlightSpacing},
};
use super::layout::truncate;

pub fn draw(f: &mut Frame, state: &mut HomeState, active_block: &ActiveBlock, area: Rect) {
    let is_home_focused = *active_block == ActiveBlock::HomeBlock;
    let border_color = if is_home_focused { Color::LightCyan } else { Color::White };

    // draw outer block
    let outer_block = Block::default()
        .title(format!(" {} ", state.greeting))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color));

    let inner_area = outer_block.inner(area);
    f.render_widget(outer_block, area);

    // chunk[0] is for tab's name,
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Min(0),
        ])
        .split(inner_area);

    let tab_titles: Vec<Line> = vec!["🔥 Top Tracks [1]", "🎤 Top Artists [2]", "🕒 Recently Played [3]"]
        .into_iter()
        .map(Line::from)
        .collect();

    let active_tab_index = match state.active_tab {
        HomeTab::TopTracks => 0,
        HomeTab::TopArtists => 1,
        HomeTab::RecentlyPlayed => 2,
    };

    // draw a line show 3 tab name
    let tabs = Tabs::new(tab_titles)
        .select(active_tab_index)
        .highlight_style(Style::default().fg(Color::LightGreen).add_modifier(Modifier::BOLD))
        .divider(" | ");

    f.render_widget(tabs, chunks[0]);
    
    // check whether if the width of terminal is > 60
    let table_width = chunks[1].width;
    let show_extra_column = table_width > 60;

    let highlight_style = Style::default().fg(Color::LightMagenta).add_modifier(Modifier::BOLD);
    let header_style = Style::default().fg(Color::DarkGray).add_modifier(Modifier::BOLD);

    state.last_area = chunks[1];
    match state.active_tab {
        HomeTab::TopTracks | HomeTab::RecentlyPlayed => {
            // if width > 60, we show 3 columns
            let (header_cells, widths) = if show_extra_column {
                let cells = vec!["  #title", "#artist", "#length"];
                let w = [Constraint::Percentage(45), Constraint::Percentage(35), Constraint::Percentage(20)];
                (cells, w.to_vec())
            } else {
                let cells = vec!["  #title", "#artist"];
                let w = [Constraint::Percentage(55), Constraint::Percentage(45)];
                (cells, w.to_vec())
            };

            let header = Row::new(header_cells).style(header_style);

            let (title_max, artist_max) = if show_extra_column {
                (
                    ((table_width as f32 * 0.45) as u16).saturating_sub(6),
                    ((table_width as f32 * 0.35) as u16).saturating_sub(2),
                )
            } else {
                (
                    ((table_width as f32 * 0.55) as u16).saturating_sub(6),
                    ((table_width as f32 * 0.45) as u16).saturating_sub(2),
                )
            };

            // determine top tracks or recent tracks and get data
            let target_table = if state.active_tab == HomeTab::TopTracks {
                &state.top_tracks
            } else {
                &state.recent_tracks
            };

            let rows: Vec<Row> = target_table.list.items.iter().map(|t| {
                let trunc_title = truncate(&t.name, title_max);
                
                let artist = t.artists.first()
                    .map(|a| a.name.clone())
                    .unwrap_or_else(|| "Unknown".to_string());

                let trunc_artist = truncate(&artist, artist_max);
                
                let duration_secs = t.duration.as_secs();
                let duration_str = format!("{}:{:02}", duration_secs / 60, duration_secs % 60);

                if show_extra_column {
                    Row::new(vec![format!("  {}", trunc_title), trunc_artist, duration_str])
                } else {
                    Row::new(vec![format!("  {}", trunc_title), trunc_artist])
                }
            }).collect();

            let table = Table::new(rows, widths)
                .header(header)
                .row_highlight_style(highlight_style)
                .highlight_symbol("▶ ")
                .highlight_spacing(HighlightSpacing::Always);

            let state_to_use = if state.active_tab == HomeTab::TopTracks {
                &mut state.top_tracks.list.state
            } else {
                &mut state.recent_tracks.list.state
            };

            f.render_stateful_widget(table, chunks[1], state_to_use);
        }

        HomeTab::TopArtists => {
            let header = Row::new(vec!["  #artist"]).style(header_style);
            let widths = [Constraint::Percentage(40), Constraint::Percentage(60)];
            let artist_max = ((table_width as f32 * 0.40) as u16).saturating_sub(6);

            let rows: Vec<Row> = state.top_artists.list.items.iter().map(|a| {
                let trunc_name = truncate(&a.name, artist_max);
                Row::new(vec![format!("  {}", trunc_name)])
            }).collect();

            let table = Table::new(rows, widths)
                .header(header)
                .row_highlight_style(highlight_style)
                .highlight_symbol("▶ ")
                .highlight_spacing(HighlightSpacing::Always);

            f.render_stateful_widget(table, chunks[1], &mut state.top_artists.list.state);
        }
    }
}
