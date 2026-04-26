use crate::app::ActiveBlock;
use crate::app::library_state::*;
use ratatui::{
    Frame,
    layout::{Constraint, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Table, Row, HighlightSpacing},
};

pub fn draw_liked_songs(f: &mut Frame, state: &mut LikedSongsState, active_block: &ActiveBlock, area: Rect) {
    let is_focused = *active_block == ActiveBlock::LikedSongs;
    let border_color = if is_focused { Color::LightCyan } else { Color::White };

    let outer_block = Block::default()
        .title(" Liked Songs ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color));

    let inner_area = outer_block.inner(area);
    state.last_area = inner_area;

    f.render_widget(outer_block, area);

    let table_width = inner_area.width;
    let title_max = ((table_width as f32 * 0.45) as u16).saturating_sub(6);
    let artist_max = ((table_width as f32 * 0.35) as u16).saturating_sub(2);

    let header_style = Style::default().fg(Color::DarkGray).add_modifier(Modifier::BOLD);
    let highlight_style = Style::default().fg(Color::LightMagenta).add_modifier(Modifier::BOLD);

    let header = Row::new(vec!["  #title", "#artist", "#length"]).style(header_style);
    let widths = [Constraint::Percentage(45), Constraint::Percentage(35), Constraint::Percentage(20)];

    let rows: Vec<Row> = state.tracks.items.iter().map(|t| {
        let trunc_title = truncate(&t.name, title_max);
        let artist_names = t.artists.iter()
            .map(|a| a.name.as_str())
            .collect::<Vec<_>>()
            .join(", ");
        let trunc_artist = truncate(&artist_names, artist_max);
        
        let duration_secs = t.duration.as_secs();
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

pub fn draw_saved_albums(f: &mut Frame, state: &mut SavedAlbumsState, active_block: &ActiveBlock, area: Rect) {
    let is_focused = *active_block == ActiveBlock::SavedAlbums;
    let border_color = if is_focused { Color::LightCyan } else { Color::White };

    let outer_block = Block::default()
        .title(" Saved Albums ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color));

    let inner_area = outer_block.inner(area);
    state.last_area = inner_area;

    f.render_widget(outer_block, area);

    let table_width = inner_area.width;
    let album_max = ((table_width as f32 * 0.45) as u16).saturating_sub(6);
    let artist_max = ((table_width as f32 * 0.35) as u16).saturating_sub(2);

    let header_style = Style::default().fg(Color::DarkGray).add_modifier(Modifier::BOLD);
    let highlight_style = Style::default().fg(Color::LightMagenta).add_modifier(Modifier::BOLD);

    // Columns: Album Title, Artists, Release Date
    let header = Row::new(vec!["  #album", "#artist", "#release"]).style(header_style);
    let widths = [Constraint::Percentage(45), Constraint::Percentage(35), Constraint::Percentage(20)];

    let rows: Vec<Row> = state.albums.items.iter().map(|a| {
        let trunc_album = truncate(&a.name, album_max);
        let artist_names = a.artists.iter()
            .map(|ar| ar.name.as_str())
            .collect::<Vec<_>>()
            .join(", ");
        let trunc_artist = truncate(&artist_names, artist_max);
        
        let release_date = a.release_date.as_deref().unwrap_or("Unknown");

        Row::new(vec![
            format!("  {}", trunc_album),
            trunc_artist, 
            release_date.to_string()
        ])
    }).collect();

    let table = Table::new(rows, widths)
        .header(header)
        .row_highlight_style(highlight_style)
        .highlight_symbol("▶ ")
        .highlight_spacing(HighlightSpacing::Always);

    f.render_stateful_widget(table, inner_area, &mut state.albums.state);
}

pub fn draw_saved_artists(f: &mut Frame, state: &mut SavedArtistsState, active_block: &ActiveBlock, area: Rect) {
    let is_focused = *active_block == ActiveBlock::SavedArtists;
    let border_color = if is_focused { Color::LightCyan } else { Color::White };

    let outer_block = Block::default()
        .title(" Saved Artists ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color));

    let inner_area = outer_block.inner(area);
    state.last_area = inner_area;

    f.render_widget(outer_block, area);

    let table_width = inner_area.width;
    let name_max = ((table_width as f32 * 0.45) as u16).saturating_sub(6);
    let genre_max = ((table_width as f32 * 0.55) as u16).saturating_sub(2);

    let header_style = Style::default().fg(Color::DarkGray).add_modifier(Modifier::BOLD);
    let highlight_style = Style::default().fg(Color::LightMagenta).add_modifier(Modifier::BOLD);

    let header = Row::new(vec!["  #artist", "#genres"]).style(header_style);
    let widths = [Constraint::Percentage(45), Constraint::Percentage(55)];

    let rows: Vec<Row> = state.artists.items.iter().map(|a| {
        let trunc_name = truncate(&a.name, name_max);
        let genres_str = a.genres.as_ref()
            .map(|g| g.join(", "))
            .unwrap_or_else(|| "None".to_string());
        let trunc_genres = truncate(&genres_str, genre_max);

        Row::new(vec![
            format!("  {}", trunc_name),
            trunc_genres,
        ])
    }).collect();

    let table = Table::new(rows, widths)
        .header(header)
        .row_highlight_style(highlight_style)
        .highlight_symbol("▶ ")
        .highlight_spacing(HighlightSpacing::Always);

    f.render_stateful_widget(table, inner_area, &mut state.artists.state);
}

pub fn draw_saved_podcasts(f: &mut Frame, state: &mut SavedPodcastsState, active_block: &ActiveBlock, area: Rect) {
    let is_focused = *active_block == ActiveBlock::SavedPodcasts;
    let border_color = if is_focused { Color::LightCyan } else { Color::White };

    let outer_block = Block::default()
        .title(" Saved Podcasts ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color));

    let inner_area = outer_block.inner(area);
    state.last_area = inner_area;

    f.render_widget(outer_block, area);

    let table_width = inner_area.width;
    let name_max = ((table_width as f32 * 0.40) as u16).saturating_sub(6);
    let show_max = ((table_width as f32 * 0.30) as u16).saturating_sub(2);

    let header_style = Style::default().fg(Color::DarkGray).add_modifier(Modifier::BOLD);
    let highlight_style = Style::default().fg(Color::LightMagenta).add_modifier(Modifier::BOLD);

    let header = Row::new(vec!["  #name", "#show", "#length", "#release"]).style(header_style);
    let widths = [
        Constraint::Percentage(40),
        Constraint::Percentage(30),
        Constraint::Percentage(15),
        Constraint::Percentage(15),
    ];

    let rows: Vec<Row> = state.podcasts.items.iter().map(|e| {
        let trunc_name = truncate(&e.name, name_max);
        let trunc_show = truncate(&e.show_name, show_max);

        let duration_secs = e.duration.as_secs();
        let duration_str = format!("{}:{:02}", duration_secs / 60, duration_secs % 60);

        Row::new(vec![
            format!("  {}", trunc_name),
            trunc_show,
            duration_str,
            e.release_date.clone(),
        ])
    }).collect();

    let table = Table::new(rows, widths)
        .header(header)
        .row_highlight_style(highlight_style)
        .highlight_symbol("▶ ")
        .highlight_spacing(HighlightSpacing::Always);

    f.render_stateful_widget(table, inner_area, &mut state.podcasts.state);
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