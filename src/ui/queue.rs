use crate::app::{ActiveBlock, queue_state::QueueState};
use crate::network::models::*;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Table, Row, HighlightSpacing, Paragraph, Padding},
};

pub fn draw(f: &mut Frame, state: &mut QueueState, active_block: &ActiveBlock, area: Rect) {
    let is_focused = *active_block == ActiveBlock::QueueBlock;
    let border_color = if is_focused { Color::LightCyan } else { Color::White };

    let block = Block::default()
        .title(" Queue ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color))
        .style(Style::default().bg(Color::Rgb(28, 28, 28)));

    let inner_area = block.inner(area);
    f.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5),
            Constraint::Min(0),
        ])
        .split(inner_area);

    // --------------------------------------- Now Playing -----------------------------
    let now_playing_content = if let Some(item) = &state.currently_playing {
        let (icon, sub_label, sub_value) = match item {
            PlayableItem::Track(t) => (
                "🎵", 
                "💿 Album: ", 
                t.album.as_ref().map(|a| a.name.as_str()).unwrap_or("Unknown Album")
            ),
            PlayableItem::Episode(e) => (
                "🎙️", 
                "📺 Show: ", 
                if e.show_name.is_empty() { "Unknown Show" } else { &e.show_name }
            ),
        };

        vec![
            Line::from(vec![
                Span::styled(format!(" {} Now Playing: ", icon), Style::default().fg(Color::LightGreen).add_modifier(Modifier::BOLD)),
                Span::styled(format!("{} - {}", item.name(), item.artists()), Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(vec![
                Span::raw(format!("    {}", sub_label)),
                Span::styled(sub_value.to_string(), Style::default().fg(Color::DarkGray)),
            ]),
        ]
    } else {
        vec![Line::from(Span::styled("   (Queue is empty)", Style::default().fg(Color::DarkGray)))]
    };

    let now_playing_widget = Paragraph::new(now_playing_content)
        .block(Block::default().padding(Padding::new(1, 1, 1, 0)));
    f.render_widget(now_playing_widget, chunks[0]);

    state.last_area = chunks[1];

    let table_width = chunks[1].width;
    let show_extra_column = table_width > 60;

    let header_style = Style::default().fg(Color::DarkGray).add_modifier(Modifier::BOLD);
    let highlight_style = Style::default().fg(Color::LightMagenta).add_modifier(Modifier::BOLD);

    let (header_cells, widths) = if show_extra_column {
        (
            vec![" # Title", "Artist / Show", "Length"],
            vec![Constraint::Percentage(45), Constraint::Percentage(35), Constraint::Percentage(20)]
        )
    } else {
        (
            vec![" # Title", "Artist / Show"],
            vec![Constraint::Percentage(55), Constraint::Percentage(45)]
        )
    };

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

    let header = Row::new(header_cells).style(header_style).bottom_margin(1);

    let rows: Vec<Row> = state.queue_items.items.iter().enumerate().map(|(i, item)| {
        let title = truncate(item.name(), title_max);
        let artist = truncate(&item.artists(), artist_max);
        
        let duration = match item {
            PlayableItem::Track(t) => t.duration,
            PlayableItem::Episode(e) => e.duration,
        };
        let duration_str = format!("{}:{:02}", duration.as_secs() / 60, duration.as_secs() % 60);
        
        let icon = match item {
            PlayableItem::Track(_) => "󰎆",
            PlayableItem::Episode(_) => "",
        };

        if show_extra_column {
            Row::new(vec![
                format!("  {} {}. {}", icon, i + 1, title),
                artist,
                duration_str,
            ]).style(Style::default().fg(Color::White))
        } else {
            Row::new(vec![
                format!("  {} {}. {}", icon, i + 1, title),
                artist,
            ]).style(Style::default().fg(Color::White))
        }
    }).collect();

    let table_block = Block::default()
        .title(" Up Next ")
        .borders(Borders::TOP)
        .border_style(Style::default().fg(border_color));

    let table = Table::new(rows, widths)
        .header(header)
        .block(table_block)
        .highlight_symbol("▶ ")
        .highlight_spacing(HighlightSpacing::Always)
        .row_highlight_style(highlight_style);

    f.render_stateful_widget(table, chunks[1], &mut state.queue_items.state);
}

fn truncate(text: &str, max_width: u16) -> String {
    let max_width = max_width as usize;
    if text.chars().count() > max_width && max_width > 3 {
        format!("{}...", text.chars().take(max_width - 3).collect::<String>())
    } else {
        text.to_string()
    }
}