use crate::app::{App, ActiveBlock, route::Route};
use crate::app::search_state::SearchHoveredPane;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, BorderType, Paragraph, List, ListItem, Padding, HighlightSpacing},
};
use super::layout::truncate;

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
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(input_color))
            .padding(Padding::horizontal(1)),
    );
    f.render_widget(input_widget, main_chunks[0]);

    if is_input_active {
        f.set_cursor_position((
            main_chunks[0].x + 2 + input_text.chars().count() as u16,
            main_chunks[0].y + 1,
        ));
    }

    // grid
    let results_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Min(0)])
        .split(main_chunks[1]);

    let mut right_area = results_chunks[1];
    right_area.x = right_area.x.saturating_sub(1);
    right_area.width += 1;

    let mut queue_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Ratio(1, 3), 
            Constraint::Ratio(1, 3), 
            Constraint::Min(0),
        ])
        .split(right_area)
        .to_vec();

    queue_chunks[1].y = queue_chunks[1].y.saturating_sub(1);
    queue_chunks[1].height += 1;

    queue_chunks[2].y = queue_chunks[2].y.saturating_sub(1);
    queue_chunks[2].height += 1;

    let (tracks_area, artists_area, albums_area, playlists_area) = match hovered_pane {
        SearchHoveredPane::Artists => (
            queue_chunks[2],   
            results_chunks[0], 
            queue_chunks[0],   
            queue_chunks[1],   
        ),
        SearchHoveredPane::Albums => (
            queue_chunks[1],   
            queue_chunks[2],   
            results_chunks[0], 
            queue_chunks[0],   
        ),
        SearchHoveredPane::Playlists => (
            queue_chunks[0],   
            queue_chunks[1],   
            queue_chunks[2],   
            results_chunks[0], 
        ),
        _ => ( 
            results_chunks[0], 
            queue_chunks[0],   
            queue_chunks[1],   
            queue_chunks[2],   
        ),
    };

    // get data
    let search_state = if let Route::Search(ref s) = app.route {
        s
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

    let get_highlight_style = |pane: SearchHoveredPane| {
        if is_results_active && hovered_pane == pane {
            Style::default().fg(Color::LightMagenta).add_modifier(Modifier::BOLD)
        } else {
            Style::default()
        }
    };

    let get_highlight_symbol = |pane: SearchHoveredPane| {
        if is_results_active && hovered_pane == pane {
            "▶ "
        } else {
            "  "
        }
    };

    let build_block = |title: &str, pane: SearchHoveredPane| {
        Block::default()
            .title(format!(" {} ", title))
            .borders(Borders::ALL)
            .border_type(BorderType::Plain) 
            .border_style(Style::default().fg(get_color(pane)))
    };

    // tracks
    let track_max_width = tracks_area.width.saturating_sub(6);
    let tracks_items: Vec<ListItem> = search_state
        .tracks_state
        .list.items
        .iter()
        .map(|t| {
            let artist_names = t.artists.iter()
                .map(|a| a.name.as_str())
                .collect::<Vec<_>>()
                .join(", ");

            let full_text = format!("{} - {}", t.name, artist_names);
            ListItem::new(format!("  {}", truncate(&full_text, track_max_width)))
        })
    .collect();

    let tracks_list = List::new(tracks_items)
        .block(build_block("Tracks", SearchHoveredPane::Tracks))
        .highlight_style(get_highlight_style(SearchHoveredPane::Tracks))
        .highlight_symbol(get_highlight_symbol(SearchHoveredPane::Tracks))
        .highlight_spacing(HighlightSpacing::Always);

    // artists
    let artist_max_width = artists_area.width.saturating_sub(6);
    let artists_items: Vec<ListItem> = search_state
        .artists_state
        .list.items
        .iter()
        .map(|a| ListItem::new(format!("  {}", truncate(&a.name, artist_max_width))))
        .collect();

    let artists_list = List::new(artists_items)
        .block(build_block("Artists", SearchHoveredPane::Artists))
        .highlight_style(get_highlight_style(SearchHoveredPane::Artists))
        .highlight_symbol(get_highlight_symbol(SearchHoveredPane::Artists))
        .highlight_spacing(HighlightSpacing::Always);

    // albums
    let album_max_width = albums_area.width.saturating_sub(6);
    let albums_items: Vec<ListItem> = search_state
        .albums_state
        .list.items
        .iter()
        .map(|a| {
            let date = a.release_date.as_deref().unwrap_or("Unknown");
            let full_text = format!("{} ({})", a.name, date);
            ListItem::new(format!("  {}", truncate(&full_text, album_max_width)))
        })
    .collect();

    let albums_list = List::new(albums_items)
        .block(build_block("Albums", SearchHoveredPane::Albums))
        .highlight_style(get_highlight_style(SearchHoveredPane::Albums))
        .highlight_symbol(get_highlight_symbol(SearchHoveredPane::Albums))
        .highlight_spacing(HighlightSpacing::Always);

    // playlists
    let playlist_max_width = playlists_area.width.saturating_sub(6);
    let playlists_items: Vec<ListItem> = search_state
        .playlists_state
        .list.items
        .iter()
        .map(|p| {
            let full_text = format!("{} by {}", p.name, p.owner.display_name);
            ListItem::new(format!("  {}", truncate(&full_text, playlist_max_width)))
        })
    .collect();

    let playlists_list = List::new(playlists_items)
        .block(build_block("Playlists", SearchHoveredPane::Playlists))
        .highlight_style(get_highlight_style(SearchHoveredPane::Playlists))
        .highlight_symbol(get_highlight_symbol(SearchHoveredPane::Playlists))
        .highlight_spacing(HighlightSpacing::Always);

    if let Route::Search(ref mut search_state) = app.route {
        // store last area
        search_state.last_area = match search_state.hovered_pane {
            SearchHoveredPane::Tracks => tracks_area,
            SearchHoveredPane::Artists => artists_area,
            SearchHoveredPane::Albums => albums_area,
            SearchHoveredPane::Playlists => playlists_area,
            _ => main_chunks[1],
        };

        let mut lists = (Some(tracks_list), Some(artists_list), Some(albums_list), Some(playlists_list));

        if tracks_area != results_chunks[0] { f.render_stateful_widget(lists.0.take().unwrap(), tracks_area, &mut search_state.tracks_state.list.state); }
        if artists_area != results_chunks[0] { f.render_stateful_widget(lists.1.take().unwrap(), artists_area, &mut search_state.artists_state.list.state); }
        if albums_area != results_chunks[0] { f.render_stateful_widget(lists.2.take().unwrap(), albums_area, &mut search_state.albums_state.list.state); }
        if playlists_area != results_chunks[0] { f.render_stateful_widget(lists.3.take().unwrap(), playlists_area, &mut search_state.playlists_state.list.state); }
        
        if let Some(w) = lists.0.take() { f.render_stateful_widget(w, tracks_area, &mut search_state.tracks_state.list.state); }
        if let Some(w) = lists.1.take() { f.render_stateful_widget(w, artists_area, &mut search_state.artists_state.list.state); }
        if let Some(w) = lists.2.take() { f.render_stateful_widget(w, albums_area, &mut search_state.albums_state.list.state); }
        if let Some(w) = lists.3.take() { f.render_stateful_widget(w, playlists_area, &mut search_state.playlists_state.list.state); }
    }
}
