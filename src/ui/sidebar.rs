use crate::app::{ActiveBlock, App};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Table, Row},
};

pub fn draw(f: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(6), Constraint::Min(0)])
        .split(area);

    draw_library(f, app, chunks[0]);
    draw_playlists(f, app, chunks[1]);
}

// library
fn draw_library(f: &mut Frame, app: &mut App, area: Rect) {
    let border_color = if app.active_block == ActiveBlock::LibraryMenu { Color::LightCyan } else { Color::White };

    let items = vec![
        ListItem::new(" ♥ Liked Songs"),
        ListItem::new(" 👤 Artists"),
        ListItem::new(" 💿 Albums"),
        ListItem::new(" 🎙 Podcasts"),
    ];

    let list = List::new(items)
        .block(
            Block::default()
                .title(" Library ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(border_color)),
        )
        .highlight_style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        );

    f.render_stateful_widget(list, area, &mut app.library_menu.state);
}

// playlists
fn draw_playlists(f: &mut Frame, app: &mut App, area: Rect) {
    let border_color = if app.active_block == ActiveBlock::PlaylistsMenu { Color::LightCyan } else { Color::White };
    let mut rows: Vec<Row> = vec![];

    for playlist in &app.playlists_menu.items {
        rows.push(Row::new(vec![format!(" ♪ {}", playlist.name)]));
    }

    if rows.is_empty() {
        rows.push(Row::new(vec![" ⏳ Loading...".to_string()]));
    }

    let table = Table::new(rows, [Constraint::Percentage(100)])
        .block(
            Block::default()
                .title(" Playlists ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(border_color)),
        )
        .row_highlight_style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        );

    f.render_stateful_widget(table, area, &mut app.playlists_menu.state);
}
