use crate::app::{App, ActiveBlock, route::Route};
use crate::app::search_state::SearchHoveredPane;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, List, ListItem},
};

pub fn draw(f: &mut Frame, app: &mut App, area: Rect) {
    let (input_text, hovered_pane) = if let Route::Search(s) = &app.route {
        (s.input.clone(), s.hovered_pane.clone())
    } else {
        (String::new(), SearchHoveredPane::Input)
    };

    let is_input_active = app.active_block == ActiveBlock::SearchInput;
    let is_results_active = app.active_block == ActiveBlock::SearchResults;

    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(area);

    // input
    let input_color = if is_input_active { Color::LightCyan } else { Color::White };
    
    let input_widget = Paragraph::new(input_text.clone()).block(
        Block::default()
            .title(" 🔍 Search ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(input_color)),
    );
    f.render_widget(input_widget, main_chunks[0]);

    if is_input_active {
        f.set_cursor_position((
            main_chunks[0].x + 1 + input_text.chars().count() as u16,
            main_chunks[0].y + 1,
        ));
    }

    // grid
    let result_rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(main_chunks[1]);

    let top_cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(result_rows[0]);

    let bottom_cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(result_rows[1]);

    // get data
    let results = if let Route::Search(ref s) = app.route {
        &s.results
    } else {
        return; 
    };

    let get_color = |pane: SearchHoveredPane| {
        if is_results_active && hovered_pane == pane {
            Color::LightCyan
        } else {
            Color::White
        }
    };

    let highlight_style = Style::default().bg(Color::DarkGray).add_modifier(ratatui::style::Modifier::BOLD);

    // tracks
    let tracks_items: Vec<ListItem> = results.tracks
        .iter()
        .flatten()
        .map(|t| {
            ListItem::new(format!(
                    "{} - {}", 
                    t.name, 
                    t.artists.iter().map(|a| a.name.as_str()).collect::<Vec<_>>().join(", ")
                    // TODO: implement to_str for base models
            ))
        })
    .collect();
    let tracks_list = List::new(tracks_items)
        .block(Block::default().title(" Tracks ").borders(Borders::ALL).border_style(Style::default().fg(get_color(SearchHoveredPane::Tracks))))
        .highlight_style(highlight_style)
        .highlight_symbol("▶ ");

    // artists
    let artists_items: Vec<ListItem> = results.artists.iter().flatten()
        .map(|a| ListItem::new(a.name.clone()))
        .collect();
    let artists_list = List::new(artists_items)
        .block(Block::default().title(" Artists ").borders(Borders::ALL).border_style(Style::default().fg(get_color(SearchHoveredPane::Artists))))
        .highlight_style(highlight_style)
        .highlight_symbol("▶ ");
    
    // albums
    let albums_items: Vec<ListItem> = results.albums.iter().flatten()
        .map(|a| ListItem::new(format!("{} ({})", a.name, a.release_date.as_deref().unwrap_or("Unknown"))))
        .collect();
    let albums_list = List::new(albums_items)
        .block(Block::default().title(" Albums ").borders(Borders::ALL).border_style(Style::default().fg(get_color(SearchHoveredPane::Albums))))
        .highlight_style(highlight_style)
        .highlight_symbol("▶ ");

    // playlists
    let playlists_items: Vec<ListItem> = results.playlists.iter().flatten()
        .map(|p| ListItem::new(format!("{} by {}", p.name, p.owner.display_name)))
        .collect();
    let playlists_list = List::new(playlists_items)
        .block(Block::default().title(" Playlists ").borders(Borders::ALL).border_style(Style::default().fg(get_color(SearchHoveredPane::Playlists))))
        .highlight_style(highlight_style)
        .highlight_symbol("▶ ");

    if let Route::Search(ref mut search_state) = app.route {
        f.render_stateful_widget(tracks_list, top_cols[0], &mut search_state.tracks_state);
        f.render_stateful_widget(artists_list, top_cols[1], &mut search_state.artists_state);
        f.render_stateful_widget(albums_list, bottom_cols[0], &mut search_state.albums_state);
        f.render_stateful_widget(playlists_list, bottom_cols[1], &mut search_state.playlists_state);
    }
}
