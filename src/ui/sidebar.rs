use crate::app::{ActiveBlock, App, Route};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Row, Table},
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
    let is_focused = app.active_block == ActiveBlock::LibraryMenu;
    let border_color = if is_focused { Color::LightCyan } else { Color::White };
    
    let selected_idx = match &app.route {
        Route::LikedSongs(_) => Some(0),
        Route::SavedArtists(_) => Some(1),
        Route::SavedAlbums(_) => Some(2),
        Route::SavedPodcasts(_) => Some(3),
        _ => None,
    };
    let items = vec![
        ListItem::new(" ♥ Liked Songs").style(create_style(
            (selected_idx == Some(0)).then_some(Color::LightCyan)
        )),
        ListItem::new(" 👤 Artists").style(create_style(
            (selected_idx == Some(1)).then_some(Color::LightCyan)
        )),
        ListItem::new(" 💿 Albums").style(create_style(
            (selected_idx == Some(2)).then_some(Color::LightCyan)
        )),
        ListItem::new(" 🎙 Podcasts").style(create_style(
            (selected_idx == Some(3)).then_some(Color::LightCyan)
        )),
    ];

    let list = List::new(items)
        .block(
            Block::default()
                .title(" [1] Library ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(border_color)),
        )
        .highlight_style(create_style(is_focused.then_some(Color::Cyan)));

    f.render_stateful_widget(list, area, &mut app.library_menu.state);
}

// playlists
fn draw_playlists(f: &mut Frame, app: &mut App, area: Rect) {
    let is_focused = app.active_block == ActiveBlock::PlaylistsMenu;
    let border_color = if is_focused { Color::LightCyan } else { Color::White };
    let mut rows: Vec<Row> = vec![];

    for playlist in &app.playlists_menu.items {
        let should_highlight = if let Route::PlaylistDetail(s) = &app.route {
            s.playlist.id == playlist.id
        } else {
            false
        };
        rows.push(
            Row::new(vec![format!(" ♪ {}", playlist.name)]).style(create_style(
                should_highlight.then_some(Color::LightCyan)
            ))
        );
    }

    if rows.is_empty() {
        rows.push(Row::new(vec![" ⏳ Loading...".to_string()]));
    }

    let table = Table::new(rows, [Constraint::Percentage(100)])
        .block(
            Block::default()
                .title(" [2] Playlists ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(border_color)),
        )
        .row_highlight_style(create_style(is_focused.then_some(Color::Cyan)));

    f.render_stateful_widget(table, area, &mut app.playlists_menu.state);
}

fn create_style(color: Option<Color>) -> Style {
    if let Some(col) = color {
        Style::default()
            .fg(col)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    }
}
