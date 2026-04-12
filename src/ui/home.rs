use crate::app::{ActiveBlock, home_state::{HomeTab, HomeState}};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, Borders, Tabs, Table, Row},
};

pub fn draw(f: &mut Frame, state: &mut HomeState, active_block: &ActiveBlock, area: Rect) {
    let is_home_focused = *active_block == ActiveBlock::HomeBlock;
    let border_color = if is_home_focused { Color::LightMagenta } else { Color::White };

    let outer_block = Block::default()
        .title(format!(" {} ", state.greeting))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color));

    let inner_area = outer_block.inner(area);
    f.render_widget(outer_block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Min(0),
        ])
        .split(inner_area);

    let tab_titles: Vec<Line> = vec!["[1] 🔥 Top Tracks", "[2] 🎤 Top Artists", "[3] 🕒 Recently Played"]
        .into_iter()
        .map(|t| Line::from(t))
        .collect();

    let active_tab_index = match state.active_tab {
        HomeTab::TopTracks => 0,
        HomeTab::TopArtists => 1,
        HomeTab::RecentlyPlayed => 2,
    };

    let tabs = Tabs::new(tab_titles)
        .select(active_tab_index)
        .highlight_style(Style::default().fg(Color::LightGreen).add_modifier(Modifier::BOLD))
        .divider(" | ");

    f.render_widget(tabs, chunks[0]);

    let show_extra_column = chunks[1].width > 60;

    let highlight_style = Style::default().fg(Color::LightMagenta).add_modifier(Modifier::BOLD);
    let header_style = Style::default().fg(Color::DarkGray).add_modifier(Modifier::BOLD);

    match state.active_tab {
        HomeTab::TopTracks | HomeTab::RecentlyPlayed => {
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

            let rows: Vec<Row> = if state.active_tab == HomeTab::TopTracks {
                state.top_tracks.items.iter().map(|t| {
                    if show_extra_column {
                        Row::new(vec![format!("  {}", t.title), t.artist.clone(), t.extra_info.clone()])
                    } else {
                        Row::new(vec![format!("  {}", t.title), t.artist.clone()])
                    }
                }).collect()
            } else {
                state.recent_tracks.items.iter().map(|t| {
                    if show_extra_column {
                        Row::new(vec![format!("  {}", t.title), t.artist.clone(), t.extra_info.clone()])
                    } else {
                        Row::new(vec![format!("  {}", t.title), t.artist.clone()])
                    }
                }).collect()
            };

            let table = Table::new(rows, widths)
                .header(header)
                .row_highlight_style(highlight_style)
                .highlight_symbol("▶ ");

            let state_to_use = if state.active_tab == HomeTab::TopTracks {
                &mut state.top_tracks.state
            } else {
                &mut state.recent_tracks.state
            };

            f.render_stateful_widget(table, chunks[1], state_to_use);
        }

        HomeTab::TopArtists => {
            let header = Row::new(vec!["  Artist", "Genres"]).style(header_style);
            let widths = [Constraint::Percentage(40), Constraint::Percentage(60)];

            let rows: Vec<Row> = state.top_artists.items.iter().map(|a| {
                Row::new(vec![format!("  {}", a.name), a.genres.clone()])
            }).collect();

            let table = Table::new(rows, widths)
                .header(header)
                .row_highlight_style(highlight_style)
                .highlight_symbol("▶ ");

            f.render_stateful_widget(table, chunks[1], &mut state.top_artists.state);
        }
    }
}